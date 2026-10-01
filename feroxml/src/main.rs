//! feroxml CLI — adaptive ML-driven fuzzing on top of feroxbuster.

use std::process::exit;

use clap::Parser;

use feroxml::config::Config;
use feroxml::ferox::RealRunner;
use feroxml::orchestrator::Campaign;

#[derive(Parser, Debug)]
#[command(
    name = "feroxml",
    version,
    about = "Adaptive, ML-driven fuzzing orchestrator for feroxbuster (K-Means fingerprinting + variable-order Markov prediction + Thompson-sampling scheduling)."
)]
struct Cli {
    /// Target URL, e.g. https://target.example (required unless --targets/--learn)
    #[arg(short, long)]
    url: Option<String>,

    /// Required acknowledgement that you are authorized to test the target(s).
    #[arg(long = "i-have-authorization")]
    authorized: bool,

    /// LEARN MODE: fingerprint and harvest path structure from the given
    /// target(s) and train the Markov model (written to --model), instead of
    /// scanning. Bootstraps the engine from your authorized labs.
    #[arg(long)]
    learn: bool,

    /// File of target URLs, one per line (used by --learn, or to scan several).
    #[arg(long)]
    targets: Option<String>,

    /// Learned-model path. In scan mode it is loaded (merged onto the profile
    /// seed) and updated; in --learn mode it is where the trained model is saved.
    #[arg(long)]
    model: Option<String>,

    /// Optional TOML config file (overrides defaults; CLI flags override it).
    #[arg(long)]
    config: Option<String>,

    /// Extra in-scope hosts/domains (repeatable).
    #[arg(long = "scope")]
    scope: Vec<String>,

    /// feroxbuster binary to drive.
    #[arg(long)]
    ferox_binary: Option<String>,

    /// Top-N predicted tokens per expansion.
    #[arg(long)]
    top_n: Option<usize>,

    /// Max feedback rounds.
    #[arg(long)]
    max_rounds: Option<usize>,

    /// Max recursion depth.
    #[arg(long)]
    max_depth: Option<usize>,

    /// Global request budget.
    #[arg(long)]
    request_budget: Option<usize>,

    /// Requests/sec cap passed to feroxbuster (0 = unlimited).
    #[arg(long)]
    rate_limit: Option<usize>,

    /// Concurrent threads passed to feroxbuster.
    #[arg(long)]
    threads: Option<usize>,

    /// Disable TLS verification (adds -k to feroxbuster).
    #[arg(short = 'k', long = "insecure")]
    insecure: bool,

    /// Scheduler: thompson | ucb1 | round_robin.
    #[arg(long)]
    scheduler: Option<String>,

    /// Classifier: nearest_centroid | kmeans.
    #[arg(long)]
    classifier: Option<String>,

    /// Deterministic seed.
    #[arg(long)]
    seed: Option<u64>,

    /// Base wordlist for hybrid coverage (adds baseline paths alongside the ML
    /// predictions). Omit for pure prediction-driven discovery.
    #[arg(long = "seed-wordlist")]
    seed_wordlist: Option<String>,

    /// Base-wordlist entries to try per expansion (requires --seed-wordlist).
    #[arg(long = "seed-per-round")]
    seed_per_round: Option<usize>,

    /// Let feroxbuster extract links from responses (mixes crawler results into
    /// discoveries; off by default so hits are attributable to the ML engine).
    #[arg(long = "extract-links")]
    extract_links: bool,
}

fn build_config(cli: &Cli) -> anyhow::Result<Config> {
    let mut cfg = match &cli.config {
        Some(path) => Config::from_toml_file(path)?,
        None => Config::default(),
    };
    // CLI overrides
    if let Some(v) = &cli.ferox_binary {
        cfg.ferox_binary = v.clone();
    }
    if let Some(v) = cli.top_n {
        cfg.top_n = v;
    }
    if let Some(v) = cli.max_rounds {
        cfg.max_rounds = v;
    }
    if let Some(v) = cli.max_depth {
        cfg.max_depth = v;
    }
    if let Some(v) = cli.request_budget {
        cfg.request_budget = v;
    }
    if let Some(v) = cli.rate_limit {
        cfg.rate_limit = v;
    }
    if let Some(v) = cli.threads {
        cfg.threads = v;
    }
    if cli.insecure {
        cfg.tls_verify = false;
    }
    if let Some(v) = &cli.scheduler {
        cfg.scheduler = v.clone();
    }
    if let Some(v) = &cli.classifier {
        cfg.classifier = v.clone();
    }
    if let Some(v) = cli.seed {
        cfg.seed = v;
    }
    if let Some(v) = &cli.model {
        cfg.model_path = v.clone();
    }
    if let Some(v) = &cli.seed_wordlist {
        cfg.seed_wordlist = v.clone();
        if cfg.seed_per_round == 0 {
            cfg.seed_per_round = 200; // sensible default once a list is provided
        }
    }
    if let Some(v) = cli.seed_per_round {
        cfg.seed_per_round = v;
    }
    if cli.extract_links {
        cfg.ferox_extract_links = true;
    }
    if !cli.scope.is_empty() {
        cfg.scope = cli.scope.clone();
    }
    Ok(cfg)
}

