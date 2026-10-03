use super::utils::*;
use super::*;
use crate::{traits::FeroxSerialize, DEFAULT_CONFIG_NAME};
use regex::Regex;
use reqwest::Url;
use std::{collections::HashMap, fs::write};
use tempfile::TempDir;

/// creates a dummy configuration file for testing
fn setup_config_test() -> Configuration {
    let data = r#"
            wordlist = "/some/path"
            status_codes = [201, 301, 401]
            replay_codes = [201, 301]
            threads = 40
            timeout = 5
            proxy = "http://127.0.0.1:8080"
            replay_proxy = "http://127.0.0.1:8081"
            quiet = true
            silent = true
            auto_tune = true
            auto_bail = true
            verbosity = 1
            scan_limit = 6
            parallel = 14
            rate_limit = 250
            time_limit = "10m"
            output = "/some/otherpath"
            debug_log = "/yet/anotherpath"
            resume_from = "/some/state/file"
            redirects = true
            insecure = true
            collect_backups = true
            collect_extensions = true
            collect_words = true
            extensions = ["html", "php", "js"]
            dont_collect = ["png", "gif", "jpg", "jpeg"]
            methods = ["GET", "PUT", "DELETE"]
            data = [31, 32, 33, 34]
            url_denylist = ["http://dont-scan.me", "https://also-not.me"]
            scope = ["http://example.com", "https://other.com"]
            regex_denylist = ["/deny.*"]
            headers = {stuff = "things", mostuff = "mothings"}
            queries = [["name","value"], ["rick", "astley"]]
            no_recursion = true
            add_slash = true
            stdin = true
            dont_filter = true
            extract_links = false
            json = true
            save_state = false
            depth = 1
            limit_bars = 3
            protocol = "http"
            request_file = "/some/request/file"
            scan_dir_listings = true
            force_recursion = true
            filter_size = [4120]
            filter_regex = ["^ignore me$"]
            filter_similar = ["https://somesite.com/soft404"]
            filter_word_count = [994, 992]
            filter_line_count = [34]
            filter_status = [201]
            server_certs = ["/some/cert.pem", "/some/other/cert.pem"]
            client_cert = "/some/client/cert.pem"
            client_key = "/some/client/key.pem"
            backup_extensions = [".save"]
            unique = true
            response_size_limit = 8388608
            ml = true
            ml_loop = true
            ml_list_dir = "/some/wordlists"
            ml_list_chunk = 300
            ml_list_max = 5000
            ml_algo = "dynsdt"
            ml_model = "/some/model.json"
            ml_order = 5
            ml_predictions = 40
            ml_rank = false
            ml_scheduler = "ucb1"
        "#;
    let tmp_dir = TempDir::new().unwrap();
    let file = tmp_dir.path().join(DEFAULT_CONFIG_NAME);
    write(&file, data).unwrap();
    Configuration::parse_config(file).unwrap()
}

