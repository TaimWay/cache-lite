use cache_lite::{Cache, CacheConfig, CacheObject};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cache: Cache = Cache::new(CacheConfig::new(r#"
{
    "path": {
        "windows": "C:/RustCache",
        "linux": "/cache"
    },
    "format": {
        "filename": "{name}.{id}.{time}.cache",
        "time": "%H.%M.%S"
    }
}"#));
    
    let cache1: CacheObject = cache.create("hello_rust_cache", None);
    println!("Cache1 Name: {}", cache1.name());
    println!("Cache1 Path: {}", cache1.path().display());

    println!("Input a text: ");
    let mut input: String = String::new();
    std::io::stdin().read_line(&mut input)?;
    
    let input = input.trim();
    cache1.write_string(input)?;

    println!("Cache1 Content: {}", cache1.get_string()?);

    std::thread::sleep(std::time::Duration::from_secs(2));
    cache1.delete()?;
    println!("Cache1 is deleted");
    
    Ok(())
}