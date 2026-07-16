use std::path::PathBuf;
use std::{env, fs};
use std::collections::HashMap;

fn list_all_executables(path_var: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for path in env::split_paths(path_var){
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Ok(meta) = fs::symlink_metadata(&path) {
                    if meta.is_file() || meta.file_type().is_symlink() {
                        paths.push(path);
                    }
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

fn find_broken_symlinks(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths
        .iter()
        .filter(|path| {
            fs::symlink_metadata(path)
                .map(|meta| meta.file_type().is_symlink())
                .unwrap_or(false)
                && !path.exists()
        })
        .cloned()
        .collect()
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

    #[test]
    fn find_one_broken_symlink() {
        let dir = tempdir().unwrap();
        let missing_target = dir.path().join("does_not_exist");
        let broken_link = dir.path().join("myapp");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&missing_target, &broken_link).unwrap();

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&missing_target, &broken_link).unwrap();

        let all = list_all_executables(dir.path().to_str().unwrap());
        let broken = find_broken_symlinks(&all);

        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0], broken_link);
    }
}