//! The four framework profiles: feature-vector centroids (Phase 1) and full
//! seed Markov transition matrices (Phase 2). Everything here is real data, not
//! placeholders — the tool is functional on first run from these seeds.

/// Ordered names of the features in every target feature vector. The centroids
/// below and [`crate::fingerprint`]'s vectorizer use this exact order.
pub const FEATURE_NAMES: [&str; 18] = [
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
];

pub const N_FEATURES: usize = FEATURE_NAMES.len();

/// `(profile_name, centroid)` for each of the four profiles. Each centroid is
/// an [`N_FEATURES`]-dim vector in `[0, 1]`; a probed target is assigned to the
/// nearest centroid (and these seed KMeans' initial centers).
pub fn centroids() -> Vec<(&'static str, [f64; N_FEATURES])> {
    vec![
        (
            "REST_API",
            // api/apiv1/swagger/json-heavy, no wp, no servlet
            [0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 0.9, 0.1, 0.5, 1.0, 0.4, 0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0],
        ),
        (
            "ENTERPRISE_JAVA_SPRING",
            // actuator + servlet cookies + jsp signal
            [0.0, 0.0, 1.0, 1.0, 0.6, 0.5, 0.6, 0.1, 0.4, 0.4, 0.3, 1.0, 1.0, 0.0, 0.0, 0.0, 0.9, 0.1],
        ),
        (
            "WORDPRESS_CMS",
            // wp-json/wp-login + wp cookie + php signal
            [1.0, 1.0, 0.0, 0.0, 0.2, 0.1, 0.1, 0.1, 0.7, 0.0, 0.7, 0.0, 0.0, 1.0, 0.5, 0.9, 0.0, 0.1],
        ),
        (
            "LEGACY_STATIC",
            // almost nothing dynamic; static signal high
            [0.0, 0.0, 0.0, 0.0, 0.1, 0.0, 0.0, 0.1, 0.6, 0.0, 0.1, 0.0, 0.0, 0.0, 0.2, 0.2, 0.0, 1.0],
        ),
    ]
}

/// A single row of a seed transition matrix: `(from_token, [(to_token, weight)])`.
type Row = (&'static str, &'static [(&'static str, f64)]);

/// Full seed transition matrices, one per profile. Weights need not sum to 1 —
/// [`crate::markov`] normalizes them. Tokens are real path segments.
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
        _ => vec![],
    }
}

/// Discriminating paths the Phase-1 probe requests. Mapping of which of these
/// map to which feature index lives in [`crate::fingerprint`].
pub const PROBE_PATHS: [&str; 18] = [
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
];
