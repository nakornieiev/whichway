use std::env;
use std::path::PathBuf;

pub fn find_matches(cmd: &str, path_var: &str) -> Vec<PathBuf> {
    env::split_paths(&path_var)
        .map(|dir| dir.join(cmd))
        .filter(|path| path.is_file())
        .collect()
}