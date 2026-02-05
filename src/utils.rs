// utils.rs
use crate::{CacheResult, CacheError};

/// Expands Windows environment variables
fn expand_windows_env_vars(path: &str) -> String {
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

/// Expands environment variables in path
pub fn expand_path(path: &str) -> String {
    let mut expanded = path.to_string();

    // Expand Windows environment variables
    if cfg!(windows) && path.contains("%") {
        expanded = expand_windows_env_vars(path);
    }

    // Expand tilde for home directory (Unix-like systems)
    if expanded.starts_with('~') {
        if let Some(home) = dirs::home_dir() {
            expanded = home.to_string_lossy().to_string() + &expanded[1..];
        }
    }

    #[cfg(windows)]
    {
        expanded = expanded.replace('/', "\\");
    }

    expanded
}

/// Validates if a cache name is valid
///
/// # Parameters
/// - `name: &str` - Cache object identifier
///
/// # Returns
/// `CacheResult<()>` - Success or error
pub fn validate_name(name: &str) -> CacheResult<()> {
    if name.is_empty() {
        return Err(CacheError::InvalidName(
            "Cache name cannot be empty".to_string(),
        ));
    }

    if name.len() > 255 {
        return Err(CacheError::InvalidName(
            "Cache name too long (max 255 characters)".to_string(),
        ));
    }

    if name.contains('\0') {
        return Err(CacheError::InvalidName(
            "Cache name cannot contain null bytes".to_string(),
        ));
    }

    if name.contains("..")
        || name.contains(std::path::MAIN_SEPARATOR)
        || name.contains('/')
        || name.contains('\\')
    {
        return Err(CacheError::InvalidName(
            "Invalid cache name: contains path components".to_string(),
        ));
    }

    #[cfg(windows)]
    {
        let invalid_chars = ['<', '>', ':', '"', '|', '?', '*'];
        if name.chars().any(|c| invalid_chars.contains(&c)) {
            return Err(CacheError::InvalidName(format!(
                "Cache name contains invalid character for Windows: {}",
                name
            )));
        }
    }

    #[cfg(windows)]
    {
        let reserved_names = [
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        let uppercase_name = name.to_uppercase();
        for reserved in &reserved_names {
            if uppercase_name == *reserved || uppercase_name.starts_with(&format!("{}.", reserved))
            {
                return Err(CacheError::InvalidName(format!(
                    "Cache name '{}' is a reserved system name",
                    name
                )));
            }
        }
    }

    if name.chars().any(|c| c.is_control()) {
        return Err(CacheError::InvalidName(
            "Cache name cannot contain control characters".to_string(),
        ));
    }

    Ok(())
}
