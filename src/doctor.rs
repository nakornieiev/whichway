use crate::shim_detect::detect_manager;
use serde::Serialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

#[derive(Serialize, Debug)]
pub struct DoctorReport {
    pub duplicates: HashMap<String, Vec<PathBuf>>,
    pub broken_symlinks: Vec<PathBuf>,
    pub orphan_shims: Vec<PathBuf>,
}

fn list_all_executables(path_var: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    for path in env::split_paths(path_var) {
        if let Ok(entries) = fs::read_dir(&path) {
            for entry in entries.flatten() {
                let path = entry.path();

                if let Ok(meta) = fs::symlink_metadata(&path)
                    && (meta.is_file() || meta.file_type().is_symlink())
                {
                    paths.push(path);
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
        let entry = grouped.entry(file_name).or_default();
        if !entry.contains(&path) {
            entry.push(path);
        }
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

fn find_orphan_shims(paths: &[PathBuf], home: &Path) -> Vec<PathBuf> {
    let mut orphan_shims: Vec<PathBuf> = Vec::new();

    for path in paths {
        let content = fs::read_to_string(path).unwrap_or(String::from(""));

        if let Some(manager) = detect_manager(path, &content)
            && !home.join(manager.home_dir()).exists()
        {
            orphan_shims.push(path.clone());
        }
    }

    orphan_shims
}

pub fn run_doctor(path_var: &str, home: &Path) -> DoctorReport {
    let executables = list_all_executables(path_var);

    DoctorReport {
        broken_symlinks: find_broken_symlinks(&executables),
        orphan_shims: find_orphan_shims(&executables, home),
        duplicates: find_duplicates(executables),
    }
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

        let path_var = env::join_paths([dir1.path(), dir2.path()]).unwrap();
        let result = list_all_executables(&path_var.to_str().unwrap());

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

    #[test]
    fn find_no_orphan_shims() {
        let home = tempdir().unwrap();
        fs::write(home.path().join(".pyenv"), "").unwrap();

        let shim = home.path().join("python");
        fs::write(&shim, "#!/bin/bash\nexec .pyenv exec python\n").unwrap();

        let orphan_shims = find_orphan_shims(&[shim], home.path());

        assert!(orphan_shims.is_empty());
    }

    #[test]
    fn find_some_orphan_shims() {
        let dir = tempdir().unwrap();
        let shim = dir.path().join("python");
        fs::write(&shim, "#!/bin/bash\nexec .pyenv exec python\n").unwrap();

        let orphan_shims = find_orphan_shims(&[shim], dir.path());
        assert_eq!(orphan_shims.len(), 1);
    }

    #[test]
    fn doctor_report_aggregates_all_checks() {
        // Duplicates
        let dup_dir1 = tempdir().unwrap();
        let dup_dir2 = tempdir().unwrap();
        fs::write(dup_dir1.path().join("myapp"), "").unwrap();
        fs::write(dup_dir2.path().join("myapp"), "").unwrap();

        // Broken symlinks
        let broken_dir = tempdir().unwrap();
        let missing_target = broken_dir.path().join("does_not_exist");
        let broken_link = broken_dir.path().join("brokenlink");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&missing_target, &broken_link).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&missing_target, &broken_link).unwrap();

        // Orphan shim
        let orphan_dir = tempdir().unwrap();
        fs::write(
            orphan_dir.path().join("orphanapp"),
            "#!/bin/bash\nexec .pyenv exec python\n",
        )
        .unwrap();

        let fake_home = tempdir().unwrap();

        let path_var = env::join_paths([
            dup_dir1.path(),
            dup_dir2.path(),
            broken_dir.path(),
            orphan_dir.path(),
        ])
        .unwrap();

        let result = run_doctor(path_var.to_str().unwrap(), fake_home.path());

        assert_eq!(result.duplicates.len(), 1);
        assert_eq!(result.broken_symlinks.len(), 1);
        assert_eq!(result.orphan_shims.len(), 1);
    }
}
