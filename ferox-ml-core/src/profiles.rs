//! The four framework profiles: feature-vector centroids (Phase 1) and full
//! seed Markov transition matrices (Phase 2). Everything here is real data, not
//! placeholders — the tool is functional on first run from these seeds.

/// Ordered names of the features in every target feature vector. The centroids
/// below and the fingerprint module's vectorizer use this exact order.
pub const FEATURE_NAMES: [&str; 24] = [
    "wp_json",           // 0  /wp-json present
    "wp_login",          // 1  /wp-login.php present
    "actuator",          // 2  /actuator present
    "actuator_health",   // 3  /actuator/health present
    "api",               // 4  /api present
    "api_v1",            // 5  /api/v1 present
    "swagger_openapi",   // 6  swagger / openapi.json / v2/api-docs present
    "git_head",          // 7  /.git/HEAD present
    "robots_txt",        // 8  /robots.txt present
    "root_json",         // 9  root Content-Type is application/json
    "x_powered_by",      // 10 any X-Powered-By header seen
    "server_java",       // 11 Server/X-Powered-By indicates a servlet container
    "cookie_jsessionid", // 12 Set-Cookie: JSESSIONID
    "cookie_wordpress",  // 13 Set-Cookie: wordpress_/wp-
    "cookie_phpsessid",  // 14 Set-Cookie: PHPSESSID
    "sig_php",           // 15 PHP signal (X-Powered-By: PHP, .php)
    "sig_jsp",           // 16 JSP/servlet signal
    "sig_static",        // 17 looks like a static/legacy site
    "sig_spa",           // 18 SPA build output (manifest.webmanifest, /_next, /assets)
    "sig_django",        // 19 Django signal (csrftoken cookie, /static/admin)
    "cookie_node",       // 20 Node session cookie (connect.sid / next-auth)
    "cookie_php_fw",     // 21 PHP-framework session (laravel / codeigniter / symfony)
    "sec_headers",       // 22 modern security headers (CSP / HSTS / X-Frame-Options)
    "heavy_html",        // 23 a real rendered HTML page was served (vs JSON/404)
];

pub const N_FEATURES: usize = FEATURE_NAMES.len();

/// E3 — per-feature weights for the (weighted-Euclidean) nearest-centroid metric,
/// aligned 1:1 with [`FEATURE_NAMES`]. Strong, hard-to-fake discriminators
/// (session cookies, servlet/PHP/JSP signals, `X-Powered-By`) are weighted above
/// noisy path-presence bits; ubiquitous signals (`robots.txt`) are weighted down.
/// So a target with a weak accidental marker but a strong contradicting
/// discriminator is classified by the discriminator, not the noise. A centroid
/// still classifies to itself under any positive weights (its distance is 0).
pub const FEATURE_WEIGHTS: [f64; N_FEATURES] = [
    1.5, // 0  wp_json
    2.0, // 1  wp_login
    2.0, // 2  actuator
    1.5, // 3  actuator_health
    1.0, // 4  api
    1.0, // 5  api_v1
    1.5, // 6  swagger_openapi
    1.0, // 7  git_head
    0.5, // 8  robots_txt (ubiquitous -> down-weighted)
    1.0, // 9  root_json
    2.0, // 10 x_powered_by
    3.0, // 11 server_java
    3.0, // 12 cookie_jsessionid
    3.0, // 13 cookie_wordpress
    2.0, // 14 cookie_phpsessid
    2.0, // 15 sig_php
    2.5, // 16 sig_jsp
    1.0, // 17 sig_static
    1.5, // 18 sig_spa
    2.0, // 19 sig_django
    2.5, // 20 cookie_node (strong, Node-specific)
    2.5, // 21 cookie_php_fw (strong, PHP-framework-specific)
    1.0, // 22 sec_headers (common; modern-vs-legacy hint)
    1.0, // 23 heavy_html (page-shape hint)
];

