//! Times a cold (miss) vs warm (cache hit) load of a wordlist directory tree.
//! Usage: cargo run -p ferox-ml-core --example listcache_bench [DIR]
use std::time::Instant;
use ferox_ml_core::wordlist::load_list_dir_cached;

fn main() {
    let dir = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/usr/share/seclists/Discovery/Web-Content".to_string());
    let cache = std::env::temp_dir().join("feroxml-listcache-bench");
    let _ = std::fs::remove_dir_all(&cache); // force a cold first run
    let _ = std::fs::create_dir_all(&cache);
    let cd = cache.to_string_lossy().into_owned();

    println!("dir: {dir}");
    for i in 1..=2 {
        let t = Instant::now();
        let (pool, hit) = load_list_dir_cached(&dir, 0, &cd).unwrap();
        println!(
            "run {i}: {:>8} entries  from_cache={:<5}  {:.2?}",
            pool.len(),
            hit,
            t.elapsed()
        );
    }
}
