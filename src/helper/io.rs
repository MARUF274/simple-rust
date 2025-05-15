use std::{env, fs, path::PathBuf, str::FromStr};

#[allow(dead_code)]
pub fn get_env_or_default<T: FromStr + ToString>(key: &str, default: T) -> T {
    env::var(key)
        .ok()
        .and_then(|val| val.parse::<T>().ok())
        .unwrap_or(default)
}

#[allow(dead_code)]
fn collect_file_paths_recursively(file_paths: &mut Vec<PathBuf>, current_path: PathBuf) {
    if current_path.is_file() {
        file_paths.push(current_path);
        return;
    }

    if let Ok(parent) = fs::read_dir(&current_path) {
        for child in parent.flatten() {
            let child_path = child.path();
            if child_path.is_file() {
                file_paths.push(child_path);
            } else {
                collect_file_paths_recursively(file_paths, child_path);
            }
        }
    } else {
        eprintln!(
            "Failed to read directory: {}",
            current_path.to_string_lossy()
        );
    }
}