fn preflight(cfg: &Config) -> anyhow::Result<()> {
    // is the feroxbuster binary reachable?
    let ok = std::process::Command::new(&cfg.ferox_binary)
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !ok {
        anyhow::bail!(
            "feroxbuster binary '{}' not found or not runnable. Install it or pass --ferox-binary <path>.",
            cfg.ferox_binary
        );
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();

    if !cli.authorized {
        eprintln!(
            "refusing to run without authorization.\n\
             Only scan systems you own or have explicit written permission to test.\n\
             Re-run with --i-have-authorization once that is true."
        );
        exit(2);
    }

    let cfg = match build_config(&cli) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[config error] {e}");
            exit(2);
        }
    };

    if let Err(e) = preflight(&cfg) {
        eprintln!("[preflight] {e}");
        exit(3);
    }

    // assemble target list from --targets file and/or --url
    let mut targets: Vec<String> = Vec::new();
    if let Some(path) = &cli.targets {
        match std::fs::read_to_string(path) {
            Ok(text) => targets.extend(
                text.lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty() && !l.starts_with('#'))
                    .map(|l| l.to_string()),
            ),
            Err(e) => {
                eprintln!("[config error] cannot read --targets {path}: {e}");
                exit(2);
            }
        }
    }
    if let Some(u) = &cli.url {
        targets.push(u.clone());
    }
    // normalize bare domains/subdomains (e.g. "api.example.com") to URLs so a plain
    // host list (one per line) works directly with --targets.
    for t in targets.iter_mut() {
        if !t.contains("://") {
            *t = format!("https://{t}");
        }
    }
    if targets.is_empty() {
        eprintln!("no targets: pass --url <URL> or --targets <file>.");
        exit(2);
    }

    let model_path = if cfg.model_path.is_empty() {
        format!("{}/model.json", cfg.state_dir)
    } else {
        cfg.model_path.clone()
    };

    // capture display values before cfg is moved into the campaign
    let scope_note = format!("{:?}", cfg.scope);
    let engines_note = format!(
        "classifier={} predictor={} scheduler={} ranker={}",
        cfg.classifier, cfg.predictor, cfg.scheduler, cfg.ranker
    );
    let budget_note = format!(
        "rounds<={} depth<={} requests<={}",
        cfg.max_rounds, cfg.max_depth, cfg.request_budget
    );

    let runner = RealRunner::new(cfg.clone());
    let mut campaign = Campaign::new(cfg, Box::new(runner));

    // ---- LEARN MODE ----
    if cli.learn {
        eprintln!("== feroxml (learn) ==");
        eprintln!("targets     : {}", targets.len());
        eprintln!("model out   : {model_path}");
        match campaign.learn(&targets, &model_path) {
            Ok(s) => {
                println!("\n=== feroxml learn summary ===");
                println!("targets learned : {}", s.targets_learned);
                println!("paths ingested  : {}", s.paths_ingested);
                println!("model contexts  : {}", s.contexts);
                println!("model transitions: {}", s.transitions);
                let mut profs: Vec<_> = s.profiles.iter().collect();
                profs.sort_by(|a, b| b.1.cmp(a.1));
                print!("profiles seen   :");
                for (p, n) in profs {
                    print!(" {p}={n}");
                }
                println!();
                println!("model saved     : {}", s.model_path);
            }
            Err(e) => {
                eprintln!("[error] learn failed: {e}");
                exit(1);
            }
        }
        return;
    }

    // ---- SCAN MODE ---- (first target; use --targets for a sweep later)
    let target = targets[0].clone();
    eprintln!("== feroxml ==");
    eprintln!("target      : {target}");
    eprintln!("scope       : {scope_note}");
    eprintln!("engines     : {engines_note}");
    eprintln!("budget      : {budget_note}");
    eprintln!(
        "model       : {}",
        if std::path::Path::new(&model_path).exists() {
            model_path.as_str()
        } else {
            "(none yet)"
        }
    );

    match campaign.run(&target) {
        Ok(summary) => {
            println!("\n=== feroxml summary ===");
            println!("target            : {}", summary.target);
            println!("detected profile  : {}", summary.profile);
            print!("centroid distances:");
            for (name, d) in &summary.distances {
                print!(" {name}={d:.3}");
            }
            println!();
            println!("rounds run        : {}", summary.rounds);
            println!("requests used     : {}", summary.requests_used);
            println!("resources found   : {}", summary.discovered.len());
            println!("predicted hits    : {}", summary.predicted_hits);
            println!("dropped (scope)   : {}", summary.dropped_out_of_scope);
            println!("filtered (soft404): {}", summary.filtered_soft404);
            println!("\n--- discovered ---");
            for (url, status) in &summary.discovered {
                println!("{status:>3}  {url}");
            }
        }
        Err(e) => {
            eprintln!("[error] campaign failed: {e}");
            exit(1);
        }
    }
}
