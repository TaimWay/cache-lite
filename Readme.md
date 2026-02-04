# Cache Lite

A lightweight, cross-platform caching library for Rust with configurable storage and file formatting.

## Features

- **Cross-platform Support**: Works seamlessly on Windows and Linux with platform-specific path configurations
- **Configurable Storage**: Customize cache paths, file naming formats
- **Simple API**: Intuitive interface for creating, reading, writing, and deleting cache objects
- **Environment Variable Expansion**: Automatic expansion of system paths and home directories
- **Time-based File Naming**: Flexible timestamp formatting in cache filenames

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rust-cache = "0.1.0"
```

## Quick Start

```rust
use rust_cache::{Cache, CacheConfig};

fn main() -> std::io::Result<()> {
    // Create cache with default configuration
    let config = CacheConfig::default();
    let mut cache = Cache::new(config);
    
    // Create a cache object
    let cache_obj = cache.create("my_data", None);
    
    // Write data to cache
    cache_obj.write_string("Hello, cached world!")?;
    
    // Read data from cache
    let content = cache_obj.get_string()?;
    println!("Cached content: {}", content);
    
    // Retrieve cache object by name
    let retrieved = cache.get("my_data")?;
    println!("Cache path: {:?}", retrieved.path());
    
    Ok(())
}
```

## Configuration

### Default Configuration

The library comes with sensible defaults:

```json
{
  "path": {
    "windows": "%temp%/Rust/Cache",
    "linux": "/tmp/Rust/Cache"
  },
  "format": {
    "filename": "r{name}.{time}.cache",
    "time": "%Y+%m+%d-%H+%M+%S"
  }
}
```

### Custom Configuration

Create a custom configuration from JSON:

```rust
let json_config = r#"
{
  "path": {
    "windows": "%appdata%/MyApp/Cache",
    "linux": "~/.myapp/cache"
  },
  "format": {
    "filename": "{name}_{id}.cache",
    "time": "%Y-%m-%d"
  }
}"#;

let config = CacheConfig::new(json_config);
let cache = Cache::new(config);
```

### Per-Object Custom Configuration

Override configuration for individual cache objects:

```rust
let custom_config = r#"{"filename": "custom_{name}.cache"}"#;
let cache_obj = cache.create("special_data", Some(custom_config));
```

## API Reference

### Cache Manager

The main `Cache` struct manages multiple cache objects:

```rust
impl Cache {
    pub fn new(config: CacheConfig) -> Self;
    pub fn create(&mut self, name: &str, custom_config: Option<&str>) -> CacheObject;
    pub fn get(&self, name: &str) -> io::Result<CacheObject>;
    pub fn remove(&mut self, name: &str) -> io::Result<()>;
    pub fn clear(&mut self) -> io::Result<()>;
    pub fn len(&self) -> io::Result<u32>;
    pub fn iter(&self) -> impl Iterator<Item = &CacheObject>;
    pub fn set_config(&mut self, config: CacheConfig);
    pub fn get_config(&self) -> CacheConfig;
}
```

### Cache Object

Individual cache objects with file operations:

```rust
impl CacheObject {
    pub fn name(&self) -> &str;
    pub fn path(&self) -> &Path;
    pub fn created_at(&self) -> SystemTime;
    pub fn id(&self) -> u32;
    pub fn get_file(&self) -> io::Result<std::fs::File>;
    pub fn get_string(&self) -> io::Result<String>;
    pub fn write_string(&self, content: &str) -> io::Result<()>;
    pub fn delete(&self) -> io::Result<()>;
}
```

## Environment Variables

### Windows

The library automatically expands these environment variables in paths:

- `%temp%`, `%tmp%` - Temporary directory
- `%appdata%` - Application data directory
- `%localappdata%` - Local application data directory
- `%userprofile%` - User profile directory

### Linux/Unix

- `~` expands to the user's home directory

## File Naming Format

The filename format supports these placeholders:

| Placeholder | Description                     | Example               |
|-------------|---------------------------------|-----------------------|
| `{name}`    | Cache object name               | `my_data`             |
| `{id}`      | Unique numeric ID               | `1`                   |
| `{time}`    | Formatted timestamp             | `2026+02+04-14+30+00` |

## Examples

### Advanced Usage

```rust
use rust_cache::{Cache, CacheConfig, CacheObject};
use std::io;

fn complex_example() -> io::Result<()> {
    // Create custom configuration
    let config = CacheConfig::default();
    let mut cache = Cache::new(config);
    
    // Create multiple cache objects
    let data_cache = cache.create("data", None);
    let config_cache = cache.create("config", None);
    
    // Write different data types
    data_cache.write_string("Important data here")?;
    config_cache.write_string(r#"{"setting": "value"}"#)?;
    
    // Iterate through all cache objects
    for obj in cache.iter() {
        println!("Cache: {} at {:?}", obj.name(), obj.path());
    }
    
    // Get cache statistics
    println!("Total cache objects: {}", cache.len()?);
    
    // Clean up
    cache.remove("data")?;
    
    Ok(())
}
```

## Platform-Specific Behavior

### Windows

- Uses Windows environment variable syntax (`%VAR%`)
- Paths use backslashes by default
- Supports Windows-specific directories

### Linux/Unix

- Uses forward slashes for paths
- Supports tilde expansion for home directory
- Follows Unix filesystem conventions

## Building from Source

```bash
# Clone the repository
git clone https://github.com/TaimWay/rust-cache.git
cd rust-cache

# Build the library
cargo build --release

# Run tests
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

For issues, questions, or suggestions:

- Open an issue on [GitHub](https://github.com/TaimWay/rust-cache/issues)
- Contact: TaimWay <taimway@gmail.com>
