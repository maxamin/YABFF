//! Fingerprinter evaluation: a labelled dataset -> confusion matrix + a margin
//! sweep for the confidence gate. Doubles as a regression guard (asserts a floor
//! on clean-sample accuracy) and generates the numbers quoted in
//! `docs/analysis/fingerprint-enhancements.md`.
//!
//! Run with output:  cargo test -p feroxbuster --test confusion -- --nocapture

use std::collections::HashMap;

use feroxbuster::ml::{self, ProbeResp, PROFILES};

const BASE: &str = "https://t.test";

#[derive(serde::Deserialize)]
struct RawResp {
    url: String,
    status: u16,
    #[serde(default)]
    headers: HashMap<String, String>,
    #[serde(default)]
    content_length: u64,
    #[serde(default)]
    word_count: u64,
    #[serde(default)]
    line_count: u64,
}

#[derive(serde::Deserialize)]
struct LabFull {
    probes: Vec<RawResp>,
}

/// Real probe captures from the live jsintel labs, with best-judgment true labels
/// for the actual stack. `(lab, true_label)`. Uses the *full* fixture (response
/// sizes + security headers) so the richer features (heavy_html, sec_headers) have
/// real values. Juice Shop / WebGoat / Django remain honest hard cases (catch-all,
/// 404s, or root-only).
fn lab_dataset() -> Vec<(&'static str, &'static str, Vec<ProbeResp>)> {
    let raw: HashMap<String, LabFull> =
        serde_json::from_str(include_str!("fixtures/jsintel_labs_full.json")).unwrap();
    let truth = [
        ("juice-shop", "REST_API"),
        ("dvwa", "PHP_GENERIC"),
        ("webgoat", "ENTERPRISE_JAVA_SPRING"),
        ("wordpress", "WORDPRESS_CMS"),
        ("django", "DJANGO"),
    ];
    truth
        .iter()
        .map(|(lab, label)| {
            let probes = raw[*lab]
                .probes
                .iter()
                .map(|r| ProbeResp {
                    url: r.url.clone(),
                    status: r.status,
                    headers: r.headers.clone(),
                    content_length: r.content_length,
                    word_count: r.word_count,
                    line_count: r.line_count,
                })
                .collect();
            (*lab, *label, probes)
        })
        .collect()
}

/// Build a probe sample from `(path, status, &[(header, value)])` triples.
fn sample(items: &[(&str, u16, &[(&str, &str)])]) -> Vec<ProbeResp> {
    items
        .iter()
        .map(|(path, status, hdrs)| {
            let mut r = ProbeResp::new(&format!("{BASE}/{path}"), *status);
            for (k, v) in *hdrs {
                r.headers.insert((*k).to_string(), (*v).to_string());
            }
            r
        })
        .collect()
}