#[test]
/// test that all default config values meet expectations
fn default_configuration() {
    let config = Configuration::default();
    assert_eq!(config.wordlist, wordlist());
    assert_eq!(config.proxy, String::new());
    assert_eq!(config.target_url, String::new());
    assert_eq!(config.time_limit, String::new());
    assert_eq!(config.resume_from, String::new());
    assert_eq!(config.debug_log, String::new());
    assert_eq!(config.config, String::new());
    assert_eq!(config.replay_proxy, String::new());
    assert_eq!(config.status_codes, status_codes());
    assert_eq!(config.replay_codes, config.status_codes);
    assert!(config.replay_client.is_none());
    assert_eq!(config.threads, threads());
    assert_eq!(config.depth, depth());
    assert_eq!(config.timeout, timeout());
    assert_eq!(config.verbosity, 0);
    assert_eq!(config.scan_limit, 0);
    assert_eq!(config.limit_bars, 0);
    assert!(!config.silent);
    assert!(!config.quiet);
    assert_eq!(config.output_level, OutputLevel::Default);
    assert!(!config.dont_filter);
    assert!(!config.auto_tune);
    assert!(!config.auto_bail);
    assert_eq!(config.requester_policy, RequesterPolicy::Default);
    assert!(!config.no_recursion);
    assert!(!config.random_agent);
    assert!(!config.json);
    assert!(config.save_state);
    assert!(!config.stdin);
    assert!(!config.add_slash);
    assert!(!config.force_recursion);
    assert!(!config.redirects);
    assert!(config.extract_links);
    assert!(!config.insecure);
    assert!(!config.collect_extensions);
    assert!(!config.collect_backups);
    assert!(!config.collect_words);
    assert!(!config.scan_dir_listings);
    assert!(config.regex_denylist.is_empty());
    assert_eq!(config.queries, Vec::new());
    assert_eq!(config.filter_size, Vec::<u64>::new());
    assert_eq!(config.extensions, Vec::<String>::new());
    assert_eq!(config.methods, vec!["GET"]);
    assert_eq!(config.data, Vec::<u8>::new());
    assert_eq!(config.url_denylist, Vec::<Url>::new());
    assert_eq!(config.scope, Vec::<Url>::new());
    assert_eq!(config.dont_collect, ignored_extensions());
    assert_eq!(config.filter_regex, Vec::<String>::new());
    assert_eq!(config.filter_similar, Vec::<String>::new());
    assert_eq!(config.filter_word_count, Vec::<usize>::new());
    assert_eq!(config.filter_line_count, Vec::<usize>::new());
    assert_eq!(config.filter_status, Vec::<u16>::new());
    assert_eq!(config.headers, HashMap::new());
    assert_eq!(config.server_certs, Vec::<String>::new());
    assert_eq!(config.client_cert, String::new());
    assert_eq!(config.client_key, String::new());
    assert_eq!(config.backup_extensions, backup_extensions());
    assert_eq!(config.protocol, request_protocol());
    assert_eq!(config.request_file, String::new());
    assert!(!config.unique);
    assert_eq!(config.response_size_limit, 4194304); // 4MB
    // ML layer defaults
    assert!(!config.ml);
    assert!(!config.ml_loop);
    assert_eq!(config.ml_list_dir, String::new());
    assert_eq!(config.ml_list_chunk, ml_list_chunk());
    assert_eq!(config.ml_list_chunk, 200);
    assert_eq!(config.ml_list_max, ml_list_max());
    assert_eq!(config.ml_list_max, 0); // 0 == unlimited
    assert_eq!(config.ml_algo, ml_algo());
    assert_eq!(config.ml_algo, "markov");
    assert_eq!(config.ml_model, String::new());
    assert_eq!(config.ml_order, ml_order());
    assert_eq!(config.ml_order, 3);
    assert_eq!(config.ml_predictions, ml_predictions());
    assert_eq!(config.ml_predictions, 25);
    assert!(config.ml_rank); // BM25 re-ranking on by default
    assert_eq!(config.ml_scheduler, ml_scheduler());
    assert_eq!(config.ml_scheduler, "thompson");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_wordlist() {
    let config = setup_config_test();
    assert_eq!(config.wordlist, vec![String::from("/some/path")]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_debug_log() {
    let config = setup_config_test();
    assert_eq!(config.debug_log, "/yet/anotherpath");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_status_codes() {
    let config = setup_config_test();
    assert_eq!(config.status_codes, vec![201, 301, 401]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_replay_codes() {
    let config = setup_config_test();
    assert_eq!(config.replay_codes, vec![201, 301]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_threads() {
    let config = setup_config_test();
    assert_eq!(config.threads, 40);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_depth() {
    let config = setup_config_test();
    assert_eq!(config.depth, 1);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_scan_limit() {
    let config = setup_config_test();
    assert_eq!(config.scan_limit, 6);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_parallel() {
    let config = setup_config_test();
    assert_eq!(config.parallel, 14);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_rate_limit() {
    let config = setup_config_test();
    assert_eq!(config.rate_limit, 250);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_timeout() {
    let config = setup_config_test();
    assert_eq!(config.timeout, 5);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_proxy() {
    let config = setup_config_test();
    assert_eq!(config.proxy, "http://127.0.0.1:8080");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_replay_proxy() {
    let config = setup_config_test();
    assert_eq!(config.replay_proxy, "http://127.0.0.1:8081");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_silent() {
    let config = setup_config_test();
    assert!(config.silent);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_force_recursion() {
    let config = setup_config_test();
    assert!(config.force_recursion);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_quiet() {
    let config = setup_config_test();
    assert!(config.quiet);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_json() {
    let config = setup_config_test();
    assert!(config.json);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_auto_bail() {
    let config = setup_config_test();
    assert!(config.auto_bail);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_auto_tune() {
    let config = setup_config_test();
    assert!(config.auto_tune);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_verbosity() {
    let config = setup_config_test();
    assert_eq!(config.verbosity, 1);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_limit_bars() {
    let config = setup_config_test();
    assert_eq!(config.limit_bars, 3);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_output() {
    let config = setup_config_test();
    assert_eq!(config.output, "/some/otherpath");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_redirects() {
    let config = setup_config_test();
    assert!(config.redirects);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_insecure() {
    let config = setup_config_test();
    assert!(config.insecure);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_no_recursion() {
    let config = setup_config_test();
    assert!(config.no_recursion);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_stdin() {
    let config = setup_config_test();
    assert!(config.stdin);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_dont_filter() {
    let config = setup_config_test();
    assert!(config.dont_filter);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_add_slash() {
    let config = setup_config_test();
    assert!(config.add_slash);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_extract_links() {
    let config = setup_config_test();
    assert!(!config.extract_links);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_collect_extensions() {
    let config = setup_config_test();
    assert!(config.collect_extensions);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_collect_backups() {
    let config = setup_config_test();
    assert!(config.collect_backups);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_collect_words() {
    let config = setup_config_test();
    assert!(config.collect_words);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_extensions() {
    let config = setup_config_test();
    assert_eq!(config.extensions, vec!["html", "php", "js"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_dont_collect() {
    let config = setup_config_test();
    assert_eq!(config.dont_collect, vec!["png", "gif", "jpg", "jpeg"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_methods() {
    let config = setup_config_test();
    assert_eq!(config.methods, vec!["GET", "PUT", "DELETE"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_data() {
    let config = setup_config_test();
    assert_eq!(config.data, vec![31, 32, 33, 34]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_regex_denylist() {
    let config = setup_config_test();
    assert_eq!(
        config.regex_denylist[0].as_str(),
        Regex::new("/deny.*").unwrap().as_str()
    );
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_url_denylist() {
    let config = setup_config_test();
    assert_eq!(
        config.url_denylist,
        vec![
            Url::parse("http://dont-scan.me").unwrap(),
            Url::parse("https://also-not.me").unwrap(),
        ]
    );
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_scope() {
    let config = setup_config_test();
    assert_eq!(
        config.scope,
        vec![
            Url::parse("http://example.com").unwrap(),
            Url::parse("https://other.com").unwrap(),
        ]
    );
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_regex() {
    let config = setup_config_test();
    assert_eq!(config.filter_regex, vec!["^ignore me$"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_similar() {
    let config = setup_config_test();
    assert_eq!(config.filter_similar, vec!["https://somesite.com/soft404"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_size() {
    let config = setup_config_test();
    assert_eq!(config.filter_size, vec![4120]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_word_count() {
    let config = setup_config_test();
    assert_eq!(config.filter_word_count, vec![994, 992]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_line_count() {
    let config = setup_config_test();
    assert_eq!(config.filter_line_count, vec![34]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_filter_status() {
    let config = setup_config_test();
    assert_eq!(config.filter_status, vec![201]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_save_state() {
    let config = setup_config_test();
    assert!(!config.save_state);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_time_limit() {
    let config = setup_config_test();
    assert_eq!(config.time_limit, "10m");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_scan_dir_listings() {
    let config = setup_config_test();
    assert!(config.scan_dir_listings);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_protocol() {
    let config = setup_config_test();
    assert_eq!(config.protocol, "http");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_request_file() {
    let config = setup_config_test();
    assert_eq!(config.request_file, String::new());
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_resume_from() {
    let config = setup_config_test();
    assert_eq!(config.resume_from, "/some/state/file");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_server_certs() {
    let config = setup_config_test();
    assert_eq!(
        config.server_certs,
        ["/some/cert.pem", "/some/other/cert.pem"]
    );
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_backup_extensions() {
    let config = setup_config_test();
    assert_eq!(config.backup_extensions, [".save"]);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_client_cert() {
    let config = setup_config_test();
    assert_eq!(config.client_cert, "/some/client/cert.pem");
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_client_key() {
    let config = setup_config_test();
    assert_eq!(config.client_key, "/some/client/key.pem");
}

#[test]
/// parse the test config and see that the values parsed are correct
fn config_reads_headers() {
    let config = setup_config_test();
    let mut headers = HashMap::new();
    headers.insert("stuff".to_string(), "things".to_string());
    headers.insert("mostuff".to_string(), "mothings".to_string());
    assert_eq!(config.headers, headers);
}

#[test]
/// parse the test config and see that the values parsed are correct
fn config_reads_queries() {
    let config = setup_config_test();
    let queries = vec![
        ("name".to_string(), "value".to_string()),
        ("rick".to_string(), "astley".to_string()),
    ];
    assert_eq!(config.queries, queries);
}

#[test]
fn config_default_not_random_agent() {
    let config = setup_config_test();
    assert!(!config.random_agent);
}

#[test]
#[should_panic]
/// test that an error message is printed and panic is called when report_and_exit is called
fn config_report_and_exit_works() {
    report_and_exit("some message");
}

#[test]
/// test as_str method of Configuration
fn as_str_returns_string_with_newline() {
    let config = Configuration::new().unwrap();
    let config_str = config.as_str();
    println!("{config_str}");
    assert!(config_str.starts_with("Configuration {"));
    assert!(config_str.ends_with("}\n"));
    assert!(config_str.contains("replay_codes:"));
    assert!(config_str.contains("client: Client {"));
    assert!(config_str.contains("user_agent: \"feroxbuster"));
}

#[test]
/// test as_json method of Configuration
fn as_json_returns_json_representation_of_configuration_with_newline() {
    let mut config = Configuration::new().unwrap();
    config.timeout = 12;
    config.depth = 2;
    let config_str = config.as_json().unwrap();
    let json: Configuration = serde_json::from_str(&config_str).unwrap();
    assert_eq!(json.config, config.config);
    assert_eq!(json.wordlist, config.wordlist);
    assert_eq!(json.replay_codes, config.replay_codes);
    assert_eq!(json.timeout, config.timeout);
    assert_eq!(json.depth, config.depth);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_unique() {
    let config = setup_config_test();
    assert!(config.unique);
}

#[test]
/// parse the test config and see that the value parsed is correct
fn config_reads_response_size_limit() {
    let config = setup_config_test();
    assert_eq!(config.response_size_limit, 8388608); // 8MB as set in setup_config_test
}

// ---------------------------------------------------------------------------
// ML layer — config-file (TOML) reading. Mirrors the per-field convention
// above; values come from the fixture in `setup_config_test`.
// ---------------------------------------------------------------------------

#[test]
/// --ml enabled via config file
fn config_reads_ml() {
    let config = setup_config_test();
    assert!(config.ml);
}

#[test]
/// --ml-loop enabled via config file
fn config_reads_ml_loop() {
    let config = setup_config_test();
    assert!(config.ml_loop);
}

#[test]
/// --ml-list-dir read from config file
fn config_reads_ml_list_dir() {
    let config = setup_config_test();
    assert_eq!(config.ml_list_dir, "/some/wordlists");
}

#[test]
/// --ml-list-chunk read from config file
fn config_reads_ml_list_chunk() {
    let config = setup_config_test();
    assert_eq!(config.ml_list_chunk, 300);
}

#[test]
/// --ml-list-max read from config file
fn config_reads_ml_list_max() {
    let config = setup_config_test();
    assert_eq!(config.ml_list_max, 5000);
}

#[test]
/// --ml-algo read from config file
fn config_reads_ml_algo() {
    let config = setup_config_test();
    assert_eq!(config.ml_algo, "dynsdt");
}

#[test]
/// --ml-model read from config file
fn config_reads_ml_model() {
    let config = setup_config_test();
    assert_eq!(config.ml_model, "/some/model.json");
}

#[test]
/// --ml-order read from config file
fn config_reads_ml_order() {
    let config = setup_config_test();
    assert_eq!(config.ml_order, 5);
}

#[test]
/// --ml-predictions read from config file
fn config_reads_ml_predictions() {
    let config = setup_config_test();
    assert_eq!(config.ml_predictions, 40);
}

#[test]
/// ml_rank = false read from config file (BM25 re-ranking disabled)
fn config_reads_ml_rank() {
    let config = setup_config_test();
    assert!(!config.ml_rank);
}

#[test]
/// --ml-scheduler read from config file
fn config_reads_ml_scheduler() {
    let config = setup_config_test();
    assert_eq!(config.ml_scheduler, "ucb1");
}

// ---------------------------------------------------------------------------
// ML layer — command-line parsing. Exercises `parse_cli_args` directly so the
// CLI-only glue (flag presence, --ml-model implying --ml, --no-ml-rank) is
// covered, not just TOML deserialization.
// ---------------------------------------------------------------------------

/// Build a `Configuration` from argv exactly as the CLI would, via the real
/// clap parser + `parse_cli_args`. `--stdin` stands in for a target source.
fn cli_config(extra: &[&str]) -> Configuration {
    let mut argv = vec!["feroxbuster", "--stdin"];
    argv.extend_from_slice(extra);
    let matches = crate::parser::initialize()
        .try_get_matches_from(argv)
        .expect("args should parse");
    Configuration::parse_cli_args(&matches)
}

#[test]
/// with no ML flags, every ML field keeps its default and the layer is off
fn cli_ml_defaults_when_absent() {
    let config = cli_config(&[]);
    assert!(!config.ml);
    assert!(!config.ml_loop);
    assert!(config.ml_list_dir.is_empty());
    assert!(config.ml_model.is_empty());
    assert_eq!(config.ml_list_chunk, 200);
    assert_eq!(config.ml_list_max, 0);
    assert_eq!(config.ml_algo, "markov");
    assert_eq!(config.ml_order, 3);
    assert_eq!(config.ml_predictions, 25);
    assert_eq!(config.ml_scheduler, "thompson");
    assert!(config.ml_rank);
}

#[test]
/// --ml turns the layer on (but not the loop)
fn cli_ml_flag_sets_ml() {
    let config = cli_config(&["--ml"]);
    assert!(config.ml);
    assert!(!config.ml_loop);
}

#[test]
/// --ml-loop sets ml_loop
fn cli_ml_loop_sets_flag() {
    let config = cli_config(&["--ml-loop"]);
    assert!(config.ml_loop);
}

#[test]
/// --ml-model sets the path AND implies --ml even without --ml present
fn cli_ml_model_implies_ml() {
    let config = cli_config(&["--ml-model", "/tmp/model.json"]);
    assert_eq!(config.ml_model, "/tmp/model.json");
    assert!(config.ml, "--ml-model must imply --ml");
}

#[test]
/// --ml-list-dir is captured
fn cli_ml_list_dir() {
    let config = cli_config(&["--ml-list-dir", "/usr/share/seclists"]);
    assert_eq!(config.ml_list_dir, "/usr/share/seclists");
}

#[test]
/// --ml-list-chunk overrides the default of 200
fn cli_ml_list_chunk() {
    let config = cli_config(&["--ml-list-chunk", "500"]);
    assert_eq!(config.ml_list_chunk, 500);
}

#[test]
/// --ml-list-max overrides the default, and 0 is preserved as "unlimited"
fn cli_ml_list_max() {
    assert_eq!(cli_config(&["--ml-list-max", "1000"]).ml_list_max, 1000);
    assert_eq!(cli_config(&["--ml-list-max", "0"]).ml_list_max, 0);
}

#[test]
/// every accepted --ml-algo value round-trips into the config
fn cli_ml_algo_each_value() {
    for algo in ["auto", "markov", "trie", "dynsdt", "tst"] {
        assert_eq!(cli_config(&["--ml-algo", algo]).ml_algo, algo);
    }
}

#[test]
/// --ml-order overrides the Markov order default of 3
fn cli_ml_order() {
    assert_eq!(cli_config(&["--ml-order", "5"]).ml_order, 5);
}

#[test]
/// --ml-predictions overrides the default of 25
fn cli_ml_predictions() {
    assert_eq!(cli_config(&["--ml-predictions", "50"]).ml_predictions, 50);
}

#[test]
/// every accepted --ml-scheduler value round-trips into the config
fn cli_ml_scheduler_each_value() {
    for sched in ["thompson", "ucb1", "round_robin"] {
        assert_eq!(cli_config(&["--ml-scheduler", sched]).ml_scheduler, sched);
    }
}

#[test]
/// --no-ml-rank disables BM25 re-ranking; absent it stays on
fn cli_no_ml_rank_disables_rank() {
    assert!(cli_config(&[]).ml_rank);
    assert!(!cli_config(&["--no-ml-rank"]).ml_rank);
}
