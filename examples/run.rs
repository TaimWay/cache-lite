use std::error::Error;

use cache_lite::{
    Cache, CacheConfig, CacheObject, CacheResult
};

fn save(cache: &mut Cache, strings: &str) -> CacheResult<CacheObject> {
    Ok(cache.create(strings, None)?)
}

fn read(obj: &CacheObject) -> CacheResult<String> {
    Ok(obj.get_string()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = CacheConfig::new(r#"
{
    "path": {
        "windows": "./cache",
        "linux": "./cache"
    },
    "format": {
        "filename": "{id}.{name}.{time}.cache",
        "time": "%Y-%m-%d+%H.%M.%S"
    }
}
"#)?;

    let mut cache = Cache::new(config)?;
    let version_cache = save(&mut cache, "version")?;
    version_cache.write_string("Hello, world!")?;
    println!("Content: {}", read(&version_cache)?);

    Ok(())
}