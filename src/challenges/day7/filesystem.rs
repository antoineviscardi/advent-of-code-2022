pub struct Filesystem {
    current_dir: DirectoryFile,
    parent_dir: Option<DirectoryFile>,
}

impl Filesystem {
    pub fn init() -> Self {
        Filesystem {
            current_dir: DirectoryFile::new(String::from("/")),
            parent_dir: None,
        }
    }

    pub fn add_files(self, files: Vec<File>) {
        self.current_dir.files.append(files);
    }

    pub fn change_dir(self, name: &str) {}

    pub fn get_directories(&self) -> &DirectoryFile {
        todo!()
    }
}

#[cfg(test)]
mod filesystem_tests {
    use super::*;

    #[test]
    fn test_init() {
        let fs = Filesystem::init();
        assert_eq!(fs.current_dir.name, "/");
        assert!(fs.current_dir.files.is_empty());
        assert!(fs.parent_dir.is_none());
    }

    fn test_change_dir() {
        let 
    }
}

pub use file::*;
mod file {
    pub enum File {
        Directory(DirectoryFile),
        Document(DocumentFile),
    }

    pub struct DocumentFile {
        pub name: String,
        pub size: u32,
    }

    pub struct DirectoryFile {
        pub name: String,
        pub files: Vec<File>,
    }

    impl File {
        pub fn get_size(&self) -> u32 {
            match self {
                File::Directory(dir) => dir.get_size(),
                File::Document(doc) => doc.size,
            }
        }
    }

    impl DirectoryFile {
        pub fn new(name: String) -> Self {
            Self {
                name,
                files: Vec::new(),
            }
        }

        pub fn get_size(&self) -> u32 {
            self.files.iter().map(|f| f.get_size()).sum()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_size() {
        let mut dir = DirectoryFile::new(String::from("dir"));
        dir.files.push(File::Document(DocumentFile {
            name: String::from(""),
            size: 50,
        }));
        dir.files.push(File::Document(DocumentFile {
            name: String::from(""),
            size: 50,
        }));
        dir.files.push(File::Directory(DirectoryFile {
            name: String::from("subdir"),
            files: vec![
                File::Document(DocumentFile {
                    name: String::from(""),
                    size: 10,
                }),
                File::Document(DocumentFile {
                    name: String::from(""),
                    size: 10,
                }),
            ],
        }));

        assert_eq!(dir.get_size(), 120);
    }

    #[test]
    fn test_get_directories() {
        let mut fs = Filesystem::init();
    }
}
