//! Broad scenario coverage for every `ferox-ml-core` engine, exercised through
//! the public API exactly as the two tools consume it. These complement the
//! focused unit tests inside each module.

use ferox_ml_core::dedup::{hamming, simhash, Signature, SoftNotFoundFilter};
use ferox_ml_core::fingerprint::{self, feature_vector, NearestCentroid};
use ferox_ml_core::interfaces::{Classifier, Predictor, Scheduler};
use ferox_ml_core::markov::MarkovModel;
use ferox_ml_core::profiles::{centroids, seed_matrix, N_FEATURES, PROBE_PATHS};
use ferox_ml_core::ranking::Bm25;
use ferox_ml_core::rng::Rng;
use ferox_ml_core::scheduler;
use ferox_ml_core::tokenize::{last_segment, path_segments, subword_tokens};
use ferox_ml_core::{ProbeResp, PROFILES};

// ------------------------------- fingerprint -------------------------------

#[test]
fn every_profile_centroid_classifies_to_itself() {
    // the defining property of the seed centroids
    for (name, centroid) in centroids() {
        let (got, dists) = NearestCentroid.classify(&centroid);
        assert_eq!(got, name, "centroid for {name} must classify to {name}");
        assert_eq!(dists.len(), PROFILES.len());
    }
}

#[test]
fn feature_vector_has_fixed_width_and_is_header_case_insensitive() {
    let mut r = ProbeResp::new("https://x.test/api", 500);
    r.headers.insert("X-Powered-By".into(), "PHP/8.2".into());
    assert_eq!(r.header("x-powered-by"), Some("PHP/8.2"));
    let fv = feature_vector(&[r]);
    assert_eq!(fv.len(), N_FEATURES);
}

#[test]
fn rest_api_recognized_even_when_base_paths_500() {
    // a JSON root + versioned API + swagger is unmistakably REST, even though the
    // API base paths error (500) rather than 200
    let mut root = ProbeResp::new("https://x.test/", 200);
    root.headers
        .insert("content-type".into(), "application/json".into());
    let probes = vec![
        root,
        ProbeResp::new("https://x.test/api", 500),
        ProbeResp::new("https://x.test/api/v1", 500),
        ProbeResp::new("https://x.test/swagger-ui.html", 200),
        ProbeResp::new("https://x.test/openapi.json", 200),
    ];
    assert_eq!(NearestCentroid.classify(&feature_vector(&probes)).0, "REST_API");
}

#[test]
fn spring_recognized_from_actuator_and_servlet_signals() {
    let mut root = ProbeResp::new("https://x.test/", 200);
    root.headers
        .insert("set-cookie".into(), "JSESSIONID=abc123; Path=/".into());
    root.headers
        .insert("server".into(), "Apache-Coyote/1.1".into());
    let probes = vec![
        root,
        ProbeResp::new("https://x.test/actuator", 200),
        ProbeResp::new("https://x.test/actuator/health", 200),
        ProbeResp::new("https://x.test/index.jsp", 200),
    ];
    assert_eq!(
        NearestCentroid.classify(&feature_vector(&probes)).0,
        "ENTERPRISE_JAVA_SPRING"
    );
}

#[test]
fn empty_or_all_404_probes_are_static() {
    let none: Vec<ProbeResp> = vec![];
    assert_eq!(NearestCentroid.classify(&feature_vector(&none)).0, "LEGACY_STATIC");
    let all404 = vec![
        ProbeResp::new("https://x.test/api", 404),
        ProbeResp::new("https://x.test/wp-json", 404),
    ];
    assert_eq!(NearestCentroid.classify(&feature_vector(&all404)).0, "LEGACY_STATIC");
}

#[test]
fn classifier_build_dispatches() {
    // both named classifiers reduce to nearest-centroid for a single vector
    let probes = vec![ProbeResp::new("https://x.test/wp-json", 200)];
    let fv = feature_vector(&probes);
    let a = fingerprint::build("kmeans", 7).classify(&fv).0;
    let b = fingerprint::build("nearest", 7).classify(&fv).0;
    assert_eq!(a, b);
}

// --------------------------------- markov ----------------------------------

