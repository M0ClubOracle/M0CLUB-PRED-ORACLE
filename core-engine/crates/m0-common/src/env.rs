
use std::env;

pub fn get_var(key: &str) -> Option<String> {
    env::var(key).ok().filter(|v| !v.trim().is_empty())
}

pub fn get_var_or(key: &str, default: &str) -> String {
    get_var(key).unwrap_or_else(|| default.to_string())
}

pub fn is_truthy(v: &str) -> bool {
    matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "y" | "on")
}
