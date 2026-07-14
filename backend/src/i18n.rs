pub fn get_string(data: &serde_json::Value, path: &[&str], lang: &str) -> String {
    let mut current = data;
    for key in path {
        current = match current.get(key) {
            Some(v) => v,
            None => return String::new(),
        };
    }
    // Try the requested language first, then fallback to 'es'
    if let Some(v) = current.get(lang) {
        v.as_str().unwrap_or("").to_string()
    } else if let Some(v) = current.get("es") {
        v.as_str().unwrap_or("").to_string()
    } else if let Some(v) = current.as_str() {
        v.to_string()
    } else {
        String::new()
    }
}

pub fn get_string_field(value: &serde_json::Value, lang: &str, fallback: &str) -> String {
    if let Some(obj) = value.as_object() {
        if let Some(v) = obj.get(lang) {
            return v.as_str().unwrap_or(fallback).to_string();
        }
        if let Some(v) = obj.get("es") {
            return v.as_str().unwrap_or(fallback).to_string();
        }
    }
    if let Some(s) = value.as_str() {
        return s.to_string();
    }
    fallback.to_string()
}
