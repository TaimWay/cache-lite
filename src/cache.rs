/*
 * @filename: cache.rs
 * @description: Main cache manager for cache-lite library
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

use crate::config::CacheConfig;
use crate::object::CacheObject;
use crate::utils::{expand_path, validate_name};
use crate::{CacheError, CacheResult};
use chrono::{DateTime, Local};
use std::collections::HashMap;
use std::time::SystemTime;

fn time_format(time: SystemTime, format: &str) -> String {
    let datetime: DateTime<Local> = time.into();
    datetime.format(format).to_string()
}

/// Main cache manager handling multiple cache objects
pub struct Cache {
    config: CacheConfig,
    objects: HashMap<String, CacheObject>,
    next_id: u32
}

impl Cache {
    /// Creates a new Cache with given configuration
    ///
    /// # Parameters
    /// - `config: CacheConfig` - Cache configuration
    ///
    /// # Returns
    /// New Cache instance
    pub fn new(config: CacheConfig) -> CacheResult<Self> {
        Ok(Cache {
            config,
            objects: HashMap::new(),
            next_id: 1
        })
    }

    /// Creates a new cache object with optional custom configuration
    ///
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    /// - `custom_config: Option<&str>` - Optional JSON configuration override
    ///
    /// # Returns
    /// New CacheObject instance
    pub fn create(&mut self, name: &str, custom_config: Option<&str>) -> CacheResult<CacheObject> {
        validate_name(name)?;

        if self.objects.contains_key(name) {
            return Err(CacheError::AlreadyExists(format!(
                "Cache object '{}' already exists",
                name
            )));
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut merged_config = self.config.clone();

        if let Some(config_str) = custom_config {
            match serde_json::from_str::<CacheConfig>(config_str) {
                Ok(custom) => {
                    if !custom.path.windows.is_empty() {
                        merged_config.path.windows = custom.path.windows.clone();
                    }
                    if !custom.path.linux.is_empty() {
                        merged_config.path.linux = custom.path.linux.clone();
                    }

                    if !custom.format.filename.is_empty() {
                        merged_config.format.filename = custom.format.filename.clone();
                    }
                    if !custom.format.time.is_empty() {
                        merged_config.format.time = custom.format.time.clone();
                    }
                }
                Err(e) => return Err(CacheError::ConfigParse(e.to_string())),
            }
        }

        let cache_path = if cfg!(windows) {
            expand_path(&merged_config.path.windows)
        } else {
            expand_path(&merged_config.path.linux)
        };

        let filename = merged_config
            .format
            .filename
            .replace("{name}", name)
            .replace("{id}", &id.to_string())
            .replace(
                "{time}",
                &time_format(SystemTime::now(), &merged_config.format.time),
            );

        let full_path = std::path::PathBuf::from(&cache_path).join(&filename);

        #[cfg(windows)]
        let full_path = std::path::PathBuf::from(full_path.to_string_lossy().replace('/', "\\"));

        // Create directory if it doesn't exist
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                CacheError::InvalidPath(format!("Failed to create cache directory: {}", e))
            })?;
        }

        let cache_object = CacheObject::new(name.to_string(), full_path.clone(), id);

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::Permissions::from_mode(0o600); // rw-------
            if let Ok(file) = std::fs::File::create(&full_path) {
                file.set_permissions(perms)
                    .map_err(|e| CacheError::PermissionDenied(e.to_string()))?;
            }
        }

        self.objects.insert(name.to_string(), cache_object.clone());

        Ok(cache_object)
    }

    /// Retrieves an existing cache object by name
    ///
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    ///
    /// # Returns
    /// `CacheResult<CacheObject>` - Retrieved cache object or error
    pub fn get(&self, name: &str) -> CacheResult<CacheObject> {
        self.objects
            .get(name)
            .cloned()
            .ok_or_else(|| CacheError::NotFound(format!("Cache object '{}' not found", name)))
    }

    /// Returns the number of cache objects
    ///
    /// # Returns
    /// `usize` - Count of cache objects
    pub fn len(&self) -> usize {
        self.objects.len()
    }

    /// Check if the cache list is empty
    ///
    /// # Returns
    /// `bool` - True if the cache list is empty, false otherwise
    pub fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    /// Removes a cache object by name
    ///
    /// # Parameters
    /// - `name: &str` - Cache object identifier
    ///
    /// # Returns
    /// `CacheResult<()>` - Success or error
    pub fn remove(&mut self, name: &str) -> CacheResult<()> {
        if let Some(cache_obj) = self.objects.remove(name) {
            cache_obj.delete()?;
        }
        Ok(())
    }

    /// Clears all cache objects
    ///
    /// # Returns
    /// `CacheResult<()>` - Success or error
    pub fn clear(&mut self) -> CacheResult<()> {
        let mut errors = Vec::new();

        for (name, cache_obj) in &self.objects {
            if let Err(e) = cache_obj.delete() {
                errors.push(format!("Failed to delete cache object '{}': {}", name, e));
            }
        }

        self.objects.clear();

        if !errors.is_empty() {
            return Err(CacheError::Generic(format!(
                "Errors occurred while clearing cache: {}",
                errors.join("; ")
            )));
        }

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