/// (true_label, probe_sample). Clean samples represent each profile faithfully;
/// the `# noisy` ones are degraded/real-world cases (catch-all servers, thin
/// signal) that a path-presence fingerprinter is expected to find hard.
fn labelled_dataset() -> Vec<(&'static str, Vec<ProbeResp>)> {
    let json = &[("content-type", "application/json")][..];
    let wpcookie = &[("set-cookie", "wordpress_test_cookie=1; path=/")][..];
    let php = &[("x-powered-by", "PHP/8.2")][..];
    let phpcookie = &[("set-cookie", "PHPSESSID=abc; path=/"), ("x-powered-by", "PHP/8.2")][..];
    let java = &[("set-cookie", "JSESSIONID=x"), ("server", "Apache-Coyote/1.1")][..];
    let csrf = &[("set-cookie", "csrftoken=xyz; Path=/")][..];

    vec![
        // ---- REST_API (clean) ----
        ("REST_API", sample(&[("", 200, json), ("api", 200, &[]), ("api/v1", 200, &[]), ("swagger-ui.html", 200, &[]), ("openapi.json", 200, &[])])),
        ("REST_API", sample(&[("", 200, json), ("api", 500, &[]), ("rest", 500, &[]), ("graphql", 200, &[])])),
        ("REST_API", sample(&[("api", 200, &[]), ("api/v2", 200, &[]), ("v2/api-docs", 200, &[]), ("openapi.json", 200, &[])])),
        // ---- ENTERPRISE_JAVA_SPRING (clean) ----
        ("ENTERPRISE_JAVA_SPRING", sample(&[("", 200, java), ("actuator", 200, &[]), ("actuator/health", 200, &[]), ("index.jsp", 200, &[])])),
        ("ENTERPRISE_JAVA_SPRING", sample(&[("actuator", 200, &[]), ("actuator/health", 200, &[]), ("api", 200, &[]), ("", 200, java)])),
        ("ENTERPRISE_JAVA_SPRING", sample(&[("", 200, &[("server", "Jetty")]), ("actuator", 200, &[]), ("index.jsp", 200, &[])])),
        // ---- WORDPRESS_CMS (clean) ----
        ("WORDPRESS_CMS", sample(&[("wp-json", 200, json), ("wp-login.php", 200, wpcookie), ("xmlrpc.php", 405, &[]), ("index.php", 200, php)])),
        ("WORDPRESS_CMS", sample(&[("wp-json", 200, &[]), ("wp-login.php", 200, wpcookie), ("index.php", 200, php)])),
        ("WORDPRESS_CMS", sample(&[("wp-login.php", 200, wpcookie), ("xmlrpc.php", 405, php)])),
        // ---- LEGACY_STATIC (clean) ----
        ("LEGACY_STATIC", sample(&[("robots.txt", 200, &[]), ("api", 404, &[]), ("wp-json", 404, &[])])),
        ("LEGACY_STATIC", sample(&[("", 200, &[("content-type", "text/html")]), ("index.php", 404, &[]), ("actuator", 404, &[])])),
        ("LEGACY_STATIC", sample(&[("api", 404, &[]), ("rest", 404, &[]), ("wp-json", 404, &[]), ("actuator", 404, &[])])),
        // ---- PHP_GENERIC (clean): PHP signals, no WordPress markers ----
        ("PHP_GENERIC", sample(&[("index.php", 200, phpcookie), ("", 200, php), ("wp-json", 404, &[]), ("actuator", 404, &[])])),
        ("PHP_GENERIC", sample(&[("index.php", 200, php), ("server-status", 200, php), ("wp-login.php", 404, &[])])),
        ("PHP_GENERIC", sample(&[("", 200, phpcookie), ("index.php", 200, &[])])),
        // ---- NODE_SPA (clean): SPA build output + a REST-ish backend ----
        ("NODE_SPA", sample(&[("manifest.webmanifest", 200, &[]), ("assets", 200, &[]), ("api", 200, &[])])),
        ("NODE_SPA", sample(&[("_next", 200, &[]), ("assets", 200, &[]), ("", 200, &[("content-type", "text/html")])])),
        ("NODE_SPA", sample(&[("manifest.webmanifest", 200, &[]), ("_next", 200, &[])])),
        // ---- DJANGO (clean): csrftoken cookie + static/admin ----
        ("DJANGO", sample(&[("", 200, csrf), ("static/admin", 200, &[]), ("api", 200, &[])])),
        ("DJANGO", sample(&[("admin/login", 200, csrf), ("static/admin", 200, &[])])),
        ("DJANGO", sample(&[("static/admin", 200, &[]), ("api", 200, csrf)])),
    ]
}

fn profile_index(name: &str) -> usize {
    PROFILES.iter().position(|p| *p == name).unwrap()
}

