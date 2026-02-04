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
//! // Create cache with default configuration
//! let config = CacheConfig::default();
//! let mut cache = Cache::new(config);
//! 
//! // Create a new cache object
//! let cache_obj = cache.create("my_cache", None).unwrap();
//! 
//! // Write data to cache
//! cache_obj.write_string("Cached data").unwrap();
//! 
//! // Read data from cache
//! let data = cache_obj.get_string().unwrap();
//! assert_eq!(data, "Cached data");
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
    use std::fs;
    use tempfile::tempdir;
    
    #[test]
    fn test_cache_config_default() {
        let config = CacheConfig::default();
        
        // Check default Windows path
        assert!(config.path.windows.contains("%temp%"));
        assert!(config.path.windows.contains("Rust/Cache"));
        
        // Check default Linux path
        assert_eq!(config.path.linux, "/tmp/Rust/Cache");
        
        // Check default filename format
        assert!(config.format.filename.contains("{name}"));
        assert!(config.format.filename.contains("{time}"));
        
        // Check default time format
        assert_eq!(config.format.time, "%Y+%m+%d-%H+%M+%S");
    }
    
    #[test]
    fn test_cache_config_from_json() {
        let json_config = r#"
        {
            "path": {
                "windows": "C:/Temp/Cache",
                "linux": "/var/cache"
            },
            "format": {
                "filename": "cache_{name}.dat",
                "time": "%Y%m%d"
            }
        }
        "#;
        
        let config = CacheConfig::new(json_config).expect("Failed to parse config");
        
        assert_eq!(config.path.windows, "C:/Temp/Cache");
        assert_eq!(config.path.linux, "/var/cache");
        assert_eq!(config.format.filename, "cache_{name}.dat");
        assert_eq!(config.format.time, "%Y%m%d");
    }
    
    #[test]
    fn test_cache_config_new_or_default() {
        // Test valid JSON
        let valid_json = r#"
        {
            "path": {
                "windows": "C:/Custom/Cache",
                "linux": "/custom/cache"
            },
            "format": {
                "filename": "custom_{name}.dat",
                "time": "%Y%m"
            }
        }
        "#;
        
        let config = CacheConfig::new_or_default(valid_json);
        assert_eq!(config.path.windows, "C:/Custom/Cache");
        assert_eq!(config.path.linux, "/custom/cache");
        assert_eq!(config.format.filename, "custom_{name}.dat");
        assert_eq!(config.format.time, "%Y%m");
        
        // Test invalid JSON - should fall back to default
        let invalid_json = "{invalid json";
        let config = CacheConfig::new_or_default(invalid_json);
        assert!(config.path.windows.contains("%temp%"));
        assert_eq!(config.path.linux, "/tmp/Rust/Cache");
    }
    
    #[test]
    fn test_cache_config_invalid_json() {
        let invalid_json = "{invalid json";
        let result = CacheConfig::new(invalid_json);
        
        // Should return error
        assert!(result.is_err());
        
        // Check error type
        let error = result.err().unwrap();
        assert_eq!(error.kind(), "config_parse");
        assert!(error.message().contains("Failed to parse config"));
    }
    
    #[test]
    fn test_cache_object_creation_and_properties() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let test_path = temp_dir.path().join("test_cache.cache");
        
        let cache_object = CacheObject::new(
            "test_object".to_string(),
            test_path.clone(),
            1
        );
        
        assert_eq!(cache_object.name(), "test_object");
        assert_eq!(cache_object.id(), 1);
        assert_eq!(cache_object.path(), test_path);
        assert!(cache_object.created_at().elapsed().is_ok());
    }
    
    #[test]
    fn test_cache_object_file_operations() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let test_path = temp_dir.path().join("test_operations.cache");
        
        let cache_object = CacheObject::new(
            "test_ops".to_string(),
            test_path.clone(),
            2
        );
        
        // Test write operation
        let content = "Hello, Cache!";
        cache_object.write_string(content).expect("Failed to write to cache");
        
        // Verify file was created
        assert!(test_path.exists());
        assert!(test_path.is_file());
        
        // Test read operation
        let read_content = cache_object.get_string().expect("Failed to read from cache");
        assert_eq!(read_content, content);
        
        // Test file handle retrieval
        let file = cache_object.get_file().expect("Failed to open cache file");
        assert!(file.metadata().is_ok());
        
        // Test delete operation
        cache_object.delete().expect("Failed to delete cache file");
        assert!(!test_path.exists());
    }
    
    #[test]
    fn test_cache_object_advanced_operations() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let test_path = temp_dir.path().join("test_advanced.cache");
        
        let cache_object = CacheObject::new(
            "test_advanced".to_string(),
            test_path.clone(),
            1
        );
        
        // Test writing and reading bytes
        let bytes = vec![1, 2, 3, 4, 5];
        cache_object.write_bytes(&bytes).expect("Failed to write bytes");
        
        let read_bytes = cache_object.get_bytes().expect("Failed to read bytes");
        assert_eq!(read_bytes, bytes);
        
        // Test file size
        let size = cache_object.size().expect("Failed to get file size");
        assert_eq!(size, 5);
        
        // Test file existence
        assert!(cache_object.exists());
        
        // Test delete
        cache_object.delete().expect("Failed to delete");
        assert!(!cache_object.exists());
        
        // Test getting size of non-existent file
        let result = cache_object.size();
        assert!(result.is_err());
    }
    
    #[test]
    fn test_cache_object_clone() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let test_path = temp_dir.path().join("test_clone.cache");
        
        let original = CacheObject::new(
            "original".to_string(),
            test_path.clone(),
            3
        );
        
        let cloned = original.clone();
        
        assert_eq!(original.name(), cloned.name());
        assert_eq!(original.id(), cloned.id());
        assert_eq!(original.path(), cloned.path());
        
        // Cloned object should have same properties but be a separate instance
        assert_eq!(original.name(), "original");
        assert_eq!(cloned.name(), "original");
    }
    
    #[test]
    fn test_cache_management() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        // Create custom configuration for testing
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#, 
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let mut cache = Cache::new(config);
        
        // Test creating cache objects
        let cache_obj1 = cache.create("test_cache_1", None)
            .expect("Failed to create cache object 1");
        assert_eq!(cache_obj1.name(), "test_cache_1");
        assert_eq!(cache_obj1.id(), 1);
        
        let cache_obj2 = cache.create("test_cache_2", None)
            .expect("Failed to create cache object 2");
        assert_eq!(cache_obj2.name(), "test_cache_2");
        assert_eq!(cache_obj2.id(), 2);
        
        // Test cache size
        assert_eq!(cache.len(), 2);
        assert!(!cache.is_empty());
        
        // Test retrieving cache objects
        let retrieved1 = cache.get("test_cache_1")
            .expect("Failed to get cache object 1");
        assert_eq!(retrieved1.name(), "test_cache_1");
        
        let retrieved2 = cache.get("test_cache_2")
            .expect("Failed to get cache object 2");
        assert_eq!(retrieved2.name(), "test_cache_2");
        
        // Test removing a cache object
        cache.remove("test_cache_1").expect("Failed to remove cache object");
        assert_eq!(cache.len(), 1);
        
        // Verify removed object can't be retrieved
        assert!(cache.get("test_cache_1").is_err());
        
        // Verify remaining object is still accessible
        assert!(cache.get("test_cache_2").is_ok());
        
        // Test clearing all cache objects
        cache.clear().expect("Failed to clear cache");
        assert_eq!(cache.len(), 0);
        assert!(cache.is_empty());
    }
    
    #[test]
    fn test_cache_management_with_error_handling() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        // Create custom configuration for testing
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#, 
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let mut cache = Cache::new(config);
        
        // Test creating cache objects with invalid names
        let result = cache.create("", None);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), "invalid_name");
        
        // Test creating valid cache object
        let cache_obj = cache.create("test_cache", None)
            .expect("Failed to create cache object");
        
        // Test writing and reading with error handling
        let write_result = cache_obj.write_string("Test content");
        assert!(write_result.is_ok());
        
        let read_result = cache_obj.get_string();
        assert!(read_result.is_ok());
        assert_eq!(read_result.unwrap(), "Test content");
        
        // Test getting non-existent cache
        let result = cache.get("non_existent");
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), "not_found");
    }
    
    #[test]
    fn test_cache_invalid_names() {
        let config = CacheConfig::default();
        let mut cache = Cache::new(config);
        
        // Test with empty name
        let result = cache.create("", None);
        assert!(result.is_err(), "Should reject empty names");
        
        // Test with path traversal attempt
        let result = cache.create("../evil", None);
        assert!(result.is_err(), "Should reject path traversal names");
        
        // Test with slash in name
        let result = cache.create("path/to/cache", None);
        assert!(result.is_err(), "Should reject names with slashes");
        
        // Test with backslash in name (Windows)
        let result = cache.create("path\\to\\cache", None);
        assert!(result.is_err(), "Should reject names with backslashes");
        
        // Test with valid name
        let result = cache.create("valid_cache_name", None);
        assert!(result.is_ok(), "Should accept valid cache names");
    }
    
    #[test]
    fn test_cache_custom_configuration() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        // Base configuration
        let base_config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "default_{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let base_config = CacheConfig::new(&base_config_json).expect("Failed to parse config");
        let mut cache = Cache::new(base_config);
        
        // Custom configuration to override filename format
        let custom_config = r#"
        {
            "path": {
                "windows": "",
                "linux": ""
            },
            "format": {
                "filename": "custom_{name}_{id}.dat",
                "time": "%H%M%S"
            }
        }
        "#;
        
        let cache_obj = cache.create("custom_cache", Some(custom_config))
            .expect("Failed to create cache with custom config");
        
        let path_str = cache_obj.path().to_string_lossy();
        
        // Should use custom filename format
        assert!(path_str.contains("custom_custom_cache_"), 
                "Path should contain custom pattern: {}", path_str);
        assert!(path_str.ends_with(".dat"), 
                "Path should end with .dat: {}", path_str);
    }
    
    #[test]
    fn test_cache_iterator() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let mut cache = Cache::new(config);
        
        // Create multiple cache objects
        let _ = cache.create("cache_a", None).expect("Failed to create cache_a");
        let _ = cache.create("cache_b", None).expect("Failed to create cache_b");
        let _ = cache.create("cache_c", None).expect("Failed to create cache_c");
        
        // Test iterator collects all names
        let names: Vec<String> = cache.iter()
            .map(|obj| obj.name().to_string())
            .collect();
        
        assert_eq!(names.len(), 3, "Should have 3 cache objects");
        assert!(names.contains(&"cache_a".to_string()));
        assert!(names.contains(&"cache_b".to_string()));
        assert!(names.contains(&"cache_c".to_string()));
        
        // Test iterator order (should be arbitrary for HashMap)
        let ids: Vec<u32> = cache.iter()
            .map(|obj| obj.id())
            .collect();
        
        assert_eq!(ids.len(), 3);
        assert!(ids.contains(&1));
        assert!(ids.contains(&2));
        assert!(ids.contains(&3));
    }
    
    #[test]
    fn test_cache_configuration_get_set() {
        let mut config = CacheConfig::default();
        
        // Modify configuration
        config.path.windows = "C:/Custom/Path".to_string();
        config.path.linux = "/custom/path".to_string();
        config.format.filename = "custom_{name}.dat".to_string();
        config.format.time = "%Y".to_string();
        
        let mut cache = Cache::new(config.clone());
        
        // Verify initial configuration
        let retrieved_config = cache.get_config();
        assert_eq!(retrieved_config.path.windows, "C:/Custom/Path");
        assert_eq!(retrieved_config.path.linux, "/custom/path");
        assert_eq!(retrieved_config.format.filename, "custom_{name}.dat");
        assert_eq!(retrieved_config.format.time, "%Y");
        
        // Update configuration
        let new_config = CacheConfig::default();
        cache.set_config(new_config.clone());
        
        // Verify updated configuration
        let updated_config = cache.get_config();
        assert_eq!(updated_config.path.windows, new_config.path.windows);
        assert_eq!(updated_config.path.linux, new_config.path.linux);
        assert_eq!(updated_config.format.filename, new_config.format.filename);
        assert_eq!(updated_config.format.time, new_config.format.time);
    }
    
    #[test]
    fn test_cache_error_types() {
        use std::io::{Error, ErrorKind};
        
        // Test CacheError display implementation
        let io_error = CacheError::Io(Error::new(ErrorKind::NotFound, "File not found"));
        assert_eq!(format!("{}", io_error), "I/O error: File not found");
        
        let invalid_name_error = CacheError::InvalidName("test".to_string());
        assert_eq!(format!("{}", invalid_name_error), "Invalid cache name: test");
        
        let config_error = CacheError::ConfigParse("Invalid JSON".to_string());
        assert_eq!(format!("{}", config_error), "Configuration parse error: Invalid JSON");
        
        // Test error kind method
        assert_eq!(io_error.kind(), "io");
        assert_eq!(invalid_name_error.kind(), "invalid_name");
        assert_eq!(config_error.kind(), "config_parse");
        
        // Test error message method
        assert_eq!(io_error.message(), "File not found");
        assert_eq!(invalid_name_error.message(), "test");
        assert_eq!(config_error.message(), "Invalid JSON");
        
        // Test error type checks
        assert!(io_error.is_io_error());
        assert!(!invalid_name_error.is_io_error());
        assert!(!config_error.is_io_error());
    }
    
    #[test]
    fn test_cache_deprecated_functions() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let mut cache = Cache::new(config);
        
        // Create a cache object to test deprecated functions
        let cache_obj = cache.create("deprecated_test", None)
            .expect("Failed to create cache object");
        
        // Test deprecated is_expired method (always returns false)
        #[allow(deprecated)]
        let expired = cache_obj.is_expired();
        assert!(!expired, "is_expired should always return false");
        
        // Test deprecated cleanup method
        #[allow(deprecated)]
        let cleanup_result = cache.cleanup();
        assert!(cleanup_result.is_ok());
        
        // After cleanup, cache should be empty
        assert_eq!(cache.len(), 0);
    }
    
    #[test]
    fn test_cache_path_expansion() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_string_lossy().to_string();
        
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "test.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_path.replace("\\", "\\\\"),
        temp_path
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let cache = Cache::new(config);
        
        // Test path expansion - should not modify already absolute paths
        let expanded = cache.expand_path(&temp_path);
        assert_eq!(expanded, temp_path);
        
        // Test tilde expansion (if home directory is available)
        if dirs::home_dir().is_some() {
            let expanded = cache.expand_path("~/test/cache");
            assert!(!expanded.contains('~'), "Tilde should be expanded");
        }
    }
    
    #[test]
    fn test_cache_concurrent_ids() {
        let config = CacheConfig::default();
        let mut cache = Cache::new(config);
        
        // Create multiple cache objects and verify they get sequential IDs
        let obj1 = cache.create("obj1", None).expect("Failed to create obj1");
        assert_eq!(obj1.id(), 1);
        
        let obj2 = cache.create("obj2", None).expect("Failed to create obj2");
        assert_eq!(obj2.id(), 2);
        
        let obj3 = cache.create("obj3", None).expect("Failed to create obj3");
        assert_eq!(obj3.id(), 3);
        
        // Remove one and create another
        cache.remove("obj2").expect("Failed to remove obj2");
        let obj4 = cache.create("obj4", None).expect("Failed to create obj4");
        assert_eq!(obj4.id(), 4); // IDs should continue incrementing
        
        // Create more objects
        let obj5 = cache.create("obj5", None).expect("Failed to create obj5");
        assert_eq!(obj5.id(), 5);
    }
    
    #[test]
    fn test_cache_large_name_rejection() {
        let config = CacheConfig::default();
        let mut cache = Cache::new(config);
        
        // Create a name that's too long
        let long_name = "a".repeat(300);
        let result = cache.create(&long_name, None);
        
        // Should reject names longer than 255 characters
        assert!(result.is_err());
        assert_eq!(result.err().unwrap().kind(), "invalid_name");
        
        // Create a name at the limit
        let limit_name = "a".repeat(255);
        let result = cache.create(&limit_name, None);
        
        // Should accept names at exactly 255 characters
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_cache_clear_with_errors() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        let config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let config = CacheConfig::new(&config_json).expect("Failed to parse config");
        let mut cache = Cache::new(config);
        
        // Create cache objects
        let obj1 = cache.create("obj1", None).expect("Failed to create obj1");
        let obj2 = cache.create("obj2", None).expect("Failed to create obj2");
        
        // Manually delete one file to simulate error
        std::fs::remove_file(obj1.path()).expect("Failed to delete file");
        
        // Try to clear cache - should still work even with one error
        let result = cache.clear();
        assert!(result.is_ok()); // clear() should still succeed even with deletion errors
        
        // Cache should be empty
        assert_eq!(cache.len(), 0);
    }
    
    #[test]
    fn test_cache_duplicate_names() {
        let config = CacheConfig::default();
        let mut cache = Cache::new(config);
        
        // Create first cache object
        let result1 = cache.create("duplicate", None);
        assert!(result1.is_ok());
        
        // Try to create another with same name - should overwrite (not error)
        let result2 = cache.create("duplicate", None);
        assert!(result2.is_ok());
        
        // Should have only one object with that name
        assert_eq!(cache.len(), 1);
    }
    
    #[test]
    fn test_cache_empty_config_override() {
        let temp_dir = tempdir().expect("Failed to create temp directory");
        
        let base_config_json = format!(r#"
        {{
            "path": {{
                "windows": "{}",
                "linux": "{}"
            }},
            "format": {{
                "filename": "base_{{name}}.cache",
                "time": "%Y%m%d"
            }}
        }}"#,
        temp_dir.path().to_string_lossy().replace("\\", "\\\\"),
        temp_dir.path().to_string_lossy()
        );
        
        let base_config = CacheConfig::new(&base_config_json).expect("Failed to parse config");
        let mut cache = Cache::new(base_config);
        
        // Custom config with empty strings (should not override base config)
        let custom_config = r#"
        {
            "path": {
                "windows": "",
                "linux": ""
            },
            "format": {
                "filename": "",
                "time": ""
            }
        }
        "#;
        
        let cache_obj = cache.create("test", Some(custom_config))
            .expect("Failed to create cache object");
        
        let path_str = cache_obj.path().to_string_lossy();
        
        // Should use base config since custom config has empty strings
        assert!(path_str.contains("base_test"), 
                "Path should contain base pattern: {}", path_str);
        assert!(path_str.ends_with(".cache"), 
                "Path should end with .cache: {}", path_str);
    }
}