
/// # File system utility
///
/// The file system utility provides a set of functions to interact with the
/// file system. This is designed with compatibility in mind. Not every
/// platform that we target supports the standard Rust file system library.
/// For example, WASM doesn't support Rust file system functions in the
/// stdlib.
pub struct FSUtil {
    /// The utility function to read file content.
    read_file: fn(path: &str) -> Option<String>,
    /// The utility function to check whether a file exists.
    file_exists: fn(path: &str) -> bool,
    /// The utility function to check if file is a directory.
    file_is_directory: fn(path: &str) -> bool,
    /// The utility function to join `base` and `path` into a single path.
    path_join: fn(base: &str, path: &str) -> String,
    /// The utility function to get the parent directory of the argument.
    parent_directory: fn(path: &str) -> String,
    /// The utility function to check whether a path is absolute.
    path_is_absolute: fn(path: &str) -> bool,
}

impl FSUtil {
    /// Create a new instance of file system utility.
    ///
    /// # Arguments
    ///
    /// * `read_file` - A function to read file content.
    /// * `file_exists` - A function to check whether a file exists.
    /// * `file_is_directory` - A function to check if file is a directory.
    /// * `path_join` - A function to join `base` and `path` into a single path.
    /// * `parent_directory` - A function to get the parent directory of the argument.
    ///
    /// # Examples
    /// ```
    /// use teo_language_parser::fsutil::FSUtil;
    /// use std::fs;
    /// use std::path::{Path, PathBuf};
    ///
    /// fn read_file(path: &str) -> Option<String> {
    ///    match fs::read_to_string(Path::new(path)) {
    ///       Ok(s) => Some(s),
    ///      Err(_) => None,
    ///   }
    /// }
    ///
    /// fn file_exists(path: &str) -> bool {
    ///     Path::new(path).exists()
    /// }
    ///
    /// fn parent_directory(path: &str) -> String {
    ///    let mut path = PathBuf::from(path);
    ///    path.pop();
    ///    path.to_str().unwrap().to_string()
    /// }
    ///
    /// fn path_is_absolute(path: &str) -> bool {
    ///     path.starts_with("/")
    /// }
    ///
    /// fn path_join(base: &str, path: &str) -> String {
    ///     format!("{}/{}", base, path)
    /// }
    ///
    /// fn file_is_directory(path: &str) -> bool {
    ///     Path::new(path).is_dir()
    /// }
    ///
    /// let fs_util = FSUtil::new(
    ///     read_file,
    ///     file_exists,
    ///     file_is_directory,
    ///     path_join,
    ///     parent_directory,
    ///     path_is_absolute
    /// );
    /// assert!(fs_util.file_exists("Cargo.toml"));
    /// assert!(!fs_util.file_is_directory("Cargo.toml"));
    /// assert_eq!(fs_util.path_join("src", "main.rs"), "src/main.rs");
    /// assert_eq!(fs_util.parent_directory("src/main.rs"), "src");
    /// assert!(fs_util.path_is_absolute("/home/user"));
    /// ```
    pub fn new(
        read_file: fn(path: &str) -> Option<String>,
        file_exists: fn(path: &str) -> bool,
        file_is_directory: fn(path: &str) -> bool,
        path_join: fn(base: &str, path: &str) -> String,
        parent_directory: fn(path: &str) -> String,
        path_is_absolute: fn(path: &str) -> bool,
    ) -> Self {
        Self {
            read_file,
            file_exists,
            file_is_directory,
            path_join,
            parent_directory,
            path_is_absolute,
        }
    }

    /// Read the file content from `path` into a String.
    /// None if file doesn't exist or cannot be read.
    pub fn read_file(&self, path: &str) -> Option<String> {
        (self.read_file)(path)
    }

    /// Returns true if file exists at `path`.
    pub fn file_exists(&self, path: &str) -> bool {
        (self.file_exists)(path)
    }

    /// Returns true if file at `path` is a directory.
    pub fn file_is_directory(&self, path: &str) -> bool {
        (self.file_is_directory)(path)
    }

    /// Returns a joined path of `base` and `path`.
    pub fn path_join(&self, base: &str, path: &str) -> String {
        (self.path_join)(base, path)
    }

    /// Returns the parent directory of `path`.
    pub fn parent_directory(&self, path: &str) -> String {
        (self.parent_directory)(path)
    }

    /// Returns true if `path` is an absolute path.
    pub fn path_is_absolute(&self, path: &str) -> bool {
        (self.path_is_absolute)(path)
    }

    /// Get the parent directory of `source_path` and join it with `path`.
    pub fn import_path(&self, source_path: &str, path: &str) -> String {
        self.path_join(&self.parent_directory(source_path), path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read_file(path: &str) -> Option<String> {
        match path {
            "exists" => Some("content".to_owned()),
            _ => None,
        }
    }

    fn file_exists(path: &str) -> bool {
        path == "exists"
    }

    fn file_is_directory(path: &str) -> bool {
        path == "directory"
    }

    fn path_join(base: &str, path: &str) -> String {
        format!("{}/{}", base, path)
    }

    fn parent_directory(path: &str) -> String {
        match path {
            "hasParent" => "parent".to_owned(),
            _ => "".to_owned(),
        }
    }

    fn path_is_absolute(path: &str) -> bool {
        path.starts_with("/")
    }

    static FS_UTIL: FSUtil = FSUtil {
        read_file,
        file_exists,
        file_is_directory,
        path_join,
        parent_directory,
        path_is_absolute,
    };

    #[test]
    fn read_file_calls_the_argument_function() {
        assert_eq!(FS_UTIL.read_file("exists"), Some("content".to_owned()));
        assert_eq!(FS_UTIL.read_file("not_exists"), None);
    }

    #[test]
    fn file_exists_calls_the_argument_function() {
        assert!(FS_UTIL.file_exists("exists"));
        assert!(!FS_UTIL.file_exists("not_exists"));
    }

    #[test]
    fn file_is_directory_calls_the_argument_function() {
        assert!(FS_UTIL.file_is_directory("directory"));
        assert!(!FS_UTIL.file_is_directory("file"));
    }

    #[test]
    fn path_join_calls_the_argument_function() {
        assert_eq!(FS_UTIL.path_join("base", "path"), "base/path");
    }

    #[test]
    fn parent_directory_calls_the_argument_function() {
        assert_eq!(FS_UTIL.parent_directory("hasParent"), "parent");
        assert_eq!(FS_UTIL.parent_directory("noParent"), "");
    }

    #[test]
    fn path_is_absolute_calls_the_argument_function() {
        assert!(FS_UTIL.path_is_absolute("/home/user"));
        assert!(!FS_UTIL.path_is_absolute("home/user"));
    }

    #[test]
    fn import_path_calls_parent_directory_and_path_join() {
        assert_eq!(FS_UTIL.import_path("hasParent", "path"), "parent/path");
        assert_eq!(FS_UTIL.import_path("noParent", "path"), "/path");
    }
}
