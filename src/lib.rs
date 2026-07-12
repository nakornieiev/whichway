use std::path::PathBuf;
use std::{env, fs};

pub fn find_matches(cmd: &str, path_var: &str) -> Vec<PathBuf> {
    env::split_paths(&path_var)
        .map(|dir| dir.join(cmd))
        .filter(|path| path.is_file())
        .collect()
}
#[derive(Debug, PartialEq)]
pub enum MatchKind {
    RealBinary,
    Symlink { target: PathBuf },
    Shim,
}

#[derive(Debug)]
pub struct ResolvedMatch {
    pub path: PathBuf,
    kind: MatchKind,
    is_active: bool,
}

pub fn classify(path: &PathBuf) -> MatchKind {
    let metadata = fs::symlink_metadata(path).unwrap();

    if metadata.file_type().is_symlink() {
        let target = fs::read_link(path).unwrap();
        return MatchKind::Symlink { target };
    }

    // TODO: Windows shim detection (.bat/.cmd) is not yet supported
    if let Ok(content) = fs::read_to_string(path) {
        if content.starts_with("#!") {
            return MatchKind::Shim;
        }
    }

    MatchKind::RealBinary
}

pub fn explain(m: &ResolvedMatch) -> String {
    let status = if m.is_active {
        "✅ active"
    } else {
        "shadowed"
    };
    match &m.kind {
        MatchKind::RealBinary => format!("[real binary]   {}", status),
        MatchKind::Symlink { target } => format!("[symlink -> {}]   {}", target.display(), status),
        MatchKind::Shim => format!("[shim script]   {}", status),
    }
}

pub fn resolve_all(cmd: &str, path_var: &str) -> Vec<ResolvedMatch> {
    find_matches(cmd, path_var)
        .into_iter()
        .enumerate()
        .map(|(i, path)| {
            let kind = classify(&path);
            ResolvedMatch {
                path,
                kind,
                is_active: i == 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::{MatchKind, classify, find_matches};
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn find_single_match() {
        let dir = tempdir().unwrap();
        let fake_bin = dir.path().join("myapp");
        fs::write(&fake_bin, "").unwrap();

        let path_var = dir.path().to_str().unwrap();
        let matches = find_matches("myapp", path_var);

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0], fake_bin);
    }

    #[test]
    fn find_multiple_matches() {
        let dir1 = tempdir().unwrap();
        let dir2 = tempdir().unwrap();
        fs::write(dir1.path().join("myapp"), "").unwrap();
        fs::write(dir2.path().join("myapp"), "").unwrap();

        let path_var = format!("{}:{}", dir1.path().display(), dir2.path().display());

        let matches = find_matches("myapp", &path_var);

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn find_nomatch() {
        let dir = tempdir().unwrap();
        let fake_bin = dir.path().join("app");
        fs::write(&fake_bin, "").unwrap();

        let path_var = dir.path().to_str().unwrap();
        let matches = find_matches("myapp", path_var);

        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn classify_symlink() {
        let dir = tempdir().unwrap();
        let original_file_path = dir.path().join("myapp");
        fs::write(&original_file_path, "").unwrap();

        let link_path = dir.path().join("mylink");

        #[cfg(unix)]
        std::os::unix::fs::symlink(&original_file_path, &link_path).unwrap();

        #[cfg(windows)]
        std::os::windows::fs::symlink_file(original_file_path, link_path).unwrap();

        let classification = classify(&link_path);

        assert_eq!(
            classification,
            MatchKind::Symlink {
                target: original_file_path
            }
        )
    }

    #[test]
    fn classify_shim() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myapp");
        fs::write(
            &file_path,
            "#!/usr/bin/env bash\nexec asdf exec python \"$@\"\n",
        )
        .unwrap();

        let classification = classify(&file_path);

        assert_eq!(classification, MatchKind::Shim);
    }

    #[test]
    fn classify_real_binary() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myapp");
        fs::write(&file_path, [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01]).unwrap();

        let classification = classify(&file_path);
        assert_eq!(classification, MatchKind::RealBinary);
    }
}
