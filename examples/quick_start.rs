use cache_lite::{Cache, CacheConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create cache with default configuration
    let config = CacheConfig::default();
    let mut cache = Cache::new(config)?;

    // Create a new cache object
    let cache_obj = cache.create("my_cache", None).unwrap();

    // Write data to cache
    cache_obj.write_string("Cached data").unwrap();

    // Read data from cache
    let data = cache_obj.get_string().unwrap();
    assert_eq!(data, "Cached data");

    cache_obj.delete()?;

    Ok(())
}