#[test]
fn seed_predicts_profile_chain_and_respects_top_n() {
    let m = MarkovModel::seeded("REST_API", 3, 0.5, 0.0);
    let preds = m.predict("https://x.test/api", 2);
    assert!(preds.len() <= 2);
    assert!(preds.iter().any(|(t, _)| t == "v1"));
}

#[test]
fn online_learning_adds_a_new_transition() {
    let mut m = MarkovModel::new(3, 0.5, 0.0);
    assert!(m.predict("/shop/cart", 5).is_empty());
    m.learn("https://x.test/shop/cart/items");
    assert!(m.predict("/shop/cart", 5).iter().any(|(t, _)| t == "items"));
}

#[test]
fn variable_order_backoff_uses_shorter_context_when_needed() {
    let mut m = MarkovModel::new(3, 0.5, 0.0);
    m.learn("https://x.test/a/b/c");
    // exact high-order context predicts c
    assert!(m.predict("https://x.test/a/b", 5).iter().any(|(t, _)| t == "c"));
    // an unseen longer prefix still backs off to the b -> c transition
    assert!(m.predict("https://x.test/z/a/b", 5).iter().any(|(t, _)| t == "c"));
}

#[test]
fn merge_combines_two_models() {
    let mut a = MarkovModel::new(3, 0.5, 0.0);
    a.learn("https://x.test/one/alpha");
    let mut b = MarkovModel::new(3, 0.5, 0.0);
    b.learn("https://x.test/one/beta");
    a.merge(&b);
    let preds: Vec<String> = a.predict("https://x.test/one", 5).into_iter().map(|(t, _)| t).collect();
    assert!(preds.contains(&"alpha".to_string()) && preds.contains(&"beta".to_string()));
}

#[test]
fn json_roundtrip_preserves_learned_transitions() {
    let mut m = MarkovModel::seeded("WORDPRESS_CMS", 3, 0.5, 0.01);
    m.learn("https://x.test/wp-content/uploads/2099");
    let json = m.to_json().unwrap();
    let back = MarkovModel::from_json(&json).unwrap();
    assert_eq!(
        m.predict("https://x.test/wp-content/uploads", 5),
        back.predict("https://x.test/wp-content/uploads", 5)
    );
    assert_eq!(m.context_count(), back.context_count());
    assert_eq!(m.transition_count(), back.transition_count());
}

#[test]
fn order_one_model_ignores_longer_context() {
    let mut m = MarkovModel::new(1, 0.5, 0.0);
    m.learn("https://x.test/a/b/c");
    // with max_order 1, prediction keys only on the last segment
    assert!(m.predict("https://x.test/a/b", 5).iter().any(|(t, _)| t == "c"));
}

// -------------------------------- scheduler --------------------------------

#[test]
fn thompson_value_tracks_reward_direction() {
    let mut s = scheduler::build("thompson", 1);
    s.add_arm("good");
    s.add_arm("bad");
    for _ in 0..20 {
        s.update("good", 1.0);
        s.update("bad", 0.0);
    }
    assert!(s.value("good") > 0.8, "good value {}", s.value("good"));
    assert!(s.value("bad") < 0.2, "bad value {}", s.value("bad"));
    assert_eq!(s.value("never-seen"), 0.5);
}

#[test]
fn ucb1_value_is_mean_reward_and_explores_unpulled() {
    let mut s = scheduler::build("ucb1", 1);
    s.add_arm("a");
    assert_eq!(s.value("a"), 0.5, "unpulled arm is neutral");
    s.update("a", 1.0);
    s.update("a", 0.0);
    assert!((s.value("a") - 0.5).abs() < 1e-9, "mean of 1 and 0");
    let first = s.choose();
    assert!(first.is_some());
}

#[test]
fn round_robin_visits_each_once_then_none() {
    let mut s = scheduler::build("round_robin", 1);
    s.add_arm("a");
    s.add_arm("b");
    s.add_arm("a"); // duplicate ignored
    assert_eq!(s.choose().as_deref(), Some("a"));
    assert_eq!(s.choose().as_deref(), Some("b"));
    assert_eq!(s.choose(), None);
    assert_eq!(s.value("a"), 0.5); // round robin keeps no value estimate
}

#[test]
fn unknown_scheduler_name_defaults_to_thompson_behavior() {
    let mut s = scheduler::build("does-not-exist", 42);
    s.add_arm("x");
    s.update("x", 1.0);
    assert!(s.value("x") > 0.5);
}

