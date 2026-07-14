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