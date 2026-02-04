/*
 * @filename: cache.rs
 * @description: Main cache manager for rust-cache library
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

use std::io;
use std::collections::HashMap;
use std::time::SystemTime;
use chrono::{DateTime, Local};
use crate::config::{CacheConfig, CacheFormatConfig, };
use crate::object::CacheObject;

fn time_format(time: SystemTime, format: &str) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format(format).to_string()
}

/// Main cache manager handling multiple cache objects
pub struct Cache {
    config: CacheConfig,
    objects: HashMap<String, CacheObject>,
    next_id: u32,
}

impl Cache {
    /// Creates a new Cache with given configuration
    /// 
    /// # Parameters
    /// - `config: CacheConfig` - Cache configuration
    /// 
    /// # Returns
    /// New Cache instance
    pub fn new(config: CacheConfig) -> Self {
        Cache {
            config,
            objects: HashMap::new(),
            next_id: 1,
        }
    }

    /// Creates a new cache object with optional custom configuration
    /// 
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    /// - `custom_config: Option<&str>` - Optional JSON configuration override
    /// 
    /// # Returns
    /// New CacheObject instance
    pub fn create(&mut self, name: &str, custom_config: Option<&str>) -> CacheObject {
        let id = self.next_id;
        self.next_id += 1;
        
        // Parse custom config if provided
        let mut format_str = self.config.format.filename.clone();
        
        if let Some(config_str) = custom_config {
            if let Ok(custom) = serde_json::from_str::<CacheFormatConfig>(config_str) {
                format_str = custom.filename;
            }
            // Note: Custom lifecycle would need more complex parsing
        }
        
        let cache_path = if cfg!(windows) {
            self.expand_path(&self.config.path.windows)
        } else {
            self.expand_path(&self.config.path.linux)
        };
        
        let filename = format_str
            .replace("{name}", name)
            .replace("{id}", &id.to_string())
            .replace("{time}", &time_format(SystemTime::now(), &self.config.format.time));
            
        let full_path = std::path::PathBuf::from(&cache_path).join(&filename);
        
        // Create directory if it doesn't exist
        if let Some(parent) = full_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        let cache_object = CacheObject::new(
            name.to_string(),
            full_path,
            id
        );
        
        self.objects.insert(name.to_string(), cache_object.clone());
        
        cache_object
    }

    /// Expands environment variables in path
    fn expand_path(&self, path: &str) -> String {
        let mut expanded = path.to_string();
        
        // Expand Windows environment variables
        if cfg!(windows) && path.contains("%") {
            expanded = self.expand_windows_env_vars(path);
        }
        
        // Expand tilde for home directory (Unix-like systems)
        if expanded.starts_with('~') {
            if let Some(home) = dirs::home_dir() {
                expanded = home.to_string_lossy().to_string() + &expanded[1..];
            }
        }
        
        expanded
    }

    /// Expands Windows environment variables
    fn expand_windows_env_vars(&self, path: &str) -> String {
        use std::env;
        let mut result = path.to_string();
        
        // Simple environment variable expansion
        if let Ok(temp) = env::var("TEMP") {
            result = result.replace("%temp%", &temp);
        }
        if let Ok(tmp) = env::var("TMP") {
            result = result.replace("%tmp%", &tmp);
        }
        if let Ok(appdata) = env::var("APPDATA") {
            result = result.replace("%appdata%", &appdata);
        }
        if let Ok(localappdata) = env::var("LOCALAPPDATA") {
            result = result.replace("%localappdata%", &localappdata);
        }
        if let Ok(userprofile) = env::var("USERPROFILE") {
            result = result.replace("%userprofile%", &userprofile);
        }
        
        result
    }

    /// Retrieves an existing cache object by name
    /// 
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    /// 
    /// # Returns
    /// `io::Result<CacheObject>` - Retrieved cache object or error
    pub fn get(&self, name: &str) -> io::Result<CacheObject> {
        self.objects.get(name)
            .cloned()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Cache object not found"))
    }

    /// Returns the number of cache objects
    /// 
    /// # Returns
    /// `io::Result<u32>` - Count of cache objects or error
    pub fn len(&self) -> io::Result<u32> {
        Ok(self.objects.len() as u32)
    }

    /// Removes a cache object by name
    /// 
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    /// 
    /// # Returns
    /// `io::Result<()>` - Success or error
    pub fn remove(&mut self, name: &str) -> io::Result<()> {
        if let Some(cache_obj) = self.objects.remove(name) {
            let _ = cache_obj.delete();
        }
        Ok(())
    }

    /// Cleans up expired cache objects
    /// 
    /// # Returns
    /// `io::Result<u32>` - Number of cleaned objects
    #[deprecated(note = "Due to being deprecated in its lifecycle, this function only returns 0; please use the Cache::clear()")]
    pub fn cleanup(&mut self) -> io::Result<u32> {
        Ok(0)
    }

    /// Clears all cache objects
    /// 
    /// # Returns
    /// `io::Result<()>` - Success or error
    pub fn clear(&mut self) -> io::Result<()> {
        for (_, cache_obj) in &self.objects {
            let _ = cache_obj.delete();
        }
        self.objects.clear();
        
        Ok(())
    }

    /// Updates the cache configuration
    /// 
    /// # Parameters
    /// - `config: CacheConfig` - New configuration
    pub fn set_config(&mut self, config: CacheConfig) {
        self.config = config;
    }

    /// Returns current cache configuration
    /// 
    /// # Returns
    /// `CacheConfig` - Current configuration
    pub fn get_config(&self) -> CacheConfig {
        self.config.clone()
    }

    /// Returns iterator over all cache objects
    /// 
    /// # Returns
    /// `impl Iterator<Item = &CacheObject>` - Iterator over cache objects
    pub fn iter(&self) -> impl Iterator<Item = &CacheObject> {
        self.objects.values()
    }
}