#[test]
fn choose_on_empty_scheduler_is_none() {
    let mut s = scheduler::build("thompson", 1);
    assert_eq!(s.choose(), None);
}

// --------------------------------- ranking ---------------------------------

#[test]
fn bm25_empty_corpus_scores_zero_and_rerank_is_identity_order() {
    let bm = Bm25::new();
    assert_eq!(bm.score("anything"), 0.0);
    let preds = vec![("b".to_string(), 0.2), ("a".to_string(), 0.3)];
    let out = bm.rerank(&preds);
    assert_eq!(out.len(), 2);
    // highest prior first when corpus can't discriminate
    assert_eq!(out[0].0, "a");
}

#[test]
fn bm25_promotes_corpus_relevant_candidate() {
    let mut bm = Bm25::new();
    for seg in ["api", "api-users", "api-auth"] {
        bm.add_document(seg);
    }
    assert!(bm.score("users") > bm.score("zzzzz"));
    let ranked = bm.rerank(&[("random".into(), 0.30), ("users".into(), 0.25)]);
    assert_eq!(ranked[0].0, "users");
    assert_eq!(ranked.len(), 2, "rerank preserves all candidates");
}

// ---------------------------------- dedup ----------------------------------

#[test]
fn simhash_is_stable_and_discriminating() {
    assert_eq!(simhash("the quick brown fox"), simhash("the quick brown fox"));
    assert_eq!(simhash(""), 0);
    let a = simhash("page not found 404 error");
    let b = simhash("welcome to your account dashboard");
    assert!(hamming(a, b) > 5, "dissimilar texts differ in many bits");
    assert_eq!(hamming(a, a), 0);
}

#[test]
fn soft_not_found_filter_matches_learned_signatures() {
    let mut f = SoftNotFoundFilter::new();
    let bogus = Signature::new(200, 1500, 42, 10);
    f.learn_bogus(bogus);
    // same 16-byte content-length bucket (1488..=1503 -> bucket 93) still matches
    assert!(f.is_soft_not_found(&Signature::new(200, 1495, 42, 10)));
    // a genuinely different response does not
    assert!(!f.is_soft_not_found(&Signature::new(200, 99000, 900, 300)));
}

// --------------------------------- tokenize --------------------------------

#[test]
fn tokenizers_handle_urls_paths_and_subwords() {
    assert_eq!(path_segments("https://x.test/api/v1/users"), ["api", "v1", "users"]);
    assert_eq!(path_segments("/wp-content/plugins/"), ["wp-content", "plugins"]);
    assert_eq!(path_segments("/a?b=c#d"), ["a"]);
    assert!(path_segments("https://x.test/").is_empty());
    assert_eq!(last_segment("/api/v1/users"), "users");
    assert_eq!(last_segment("https://x.test/"), "");
    assert_eq!(subword_tokens("getUserById"), ["get", "user", "by", "id"]);
    assert_eq!(subword_tokens("wp-login.php"), ["wp", "login", "php"]);
    assert!(subword_tokens("").is_empty());
}

// ----------------------------------- rng -----------------------------------

#[test]
fn rng_is_deterministic_and_bounded() {
    let mut a = Rng::new(123);
    let mut b = Rng::new(123);
    for _ in 0..50 {
        let x = a.uniform();
        assert!((0.0..1.0).contains(&x));
        assert_eq!(x, b.uniform(), "same seed -> same sequence");
    }
    let mut r = Rng::new(9);
    for _ in 0..100 {
        let s = r.beta(2.0, 5.0);
        assert!((0.0..=1.0).contains(&s));
    }
}

// --------------------------------- profiles --------------------------------

#[test]
fn profile_tables_are_well_formed() {
    assert_eq!(PROFILES.len(), 4);
    assert_eq!(centroids().len(), 4);
    for (_, c) in centroids() {
        assert_eq!(c.len(), N_FEATURES);
    }
    assert!(!PROBE_PATHS.is_empty());
    for p in PROFILES {
        assert!(!seed_matrix(p).is_empty(), "{p} must ship a seed matrix");
    }
    assert!(seed_matrix("NOPE").is_empty(), "unknown profile -> no seed");
}