#[test]
fn confusion_matrix_and_margin_sweep() {
    let data = labelled_dataset();
    let n = PROFILES.len();
    let mut matrix = vec![vec![0usize; n]; n]; // matrix[true][pred]

    for (truth, probes) in &data {
        let (pred, _) = ml::fingerprint(probes);
        matrix[profile_index(truth)][profile_index(&pred)] += 1;
    }

    // ---- print a markdown confusion matrix ----
    let short = |p: &str| match p {
        "REST_API" => "REST",
        "ENTERPRISE_JAVA_SPRING" => "SPRING",
        "WORDPRESS_CMS" => "WP",
        "LEGACY_STATIC" => "STATIC",
        "PHP_GENERIC" => "PHP",
        "NODE_SPA" => "SPA",
        "DJANGO" => "DJANGO",
        _ => "?",
    };
    println!("\nConfusion matrix (rows = true, cols = predicted):\n");
    print!("| true \\ pred |");
    for p in PROFILES {
        print!(" {} |", short(p));
    }
    println!(" recall |");
    print!("|---|");
    for _ in PROFILES {
        print!("---:|");
    }
    println!("---:|");
    let mut correct = 0usize;
    for (i, p) in PROFILES.iter().enumerate() {
        let row_total: usize = matrix[i].iter().sum();
        print!("| **{}** |", short(p));
        for j in 0..n {
            print!(" {} |", matrix[i][j]);
        }
        correct += matrix[i][i];
        let recall = if row_total > 0 {
            matrix[i][i] as f64 / row_total as f64
        } else {
            0.0
        };
        println!(" {:.2} |", recall);
    }
    // precision row
    print!("| **precision** |");
    for j in 0..n {
        let col_total: usize = (0..n).map(|i| matrix[i][j]).sum();
        let prec = if col_total > 0 {
            matrix[j][j] as f64 / col_total as f64
        } else {
            0.0
        };
        print!(" {:.2} |", prec);
    }
    let acc = correct as f64 / data.len() as f64;
    println!(" acc={:.2} |", acc);

    // ---- margin sweep for the confidence gate ----
    println!("\nGate margin sweep (confident = classified, else abstained):\n");
    println!("| margin | confident | accuracy-when-confident | abstained |");
    println!("|---:|---:|---:|---:|");
    for margin in [0.0, 0.10, 0.25, 0.5, 1.0] {
        let mut confident = 0usize;
        let mut conf_correct = 0usize;
        for (truth, probes) in &data {
            let (pred, is_conf) = ml::fingerprint_gated(probes, margin);
            if is_conf {
                confident += 1;
                if pred == *truth {
                    conf_correct += 1;
                }
            }
        }
        let acc_conf = if confident > 0 {
            conf_correct as f64 / confident as f64
        } else {
            0.0
        };
        println!(
            "| {:.2} | {}/{} | {:.2} | {} |",
            margin,
            confident,
            data.len(),
            acc_conf,
            data.len() - confident
        );
    }

    // ---- field confusion matrix over the real jsintel lab captures ----
    let labs = lab_dataset();
    let mut field = vec![vec![0usize; n]; n];
    println!("\nField predictions (real jsintel labs, best-judgment true labels):\n");
    println!("| lab | true | predicted | correct |");
    println!("|---|---|---|:-:|");
    let mut field_correct = 0usize;
    for (lab, truth, probes) in &labs {
        let (pred, _) = ml::fingerprint(probes);
        let ok = pred == *truth;
        if ok {
            field_correct += 1;
        }
        field[profile_index(truth)][profile_index(&pred)] += 1;
        println!(
            "| {} | {} | {} | {} |",
            lab,
            short(truth),
            short(&pred),
            if ok { "✓" } else { "✗" }
        );
    }
    println!("\nField confusion matrix (rows = true, cols = predicted):\n");
    print!("| true \\ pred |");
    for p in PROFILES {
        print!(" {} |", short(p));
    }
    println!();
    print!("|---|");
    for _ in PROFILES {
        print!("---:|");
    }
    println!();
    for (i, p) in PROFILES.iter().enumerate() {
        print!("| **{}** |", short(p));
        for j in 0..n {
            print!(" {} |", field[i][j]);
        }
        println!();
    }
    println!(
        "\nField accuracy: {}/{} = {:.2}",
        field_correct,
        labs.len(),
        field_correct as f64 / labs.len() as f64
    );

    // ---- ENHANCEMENT E1: catch-all guard (shipped in the engine) ----
    // ml::is_catch_all abstains on servers that answer ~every probe with no strong
    // discriminator (wp cookie, JSESSIONID, xmlrpc 405). Compare the raw classifier
    // (always commits) against the shipped guard on the real field labs.
    println!("\nEnhancement — catch-all guard, baseline vs guarded (field labs):\n");
    println!("| | confident-correct | confident-wrong | abstained |");
    println!("|---|---:|---:|---:|");
    let (mut b_c, mut b_w) = (0, 0);
    let (mut g_c, mut g_w, mut g_a) = (0, 0, 0);
    for (_, truth, probes) in &labs {
        let (pred, _) = ml::fingerprint(probes);
        if pred == *truth {
            b_c += 1
        } else {
            b_w += 1
        }
        if ml::is_catch_all(probes) {
            g_a += 1;
        } else if pred == *truth {
            g_c += 1;
        } else {
            g_w += 1;
        }
    }
    println!("| baseline | {b_c} | {b_w} | 0 |");
    println!("| + catch-all guard | {g_c} | {g_w} | {g_a} |");

    // The guard must strictly cut confident mistakes, and must only ever abstain
    // genuine catch-alls — so it never drops a *trustworthy* correct answer. (A
    // correct raw guess on a catch-all is coincidence: the server answers every
    // path, so its classification can't be trusted even when it happens to match.)
    assert!(g_w < b_w, "catch-all guard should cut confident errors ({b_w} -> {g_w})");
    for (lab, truth, probes) in &labs {
        if !ml::is_catch_all(probes) {
            // non-catch-all targets are never abstained: their raw verdict stands
            let (pred, _) = ml::fingerprint(probes);
            let (gated, confident) = ml::fingerprint_gated(probes, 0.10);
            if pred == *truth {
                assert!(
                    confident && gated == *truth,
                    "{lab}: correct non-catch-all answer must be kept, got {gated} confident={confident}"
                );
            }
        }
    }

    // ---- regression guards ----
    // the classifier gets the clean dataset right ...
    assert!(
        acc >= 0.90,
        "fingerprint accuracy on the clean labelled set regressed: {acc:.2}"
    );
    // ... and the one unambiguous field case (WordPress) stays correct
    let wp = labs.iter().find(|(l, _, _)| *l == "wordpress").unwrap();
    assert_eq!(ml::fingerprint(&wp.2).0, "WORDPRESS_CMS");
}