/// `(profile_name, centroid)` for each of the four profiles. Each centroid is
/// an [`N_FEATURES`]-dim vector in `[0, 1]`; a probed target is assigned to the
/// nearest centroid (and these seed KMeans' initial centers).
pub fn centroids() -> Vec<(&'static str, [f64; N_FEATURES])> {
    // indices 0..=17 as before; 18 = sig_spa, 19 = sig_django
    vec![
        (
            "REST_API",
            // api/apiv1/swagger/json-heavy, no wp, no servlet
            [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.9, 0.1, 0.5, 1.0, 0.4, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.6, 0.1],
        ),
        (
            "ENTERPRISE_JAVA_SPRING",
            // actuator + servlet cookies + jsp signal
            [0.0, 0.0, 1.0, 1.0, 0.6, 0.5, 0.6, 0.1, 0.4, 0.4, 0.3, 1.0, 1.0, 0.0, 0.0, 0.0, 0.9, 0.1, 0.0, 0.0, 0.0, 0.0, 0.6, 0.3],
        ),
        (
            "WORDPRESS_CMS",
            // wp-json/wp-login + wp cookie + php signal
            [1.0, 1.0, 0.0, 0.0, 0.2, 0.1, 0.1, 0.1, 0.7, 0.0, 0.7, 0.0, 0.0, 1.0, 0.5, 0.9, 0.0, 0.1, 0.0, 0.0, 0.0, 0.1, 0.3, 0.8],
        ),
        (
            "LEGACY_STATIC",
            // almost nothing dynamic; static signal high
            [0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.1, 0.6, 0.0, 0.1, 0.0, 0.0, 0.0, 0.2, 0.2, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.1, 0.5],
        ),
        (
            "PHP_GENERIC",
            // PHP app that is NOT WordPress: PHP signals + PHPSESSID, no wp markers
            [0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.1, 0.5, 0.0, 0.8, 0.0, 0.0, 0.0, 1.0, 1.0, 0.0, 0.3, 0.0, 0.0, 0.0, 0.6, 0.2, 0.6],
        ),
        (
            "NODE_SPA",
            // single-page app: SPA build output, maybe a REST backend, HTML root
            [0.0, 0.0, 0.0, 0.0, 0.4, 0.2, 0.1, 0.1, 0.3, 0.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 1.0, 0.0, 0.7, 0.0, 0.6, 0.9],
        ),
        (
            "DJANGO",
            // Django: csrftoken/static-admin signal, often a DRF api, no x-powered-by
            [0.0, 0.0, 0.0, 0.0, 0.3, 0.1, 0.1, 0.1, 0.4, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.2, 0.0, 1.0, 0.0, 0.0, 0.6, 0.4],
        ),
    ]
}

