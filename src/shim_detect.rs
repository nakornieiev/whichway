use std::path::{Component, Path};
use crate::resolvers::ManagerInfo;

pub fn detect_manager(path: &Path, content: &str) -> Option<ManagerInfo> {
    for component in path.components() {
        if let Component::Normal(name) = component {
            if name == ".asdf" {
                return Some(ManagerInfo::Asdf);
            } else if name == ".pyenv" {
                return Some(ManagerInfo::Pyenv)
            } else if name == ".nvm" {
                return Some(ManagerInfo::Nvm)
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_asdf_by_path() {
        let path = Path::new("/home/user/.asdf/shims/python");

        let manager_info = detect_manager(&path, "");
        assert_eq!(manager_info, Some(ManagerInfo::Asdf));
    }

    #[test]
    fn detect_nvm_by_path() {
        let path = Path::new("/home/user/.nvm/shims/node");

        let manager_info = detect_manager(&path, "");
        assert_eq!(manager_info, Some(ManagerInfo::Nvm));
    }

    #[test]
    fn detect_pyenv_by_path() {
        let path = Path::new("/home/user/.pyenv/shims/python");

        let manager_info = detect_manager(&path, "");
        assert_eq!(manager_info, Some(ManagerInfo::Pyenv));
    }

    #[test]
    fn detect_no_manager() {
        let path = Path::new("/home/user/.some_manager/shims/some_package");

        let manager_info = detect_manager(&path, "");
        assert_eq!(manager_info, None);
    }
}