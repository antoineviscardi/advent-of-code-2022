struct Filesystem {
    root: Directory,
}

struct Directory {
    name: String,
    files: Vec<File>,
}

enum File {
    Directory,
    Document,
}

impl Filesystem {
    /// Factory function that initializes a Filesystem with an empty root Directory.
    ///
    /// ```
    /// let filesystem = Filesystem::init();
    /// assert_eq!(filesystem.root.name, "/");
    /// assert_eq(filesystem.root.files.is_empty());
    ///
    pub fn init() -> Self {
        todo!()
    }
}
