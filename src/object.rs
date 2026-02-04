/*
 * @filename: object.rs
 * @description: Cache object implementation for cache-lite library
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

use std::path::{Path, PathBuf};
use std::time::SystemTime;
use crate::{CacheError, CacheResult};

/// Represents an individual cache object with file operations
pub struct CacheObject {
    name: String,
    path: PathBuf,
    id: u32,
    created_at: SystemTime
}

impl CacheObject {
    /// Creates a new CacheObject
    pub fn new(
        name: String, 
        path: PathBuf, 
        id: u32
    ) -> Self {
        let obj = CacheObject {
            name,
            path,
            id,
            created_at: SystemTime::now()
        };
        
        obj
    }

    /// Returns the cache object name
    /// 
    /// # Returns
    /// `&str` - Cache object identifier
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the filesystem path of the cache object
    /// 
    /// # Returns
    /// `&Path` - Path to cache file
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Returns the creation time of the cache object
    /// 
    /// # Returns
    /// `SystemTime` - Creation timestamp
    pub fn created_at(&self) -> SystemTime {
        self.created_at
    }

    /// Returns the cache object ID
    /// 
    /// # Returns
    /// `u32` - Unique identifier
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Opens the cache file for reading/writing
    /// 
    /// # Returns
    /// `CacheResult<std::fs::File>` - File handle or error
    pub fn get_file(&self) -> CacheResult<std::fs::File> {
        std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&self.path)
            .map_err(|e| CacheError::Io(e))
    }

    /// Reads and returns the entire cache content as string
    /// 
    /// # Returns
    /// `CacheResult<String>` - Cache content or error
    pub fn get_string(&self) -> CacheResult<String> {
        std::fs::read_to_string(&self.path)
            .map_err(|e| CacheError::Io(e))
    }

    /// Writes string content to the cache file
    /// 
    /// # Parameters
    /// - `content: &str` - Content to write
    /// 
    /// # Returns
    /// `CacheResult<()>` - Success or error
    pub fn write_string(&self, content: &str) -> CacheResult<()> {
        std::fs::write(&self.path, content)
            .map_err(|e| CacheError::Io(e))
    }

    /// Writes binary content to the cache file
    /// 
    /// # Parameters
    /// - `content: &[u8]` - Binary content to write
    /// 
    /// # Returns
    /// `CacheResult<()>` - Success or error
    pub fn write_bytes(&self, content: &[u8]) -> CacheResult<()> {
        std::fs::write(&self.path, content)
            .map_err(|e| CacheError::Io(e))
    }

    /// Reads and returns the entire cache content as bytes
    /// 
    /// # Returns
    /// `CacheResult<Vec<u8>>` - Cache content or error
    pub fn get_bytes(&self) -> CacheResult<Vec<u8>> {
        std::fs::read(&self.path)
            .map_err(|e| CacheError::Io(e))
    }

    /// Deletes the cache object and its file
    /// 
    /// # Returns
    /// `CacheResult<()>` - Success or error
    pub fn delete(&self) -> CacheResult<()> {
        if self.path.exists() {
            std::fs::remove_file(&self.path)
                .map_err(|e| CacheError::Io(e))?;
        }
        Ok(())
    }

    /// Checks if the cache file exists
    /// 
    /// # Returns
    /// `bool` - True if the cache file exists
    pub fn exists(&self) -> bool {
        self.path.exists()
    }

    /// Gets the file size in bytes
    /// 
    /// # Returns
    /// `CacheResult<u64>` - File size in bytes or error
    pub fn size(&self) -> CacheResult<u64> {
        std::fs::metadata(&self.path)
            .map(|metadata| metadata.len())
            .map_err(|e| CacheError::Io(e))
    }

    /// Checks if the cache has expired based on its lifecycle policy
    /// 
    /// # Returns
    /// `bool` - True if expired, false otherwise
    #[deprecated(note="This enumeration has been deprecated due to issues, and it now only returns false")]
    pub fn is_expired(&self) -> bool {
        false
    }
}

impl Clone for CacheObject {
    fn clone(&self) -> Self {
        CacheObject {
            name: self.name.clone(),
            path: self.path.clone(),
            id: self.id,
            created_at: self.created_at
        }
    }
}