/// A single row of a seed transition matrix: `(from_token, [(to_token, weight)])`.
type Row = (&'static str, &'static [(&'static str, f64)]);

/// Full seed transition matrices, one per profile. Weights need not sum to 1 —
/// the markov module normalizes them. Tokens are real path segments.
pub fn seed_matrix(profile: &str) -> Vec<Row> {
    match profile {
        "REST_API" => vec![
            ("", &[("api", 0.35), ("v1", 0.15), ("v2", 0.1), ("swagger", 0.1), ("openapi.json", 0.1), ("health", 0.1), ("graphql", 0.1)]),
            ("api", &[("v1", 0.4), ("v2", 0.25), ("v3", 0.08), ("users", 0.1), ("auth", 0.07), ("docs", 0.05), ("health", 0.05)]),
            ("v1", &[("users", 0.28), ("auth", 0.2), ("health", 0.12), ("orders", 0.1), ("products", 0.1), ("login", 0.1), ("admin", 0.1)]),
            ("v2", &[("users", 0.3), ("auth", 0.2), ("orders", 0.15), ("products", 0.15), ("health", 0.1), ("admin", 0.1)]),
            ("users", &[("me", 0.3), ("login", 0.2), ("search", 0.15), ("profile", 0.15), ("id", 0.2)]),
            ("auth", &[("login", 0.35), ("token", 0.25), ("refresh", 0.2), ("logout", 0.1), ("register", 0.1)]),
            ("swagger", &[("index.html", 0.4), ("ui", 0.3), ("swagger.json", 0.3)]),
        ],
        "ENTERPRISE_JAVA_SPRING" => vec![
            ("", &[("actuator", 0.3), ("api", 0.15), ("swagger-ui.html", 0.1), ("v2", 0.1), ("login", 0.1), ("admin", 0.1), ("services", 0.15)]),
            ("actuator", &[("health", 0.3), ("env", 0.18), ("info", 0.12), ("mappings", 0.12), ("beans", 0.1), ("metrics", 0.1), ("heapdump", 0.08)]),
            ("health", &[("readiness", 0.4), ("liveness", 0.4), ("db", 0.2)]),
            ("api", &[("v1", 0.35), ("v2", 0.25), ("users", 0.15), ("swagger", 0.15), ("health", 0.1)]),
            ("services", &[("rest", 0.3), ("soap", 0.2), ("user", 0.25), ("admin", 0.25)]),
            ("admin", &[("login", 0.35), ("console", 0.3), ("index.jsp", 0.2), ("users", 0.15)]),
            ("v2", &[("api-docs", 0.6), ("users", 0.2), ("swagger.json", 0.2)]),
        ],
        "WORDPRESS_CMS" => vec![
            ("", &[("wp-admin", 0.22), ("wp-content", 0.22), ("wp-includes", 0.15), ("wp-json", 0.15), ("wp-login.php", 0.14), ("xmlrpc.php", 0.12)]),
            ("wp-content", &[("plugins", 0.5), ("themes", 0.3), ("uploads", 0.2)]),
            ("wp-admin", &[("admin-ajax.php", 0.35), ("index.php", 0.25), ("login.php", 0.2), ("install.php", 0.1), ("setup-config.php", 0.1)]),
            ("wp-json", &[("wp", 0.7), ("oembed", 0.2), ("index", 0.1)]),
            ("wp", &[("v2", 0.7), ("v1", 0.3)]),
            ("plugins", &[("akismet", 0.25), ("woocommerce", 0.25), ("contact-form-7", 0.2), ("elementor", 0.15), ("jetpack", 0.15)]),
            ("themes", &[("twentytwentyone", 0.3), ("twentytwentytwo", 0.3), ("astra", 0.2), ("divi", 0.2)]),
            ("uploads", &[("2023", 0.3), ("2024", 0.4), ("2025", 0.3)]),
        ],
        "LEGACY_STATIC" => vec![
            ("", &[("index.html", 0.2), ("admin", 0.15), ("images", 0.12), ("css", 0.12), ("js", 0.12), ("about.html", 0.1), ("contact.html", 0.1), ("cgi-bin", 0.09)]),
            ("admin", &[("login", 0.3), ("index.php", 0.2), ("index.html", 0.2), ("admin.php", 0.15), ("config.php", 0.15)]),
            ("images", &[("logo.png", 0.4), ("banner.jpg", 0.3), ("icons", 0.3)]),
            ("css", &[("style.css", 0.6), ("main.css", 0.4)]),
            ("js", &[("main.js", 0.4), ("app.js", 0.3), ("jquery.js", 0.3)]),
            ("cgi-bin", &[("test.cgi", 0.4), ("admin.cgi", 0.3), ("status.cgi", 0.3)]),
        ],
        "PHP_GENERIC" => vec![
            ("", &[("index.php", 0.25), ("admin", 0.2), ("login.php", 0.12), ("config.php", 0.1), ("uploads", 0.1), ("includes", 0.1), ("api", 0.08), ("phpinfo.php", 0.05)]),
            ("admin", &[("index.php", 0.3), ("login.php", 0.25), ("config.php", 0.2), ("dashboard.php", 0.15), ("users.php", 0.1)]),
            ("includes", &[("config.php", 0.4), ("db.php", 0.3), ("functions.php", 0.3)]),
            ("uploads", &[("files", 0.4), ("images", 0.35), ("tmp", 0.25)]),
            ("api", &[("v1", 0.4), ("login.php", 0.3), ("user.php", 0.3)]),
        ],
        "NODE_SPA" => vec![
            ("", &[("api", 0.25), ("static", 0.18), ("assets", 0.18), ("_next", 0.12), ("manifest.webmanifest", 0.1), ("graphql", 0.1), ("favicon.ico", 0.07)]),
            ("api", &[("v1", 0.3), ("graphql", 0.2), ("users", 0.2), ("auth", 0.2), ("health", 0.1)]),
            ("assets", &[("index.js", 0.35), ("index.css", 0.3), ("vendor.js", 0.2), ("main.js", 0.15)]),
            ("_next", &[("static", 0.6), ("data", 0.25), ("image", 0.15)]),
            ("static", &[("js", 0.35), ("css", 0.35), ("media", 0.3)]),
        ],
        "DJANGO" => vec![
            ("", &[("admin", 0.25), ("api", 0.2), ("static", 0.15), ("accounts", 0.12), ("media", 0.1), ("graphql", 0.08), ("__debug__", 0.1)]),
            ("admin", &[("login", 0.4), ("", 0.2), ("auth", 0.2), ("jsi18n", 0.2)]),
            ("api", &[("v1", 0.35), ("auth", 0.2), ("users", 0.2), ("token", 0.15), ("schema", 0.1)]),
            ("accounts", &[("login", 0.4), ("logout", 0.25), ("password_reset", 0.2), ("register", 0.15)]),
            ("static", &[("admin", 0.5), ("rest_framework", 0.3), ("css", 0.2)]),
        ],
        _ => vec![],
    }
}

/// Discriminating paths the Phase-1 probe requests. Mapping of which of these
/// map to which feature index lives in the fingerprint module.
pub const PROBE_PATHS: [&str; 22] = [
    "wp-json",
    "wp-login.php",
    "actuator",
    "actuator/health",
    "api",
    "api/v1",
    "api/v2",
    "rest",
    "graphql",
    "swagger-ui.html",
    "openapi.json",
    "v2/api-docs",
    ".git/HEAD",
    "robots.txt",
    "xmlrpc.php",
    "index.php",
    "index.jsp",
    "server-status",
    "manifest.webmanifest", // SPA / PWA
    "_next",                // Next.js build output
    "assets",               // bundled SPA assets
    "static/admin",         // Django admin static tree
];

/// The framework profiles the fingerprinting engine can assign.
pub const PROFILES: [&str; 7] = [
    "REST_API",
    "ENTERPRISE_JAVA_SPRING",
    "WORDPRESS_CMS",
    "LEGACY_STATIC",
    "PHP_GENERIC",
    "NODE_SPA",
    "DJANGO",
];
