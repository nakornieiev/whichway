#[cfg(test)]
mod tests {
    use std::fs;
    use tempfile::tempdir;
    use whichway::find_matches;

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
}