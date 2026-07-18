use crate::shim_detect::detect_manager;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::{env, fs};

#[derive(Debug, PartialEq, Serialize)]
pub enum MatchKind {
    RealBinary,
    Symlink { target: PathBuf },
    Shim,
    NotIdentified(String),
}

#[derive(Debug, PartialEq, Serialize)]
pub enum ManagerInfo {
    Asdf,
    Nvm,
    Pyenv,
}

impl ManagerInfo {
    pub fn home_dir(&self) -> &'static str {
        match self {
            ManagerInfo::Asdf => ".asdf",
            ManagerInfo::Nvm => ".nvm",
            ManagerInfo::Pyenv => ".pyenv",
        }
    }
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ResolvedMatch {
    pub path: PathBuf,
    pub kind: MatchKind,
    pub manager: Option<ManagerInfo>,
    pub is_active: bool,
}

#[derive(Debug)]
pub enum ClassifyError {
    MetadataReadFailed(std::io::Error),
    ReadLinkFailed(std::io::Error),
}

impl Display for ClassifyError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ClassifyError::MetadataReadFailed(e) => write!(f, "Failed to read metadata: {}", e),
            ClassifyError::ReadLinkFailed(e) => write!(f, "Failed to read link: {}", e),
        }
    }
}

pub fn find_matches(cmd: &str, path_var: &str) -> Vec<PathBuf> {
    env::split_paths(path_var)
        .map(|dir| dir.join(cmd))
        .filter(|path| path.is_file())
        .collect()
}

pub fn classify(path: &PathBuf, content: &str) -> Result<MatchKind, ClassifyError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(err) => return Err(ClassifyError::MetadataReadFailed(err)),
    };

    if metadata.file_type().is_symlink() {
        let target = match fs::read_link(path) {
            Ok(target) => target,
            Err(err) => return Err(ClassifyError::ReadLinkFailed(err)),
        };
        return Ok(MatchKind::Symlink { target });
    }

    // TODO: Windows shim detection (.bat/.cmd) is not yet supported
    if content.starts_with("#!") {
        return Ok(MatchKind::Shim);
    }

    Ok(MatchKind::RealBinary)
}

pub fn resolve_all(cmd: &str, path_var: &str) -> Vec<ResolvedMatch> {
    find_matches(cmd, path_var)
        .into_iter()
        .enumerate()
        .map(|(i, path)| {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let manager = detect_manager(&path, &content);
            let kind = classify(&path, &content)
                .unwrap_or_else(|err| MatchKind::NotIdentified(err.to_string()));
            ResolvedMatch {
                path,
                kind,
                manager,
                is_active: i == 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
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
        std::os::windows::fs::symlink_file(&original_file_path, &link_path).unwrap();

        let classification = classify(&link_path, "").unwrap();

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

        let content = fs::read_to_string(&file_path).unwrap();

        let classification = classify(&file_path, &content).unwrap();

        assert_eq!(classification, MatchKind::Shim);
    }

    #[test]
    fn classify_real_binary() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myapp");
        fs::write(&file_path, [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01]).unwrap();

        let classification = classify(&file_path, "").unwrap();
        assert_eq!(classification, MatchKind::RealBinary);
    }

    #[test]
    fn classify_nonexistent_path_returns_error() {
        let dir = tempdir().unwrap();
        let missing = dir.path().join("does-not-exist");

        let result = classify(&missing, "");
        assert!(result.is_err());
    }

    #[test]
    fn resolve_all_finds_symlink() {
        let dir = tempdir().unwrap();
        let symlink = dir.path().join("target");
        fs::write(&symlink, "").unwrap();
        let link_path = dir.path().join("myapp");
        #[cfg(unix)]
        std::os::unix::fs::symlink(&symlink, &link_path).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_file(&symlink, &link_path).unwrap();

        let resolved = resolve_all("myapp", dir.path().to_str().unwrap());
        assert_eq!(
            resolved[0],
            ResolvedMatch {
                path: link_path,
                kind: MatchKind::Symlink { target: symlink },
                manager: None,
                is_active: true
            }
        );
    }

    #[test]
    fn resolve_all_finds_shim() {
        let dir = tempdir().unwrap();
        let shim = dir.path().join("myapp");
        fs::write(&shim, "#!/usr/bin/env bash\nexec asdf exec python \"$@\"\n").unwrap();

        let resolved = resolve_all("myapp", dir.path().to_str().unwrap());
        assert_eq!(
            resolved[0],
            ResolvedMatch {
                path: shim,
                kind: MatchKind::Shim,
                manager: None,
                is_active: true
            }
        );
    }
    #[test]
    fn resolve_all_finds_binary() {
        let dir = tempdir().unwrap();
        let binary = dir.path().join("myapp");
        fs::write(&binary, [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01]).unwrap();

        let resolved = resolve_all("myapp", dir.path().to_str().unwrap());
        assert_eq!(
            resolved[0],
            ResolvedMatch {
                path: binary,
                kind: MatchKind::RealBinary,
                manager: None,
                is_active: true
            }
        );
    }

    #[test]
    fn resolve_all_finds_manager_via_path() {
        let dir = tempdir().unwrap();
        let manager_bin = dir
            .path()
            .join(".nvm")
            .join("versions")
            .join("node")
            .join("v18.0.0")
            .join("bin");
        fs::create_dir_all(&manager_bin).unwrap();

        let binary = manager_bin.join("node");
        fs::write(&binary, [0x7F, 0x45, 0x4C, 0x46, 0x02, 0x01]).unwrap();

        let resolved = resolve_all("node", manager_bin.to_str().unwrap());

        assert_eq!(resolved.len(), 1);
        assert_eq!(resolved[0].kind, MatchKind::RealBinary);
        assert_eq!(resolved[0].manager, Some(ManagerInfo::Nvm));
    }

    #[test]
    fn resolve_all_finds_manager_via_content_fallback() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("myapp");
        fs::write(
            &file_path,
            "#!/usr/bin/env bash\nexec .asdf exec python \"$@\"\n",
        )
        .unwrap();

        let resolved = resolve_all("myapp", dir.path().to_str().unwrap());

        assert_eq!(resolved[0].kind, MatchKind::Shim);
        assert_eq!(resolved[0].manager, Some(ManagerInfo::Asdf));
    }
}
