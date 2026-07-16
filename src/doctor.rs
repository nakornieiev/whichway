use std::path::PathBuf;
use std::{env, fs};
use std::collections::HashMap;

fn list_all_executables(path_var: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for path in env::split_paths(path_var){
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                if entry.path().is_file() {
                    paths.push(entry.path());
                }
            }
        }
    }

    paths
}

fn find_duplicates(paths: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut grouped: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for path in paths {
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
        grouped.entry(file_name).or_default().push(path);
    }

    grouped.into_iter().filter(|(_, v)| v.len() > 1).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    #[test]
    fn list_all_executables_across_multiple_dirs() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        fs::write(dir1.path().join("app1"), "").unwrap();
        fs::write(dir2.path().join("app2"), "").unwrap();

        let path_var = format!("{}:{}", dir1.path().display(), dir2.path().display());
        let result = list_all_executables(&path_var);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn find_two_duplicates() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();

        let paths = vec![dir1.path().join("myapp"), dir2.path().join("myapp")];

        assert_eq!(find_duplicates(paths.clone()).contains_key("myapp"), true);
        assert_eq!(find_duplicates(paths).get("myapp").unwrap().len(), 2);
    }

    #[test]
    fn no_duplicates_for_unique_file() {
        let dir1 = tempdir().unwrap();

        let paths = vec![dir1.path().join("uniqueapp")];
        let duplicates = find_duplicates(paths);

        assert!(!duplicates.contains_key("uniqueapp"));
    }
}