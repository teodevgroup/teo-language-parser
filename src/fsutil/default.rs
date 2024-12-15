use super::FSUtil;
use path_clean::PathClean;
use std::fs;
use std::path::{Path, PathBuf};

fn read_file(file_path: &str) -> Option<String> {
    match fs::read_to_string(Path::new(file_path)) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}

fn parent_directory(path: &str) -> String {
    let mut path = PathBuf::from(path);
    path.pop();
    path.to_str().unwrap().to_string()
}

fn path_is_absolute(path: &str) -> bool {
    Path::new(path).is_absolute()
}

fn path_join(base: &str, path: &str) -> String {
    Path::new(base)
        .join(Path::new(path))
        .clean()
        .to_str()
        .unwrap()
        .to_string()
}

fn file_is_directory(file_path: &str) -> bool {
    Path::new(file_path).is_dir()
}

/// Provide a default implementation for file system utility.
impl Default for FSUtil {
    fn default() -> Self {
        Self::new(
            read_file,
            file_exists,
            file_is_directory,
            path_join,
            parent_directory,
            path_is_absolute,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_returns_string_if_file_exists_and_readable() {
        let file_path = "src/fsutil/default.rs";
        let content = read_file(file_path).unwrap();
        let fs_content = fs::read_to_string(Path::new(file_path)).unwrap();
        assert_eq!(content, fs_content);
    }

    #[test]
    fn test_read_file_returns_none_if_file_does_not_exist() {
        let file_path = "src/fsutil/nonexistent.rs";
        let content = read_file(file_path);
        assert_eq!(content, None);
    }

    #[test]
    fn test_file_exists_returns_true_if_file_exists() {
        let file_path = "src/fsutil/default.rs";
        let exists = file_exists(file_path);
        assert_eq!(exists, true);
    }

    #[test]
    fn test_file_exists_returns_false_if_file_does_not_exist() {
        let file_path = "src/fsutil/nonexistent.rs";
        let exists = file_exists(file_path);
        assert_eq!(exists, false);
    }

    #[test]
    fn test_file_is_directory_returns_true_if_file_is_directory() {
        let file_path = "src/fsutil";
        let is_directory = file_is_directory(file_path);
        assert_eq!(is_directory, true);
    }

    #[test]
    fn test_file_is_directory_returns_false_if_file_is_not_directory() {
        let file_path = "src/fsutil/default.rs";
        let is_directory = file_is_directory(file_path);
        assert_eq!(is_directory, false);
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path() {
        let path = "src/fsutil/default.rs";
        let parent = parent_directory(path);
        assert_eq!(parent, "src/fsutil");
    }

    #[test]
    fn test_path_is_absolute_returns_true_if_path_is_absolute() {
        if cfg!(windows) {
            let path = "C:\\Users\\user";
            let is_absolute = path_is_absolute(path);
            assert_eq!(is_absolute, true);
            return;
        } else {
            let path = "/home/user";
            let is_absolute = path_is_absolute(path);
            assert_eq!(is_absolute, true);
        }
    }

    #[test]
    fn test_path_is_absolute_returns_false_if_path_is_not_absolute() {
        let path = "home/user";
        let is_absolute = path_is_absolute(path);
        assert_eq!(is_absolute, false);
    }

    #[test]
    fn test_path_join_returns_joined_path() {
        if cfg!(windows) {
            let base = "src";
            let path = "fsutil\\default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "src\\fsutil\\default.rs");
            return;
        } else {
            let base = "src";
            let path = "fsutil/default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "src/fsutil/default.rs");
        }
    }

    #[test]
    fn test_path_join_removes_redundant_path_components() {
        if cfg!(windows) {
            let base = "src";
            let path = "fsutil\\..\\fsutil\\default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "src\\fsutil\\default.rs");
            return;
        } else {
            let base = "src";
            let path = "fsutil/../fsutil/default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "src/fsutil/default.rs");
        }
    }

    #[test]
    fn test_path_join_returns_base_if_path_is_empty() {
        let base = "src";
        let path = "";
        let joined = path_join(base, path);
        assert_eq!(joined, "src");
    }

    #[test]
    fn test_path_join_returns_path_if_base_is_empty() {
        if cfg!(windows) {
            let base = "";
            let path = "fsutil\\default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "fsutil\\default.rs");
            return;
        } else {
            let base = "";
            let path = "fsutil/default.rs";
            let joined = path_join(base, path);
            assert_eq!(joined, "fsutil/default.rs");
        }
    }

    #[test]
    fn test_parent_directory_returns_root_if_path_is_root() {
        let path = "/";
        let parent = parent_directory(path);
        assert_eq!(parent, "/");
    }

    #[test]
    fn test_parent_directory_returns_empty_string_if_path_is_empty() {
        let path = "";
        let parent = parent_directory(path);
        assert_eq!(parent, "");
    }

    #[test]
    fn test_parent_directory_returns_empty_string_if_path_is_single_component() {
        let path = "src";
        let parent = parent_directory(path);
        assert_eq!(parent, "");
    }

    #[test]
    fn test_parent_directory_returns_empty_string_if_path_is_single_component_with_trailing_slash()
    {
        let path = "src/";
        let parent = parent_directory(path);
        assert_eq!(parent, "");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_trailing_slash() {
        let path = "src/fsutil/";
        let parent = parent_directory(path);
        assert_eq!(parent, "src");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components() {
        let path = "src/fsutil/default.rs";
        let parent = parent_directory(path);
        assert_eq!(parent, "src/fsutil");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components_and_trailing_slash(
    ) {
        let path = "src/fsutil/default.rs/";
        let parent = parent_directory(path);
        assert_eq!(parent, "src/fsutil");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components_and_multiple_trailing_slashes(
    ) {
        let path = "src/fsutil/default.rs///";
        let parent = parent_directory(path);
        assert_eq!(parent, "src/fsutil");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components_and_multiple_trailing_slashes_and_root(
    ) {
        let path = "/src/fsutil/default.rs///";
        let parent = parent_directory(path);
        assert_eq!(parent, "/src/fsutil");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components_and_multiple_trailing_slashes_and_root_and_trailing_slash(
    ) {
        let path = "/src/fsutil/default.rs///";
        let parent = parent_directory(path);
        assert_eq!(parent, "/src/fsutil");
    }

    #[test]
    fn test_parent_directory_returns_parent_directory_of_path_with_multiple_components_and_multiple_trailing_slashes_and_root_and_trailing_slash_and_root(
    ) {
        let path = "/src/fsutil/default.rs///";
        let parent = parent_directory(path);
        assert_eq!(parent, "/src/fsutil");
    }
}
