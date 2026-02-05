/*
 * @filename: error.rs
 * @description: Used for describing and handling cache errors
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

use std::fmt;
use std::io;

/// Cache library error types
#[derive(Debug)]
pub enum CacheError {
    /// I/O operation failed
    Io(io::Error),
    /// Invalid cache name
    InvalidName(String),
    /// Configuration parsing error
    ConfigParse(String),
    /// Cache object not found
    NotFound(String),
    /// Permission denied for operation
    PermissionDenied(String),
    /// Cache object already exists
    AlreadyExists(String),
    /// Cache object has expired
    Expired(String),
    /// Invalid configuration provided
    InvalidConfig(String),
    /// Serialization/deserialization error
    Serialization(String),
    /// Invalid path or directory
    InvalidPath(String),
    /// Symbolic link detected (security risk)
    SymlinkDetected(String),
    /// Cache size limit exceeded
    SizeLimitExceeded(String),
    /// Cache file count limit exceeded
    FileCountLimitExceeded(String),
    /// Cache object corrupted
    Corrupted(String),
    /// Generic error with message
    Generic(String),
}

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CacheError::Io(err) => write!(f, "I/O error: {}", err),
            CacheError::InvalidName(msg) => write!(f, "Invalid cache name: {}", msg),
            CacheError::ConfigParse(msg) => write!(f, "Configuration parse error: {}", msg),
            CacheError::NotFound(msg) => write!(f, "Cache not found: {}", msg),
            CacheError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
            CacheError::AlreadyExists(msg) => write!(f, "Cache already exists: {}", msg),
            CacheError::Expired(msg) => write!(f, "Cache expired: {}", msg),
            CacheError::InvalidConfig(msg) => write!(f, "Invalid configuration: {}", msg),
            CacheError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            CacheError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            CacheError::SymlinkDetected(msg) => write!(f, "Symbolic link detected: {}", msg),
            CacheError::SizeLimitExceeded(msg) => write!(f, "Cache size limit exceeded: {}", msg),
            CacheError::FileCountLimitExceeded(msg) => write!(f, "Cache file count limit exceeded: {}", msg),
            CacheError::Corrupted(msg) => write!(f, "Cache corrupted: {}", msg),
            CacheError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for CacheError {}

impl From<io::Error> for CacheError {
    fn from(err: io::Error) -> Self {
        CacheError::Io(err)
    }
}

impl From<serde_json::Error> for CacheError {
    fn from(err: serde_json::Error) -> Self {
        CacheError::ConfigParse(err.to_string())
    }
}

impl CacheError {
    /// Returns the error kind as a string
    pub fn kind(&self) -> &'static str {
        match self {
            CacheError::Io(_) => "io",
            CacheError::InvalidName(_) => "invalid_name",
            CacheError::ConfigParse(_) => "config_parse",
            CacheError::NotFound(_) => "not_found",
            CacheError::PermissionDenied(_) => "permission_denied",
            CacheError::AlreadyExists(_) => "already_exists",
            CacheError::Expired(_) => "expired",
            CacheError::InvalidConfig(_) => "invalid_config",
            CacheError::Serialization(_) => "serialization",
            CacheError::InvalidPath(_) => "invalid_path",
            CacheError::SymlinkDetected(_) => "symlink_detected",
            CacheError::SizeLimitExceeded(_) => "size_limit_exceeded",
            CacheError::FileCountLimitExceeded(_) => "file_count_limit_exceeded",
            CacheError::Corrupted(_) => "corrupted",
            CacheError::Generic(_) => "generic",
        }
    }
    
    /// Returns the error message without the error kind prefix
    pub fn message(&self) -> String {
        match self {
            CacheError::Io(err) => err.to_string(),
            CacheError::InvalidName(msg) => msg.clone(),
            CacheError::ConfigParse(msg) => msg.clone(),
            CacheError::NotFound(msg) => msg.clone(),
            CacheError::PermissionDenied(msg) => msg.clone(),
            CacheError::AlreadyExists(msg) => msg.clone(),
            CacheError::Expired(msg) => msg.clone(),
            CacheError::InvalidConfig(msg) => msg.clone(),
            CacheError::Serialization(msg) => msg.clone(),
            CacheError::InvalidPath(msg) => msg.clone(),
            CacheError::SymlinkDetected(msg) => msg.clone(),
            CacheError::SizeLimitExceeded(msg) => msg.clone(),
            CacheError::FileCountLimitExceeded(msg) => msg.clone(),
            CacheError::Corrupted(msg) => msg.clone(),
            CacheError::Generic(msg) => msg.clone(),
        }
    }
    
    /// Creates a new generic error
    pub fn new<S: Into<String>>(message: S) -> Self {
        CacheError::Generic(message.into())
    }
    
    /// Checks if the error is an I/O error
    pub fn is_io_error(&self) -> bool {
        matches!(self, CacheError::Io(_))
    }
    
    /// Checks if the error indicates something wasn't found
    pub fn is_not_found(&self) -> bool {
        matches!(self, CacheError::NotFound(_))
    }
    
    /// Checks if the error indicates permission was denied
    pub fn is_permission_denied(&self) -> bool {
        matches!(self, CacheError::PermissionDenied(_))
    }
}