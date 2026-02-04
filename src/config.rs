/*
 * @filename: config.rs
 * @description: Configuration structures for cache-lite library
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

use serde::{Deserialize, Serialize};

/// Main configuration structure for cache behavior
/// 
/// # Fields
/// - `path`: Platform-specific storage paths (Windows/Linux)
/// - `format`: File naming format template
/// - `lifecycle`: Cache lifecycle policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub path: CachePathConfig,
    pub format: CacheFormatConfig
}

/// Platform-specific path configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachePathConfig {
    pub windows: String,
    pub linux: String,
}

/// File naming format configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheFormatConfig {
    pub filename: String,
    pub time: String
}

/// Cache lifecycle policy
#[deprecated(note="This enumeration has been temporarily deprecated due to issues. You can use the CacheObject::delete() function to delete cache files.")]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum LifecyclePolicy {
    /// Cache persists until program termination
    ProgramTerminated,
    /// Cache persists until it goes out of scope
    Scope,
    /// Cache never expires (default)
    Never
}

impl CacheConfig {
    /// Creates a new CacheConfig from JSON string
    /// 
    /// # Parameters
    /// - `json_config: &str` - JSON configuration string
    /// 
    /// # Returns
    /// New CacheConfig instance
    pub fn new(json_config: &str) -> Self {
        // Parse JSON configuration
        serde_json::from_str(json_config)
            .unwrap_or_else(|_| Self::default())
    }
    
    /// Creates a new CacheConfig with default values
    pub fn default() -> Self {
        CacheConfig {
            path: CachePathConfig {
                windows: "%temp%/Rust/Cache".to_string(),
                linux: "/tmp/Rust/Cache".to_string(),
            },
            format: CacheFormatConfig {
                filename: "r{name}.{time}.cache".to_string(),
                time: "%Y+%m+%d-%H+%M+%S".to_string()
            }
        }
    }
}