/*
 * @filename: lib.rs
 * @description: A cross-platform caching library for Rust with configurable storage, lifecycle, and file formatting.
 * @author: TaimWay <taimway@gmail.com>
 * 
 * Copyright (C) 2026 TaimWay
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! # Rust Cache Library
//! 
//! A lightweight, cross-platform caching library for Rust applications.
//! Provides configurable cache storage, lifecycle management, and file formatting.
//! 
//! # Features
//! 
//! - **Cross-platform**: Automatic path handling for Windows and Linux
//! - **Configurable**: JSON-based configuration with runtime overrides
//! - **Simple API**: Easy-to-use interface for cache operations
//! - **File-based**: Persistent cache storage on disk
//! 
//! # Quick Start
//! 
//! ```no_run
//! use cache_lite::{Cache, CacheConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create cache with default configuration
//!     let config = CacheConfig::default();
//!     let mut cache = Cache::new(config)?;
//!
//!     // Create a new cache object
//!     let cache_obj = cache.create("my_cache", None).unwrap();
//!
//!     // Write data to cache
//!     cache_obj.write_string("Cached data").unwrap();
//!
//!     // Read data from cache
//!     let data = cache_obj.get_string().unwrap();
//!     assert_eq!(data, "Cached data");
//!
//!     cache_obj.delete()?;
//!
//!     Ok(())
//! }
//! ```
//! 
//! # Configuration
//! 
//! The library supports JSON configuration for customizing cache behavior:
//! 
//! ```json
//! {
//!   "path": {
//!     "windows": "%temp%/MyApp/Cache",
//!     "linux": "/tmp/myapp/cache"
//!   },
//!   "format": {
//!     "filename": "{name}_{time}.cache",
//!     "time": "%Y%m%d_%H%M%S"
//!   }
//! }
//! ```
//! 
//! # Error Handling
//! 
//! The library provides a comprehensive error type `CacheError` for handling
//! various failure scenarios including I/O errors, invalid configurations,
//! permission issues, and more.

mod config;
mod object;
mod cache;
mod error;
mod utils;

// Re-export public API
pub use config::{CacheConfig, CachePathConfig, CacheFormatConfig};
pub use object::CacheObject;
pub use cache::Cache;
pub use error::CacheError;

/// Result type alias for cache operations
pub type CacheResult<T> = std::result::Result<T, CacheError>;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        assert_eq!(config.max_size, 0);
        assert_eq!(config.max_files, 0);
        assert!(!config.path.windows.is_empty());
        assert!(!config.path.linux.is_empty());
        assert!(!config.format.filename.is_empty());
        assert!(!config.format.time.is_empty());
    }

    #[test]
    fn test_cache_config_from_json() {
        let json = r#"{
            "path": {
                "windows": "%temp%/TestCache",
                "linux": "/tmp/testcache"
            },
            "format": {
                "filename": "test_{name}.cache",
                "time": "%Y%m%d"
            },
            "max_size": 1024,
            "max_files": 10
        }"#;

        let config = CacheConfig::new(json).unwrap();
        assert_eq!(config.path.windows, "%temp%/TestCache");
        assert_eq!(config.path.linux, "/tmp/testcache");
        assert_eq!(config.format.filename, "test_{name}.cache");
        assert_eq!(config.format.time, "%Y%m%d");
        assert_eq!(config.max_size, 1024);
        assert_eq!(config.max_files, 10);
    }

    #[test]
    fn test_cache_config_from_partial_json() {
        let json = r#"{
            "path": {
                "linux": "/custom/path"
            },
            "format": {
                "filename": "custom_{name}.cache"
            }
        }"#;

        let config = CacheConfig::new(json).unwrap();
        assert_eq!(config.path.linux, "/custom/path");
        assert_eq!(config.format.filename, "custom_{name}.cache");
        // Windows path should use default
        assert!(!config.path.windows.is_empty());
        // Time format should use default
        assert!(!config.format.time.is_empty());
    }

    #[test]
    fn test_cache_config_new_or_default() {
        let json = "invalid json";
        let config = CacheConfig::new_or_default(json);
        // Should fall back to default
        assert_eq!(config.max_size, 0);
        assert_eq!(config.max_files, 0);
    }

    #[test]
    fn test_cache_creation() {
        let temp_dir = tempdir().unwrap();
        let config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let config = CacheConfig::new(&config_json).unwrap();
        let mut cache = Cache::new(config).unwrap();

        // Test create cache object
        let cache_obj = cache.create("test_cache", None).unwrap();
        assert_eq!(cache_obj.name(), "test_cache");
        
        // Write some data to ensure file exists
        cache_obj.write_string("test data").unwrap();
        assert!(cache_obj.exists());

        // Test duplicate creation fails
        let result = cache.create("test_cache", None);
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, CacheError::AlreadyExists(_)));
        }

        // Test get cache object
        let retrieved = cache.get("test_cache").unwrap();
        assert_eq!(retrieved.name(), "test_cache");
        assert_eq!(retrieved.id(), cache_obj.id());

        // Test get non-existent cache
        let result = cache.get("nonexistent");
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(matches!(e, CacheError::NotFound(_)));
        }

        // Test length and empty
        assert_eq!(cache.len(), 1);
        assert!(!cache.is_empty());

        // Test iterator
        let objects: Vec<_> = cache.iter().collect();
        assert_eq!(objects.len(), 1);
        assert_eq!(objects[0].name(), "test_cache");
    }

    #[test]
    fn test_cache_operations() {
        let temp_dir = tempdir().unwrap();
        let config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let config = CacheConfig::new(&config_json).unwrap();
        let mut cache = Cache::new(config).unwrap();

        // Create multiple cache objects
        cache.create("cache1", None).unwrap();
        cache.create("cache2", None).unwrap();
        cache.create("cache3", None).unwrap();

        assert_eq!(cache.len(), 3);

        // Test remove operation
        cache.remove("cache2").unwrap();
        assert_eq!(cache.len(), 2);
        assert!(cache.get("cache2").is_err());

        // Test clear operation
        cache.clear().unwrap();
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }

    #[test]
    fn test_cache_object_operations() {
        let temp_dir = tempdir().unwrap();
        let config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let config = CacheConfig::new(&config_json).unwrap();
        let mut cache = Cache::new(config).unwrap();
        let cache_obj = cache.create("test_operations", None).unwrap();

        // Test string operations
        let test_string = "Hello, World!";
        cache_obj.write_string(test_string).unwrap();

        let read_string = cache_obj.get_string().unwrap();
        assert_eq!(read_string, test_string);

        // Test binary operations
        let test_bytes = vec![1, 2, 3, 4, 5];
        cache_obj.write_bytes(&test_bytes).unwrap();

        let read_bytes = cache_obj.get_bytes().unwrap();
        assert_eq!(read_bytes, test_bytes);

        // Test file operations
        let file = cache_obj.get_file().unwrap();
        assert!(file.metadata().is_ok());

        // Test size
        let size = cache_obj.size().unwrap();
        assert!(size > 0);

        // Test delete
        cache_obj.delete().unwrap();
        assert!(!cache_obj.exists());

        // Test creation time
        let new_obj = cache.create("new_cache", None).unwrap();
        let created_at = new_obj.created_at();
        assert!(created_at.elapsed().is_ok());
    }

    #[test]
    fn test_cache_with_custom_config() {
        let temp_dir = tempdir().unwrap();
        let base_config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "base_{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let base_config = CacheConfig::new(&base_config_json).unwrap();
        let mut cache = Cache::new(base_config).unwrap();

        let custom_config = r#"{
            "path": {
                "linux": "/custom/path"
            },
            "format": {
                "filename": "custom_{name}_{id}.cache"
            }
        }"#;

        let cache_obj = cache.create("custom_cache", Some(custom_config)).unwrap();
        let path_str = cache_obj.path().to_string_lossy().to_string();

        // Write data to ensure file exists
        cache_obj.write_string("test").unwrap();

        // Check that custom format is used
        assert!(path_str.contains("custom_cache"));
        assert!(path_str.contains(".cache"));
    }

    #[test]
    fn test_cache_config_updates() {
        let config = CacheConfig::default();
        let mut cache = Cache::new(config).unwrap();

        let new_config_json = r#"{
            "path": {
                "windows": "%temp%/UpdatedCache",
                "linux": "/tmp/updatedcache"
            },
            "format": {
                "filename": "updated_{name}.cache",
                "time": "%H%M%S"
            },
            "max_size": 2048,
            "max_files": 20
        }"#;

        let new_config = CacheConfig::new(new_config_json).unwrap();
        cache.set_config(new_config.clone());

        let retrieved_config = cache.get_config();
        assert_eq!(retrieved_config.max_size, 2048);
        assert_eq!(retrieved_config.max_files, 20);
        assert_eq!(retrieved_config.format.filename, "updated_{name}.cache");
    }

    #[test]
    fn test_validate_name() {
        // Valid names
        assert!(crate::utils::validate_name("valid_name").is_ok());
        assert!(crate::utils::validate_name("valid123").is_ok());
        assert!(crate::utils::validate_name("a").is_ok());

        // Invalid names
        assert!(crate::utils::validate_name("").is_err());
        assert!(crate::utils::validate_name(&"a".repeat(256)).is_err());
        assert!(crate::utils::validate_name("test/name").is_err());
        assert!(crate::utils::validate_name("test\\name").is_err());
        assert!(crate::utils::validate_name("test..name").is_err());
        
        #[cfg(windows)]
        {
            assert!(crate::utils::validate_name("CON").is_err());
            assert!(crate::utils::validate_name("test:name").is_err());
            assert!(crate::utils::validate_name("test<name").is_err());
        }
    }

    #[test]
    fn test_error_handling() {
        // Test error creation
        let io_error = CacheError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert_eq!(io_error.kind(), "io");

        let generic_error = CacheError::new("Test error");
        assert_eq!(generic_error.kind(), "generic");
        assert_eq!(generic_error.message(), "Test error");

        // Test error conversions
        let io_err: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, "test");
        let cache_err: CacheError = io_err.into();
        assert!(cache_err.is_io_error());

        let json_err = serde_json::from_str::<CacheConfig>("invalid json");
        assert!(json_err.is_err());
    }

    #[test]
    fn test_cache_object_clone() {
        let temp_dir = tempdir().unwrap();
        let config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let config = CacheConfig::new(&config_json).unwrap();
        let mut cache = Cache::new(config).unwrap();
        let cache_obj = cache.create("clone_test", None).unwrap();

        // Write data first
        cache_obj.write_string("test data").unwrap();

        // Test cloning
        let cloned = cache_obj.clone();
        assert_eq!(cloned.name(), cache_obj.name());
        assert_eq!(cloned.id(), cache_obj.id());
        assert_eq!(cloned.path(), cache_obj.path());

        // Check clone sees the same content
        let cloned_content = cloned.get_string().unwrap();
        assert_eq!(cloned_content, "test data");
    }

    #[test]
    fn test_expand_path() {
        // Test tilde expansion
        let path_with_tilde = "~/test/path";
        let expanded = crate::utils::expand_path(path_with_tilde);
        if let Some(home) = dirs::home_dir() {
            let home_str = home.to_string_lossy();
            assert!(expanded.starts_with(&*home_str));
        }

        // Test Windows env var expansion (only on Windows)
        #[cfg(windows)]
        {
            let path_with_env = "%temp%/test";
            let expanded = crate::utils::expand_path(path_with_env);
            assert!(!expanded.contains("%temp%"));
        }

        // Test path separator conversion
        let unix_path = "path/to/file";
        let expanded = crate::utils::expand_path(unix_path);
        
        #[cfg(windows)]
        assert!(expanded.contains('\\'));
        
        #[cfg(unix)]
        assert!(expanded.contains('/'));
    }

    #[test]
    fn test_cache_clear_with_errors() {
        let temp_dir = tempdir().unwrap();
        let config_json = format!(
            r#"{{
                "path": {{
                    "windows": "{}",
                    "linux": "{}"
                }},
                "format": {{
                    "filename": "{{name}}.cache",
                    "time": "%Y%m%d"
                }},
                "max_size": 0,
                "max_files": 0
            }}"#,
            temp_dir.path().to_string_lossy(),
            temp_dir.path().to_string_lossy()
        );

        let config = CacheConfig::new(&config_json).unwrap();
        let mut cache = Cache::new(config).unwrap();

        // Create a cache object
        let cache_obj = cache.create("test_clear", None).unwrap();
        
        // Write data to ensure file exists
        cache_obj.write_string("test data").unwrap();
        assert!(cache_obj.exists());

        // Clear should work
        cache.clear().unwrap();
        
        // Cache should be empty
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
        
        // File should be deleted
        assert!(!cache_obj.exists());
    }

    #[test]
    fn test_error_matches() {
        // Test error matches
        let io_error = CacheError::Io(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        assert!(io_error.is_io_error());

        let not_found_error = CacheError::NotFound("test".to_string());
        assert!(not_found_error.is_not_found());

        let permission_error = CacheError::PermissionDenied("test".to_string());
        assert!(permission_error.is_permission_denied());
    }

    #[test]
    fn test_config_serde_roundtrip() {
        let config = CacheConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let parsed_config = CacheConfig::new(&json).unwrap();
        
        assert_eq!(config.max_size, parsed_config.max_size);
        assert_eq!(config.max_files, parsed_config.max_files);
        assert_eq!(config.path.windows, parsed_config.path.windows);
        assert_eq!(config.path.linux, parsed_config.path.linux);
        assert_eq!(config.format.filename, parsed_config.format.filename);
        assert_eq!(config.format.time, parsed_config.format.time);
    }
}
