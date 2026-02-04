/// The code will create cache files in your temporary directory and provides some basic operational methods. 
/// You can comment out lines 31, 32 and 33 to prevent the deletion of the cache.

use cache_lite::{Cache, CacheConfig, CacheObject};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let style: CacheConfig = CacheConfig::new(r#"
{
    "path": {
        "windows": "%temp%/Rust/Cache",
        "linux": "/tmp/Rust/Cache"
    },
    "format": {
        "filename": "{name}.{id}.{time}.cache",
        "time": "%H.%M.%S"
    }
}"#);

    let mut cache: Cache = Cache::new(style);
    let cache1: CacheObject = cache.create("hello_rust_cache", None);
    println!("Cache1 Name: {}", cache1.name());
    println!("Cache1 Path: {}", cache1.path().display());

    println!("Input a text: ");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    cache1.write_string(&input)?;

    println!("Cache1 Content: {}", cache1.get_string()?);

    std::thread::sleep(std::time::Duration::from_secs(5));
    cache1.delete()?;
    println!("Cache1 is deleted");
    Ok(())
}