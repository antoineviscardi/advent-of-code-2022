pub struct Filesystem {
    root: DirectoryFile,
}

impl Filesystem {
    pub fn init() -> Self {
        Filesystem {
            root: DirectoryFile::new("/"),
        }
    }
}

#[cfg(test)]
mod filesystem_tests {
    use super::*;

    #[test]
    fn test_init_works() {
        let fs = Filesystem::init();
        assert_eq!(fs.root.name, "/");
        assert!(fs.root.files.is_empty());
    }
}

pub use file::File;
mod file {
    use super::*;

    pub enum File {
        Directory(DirectoryFile),
        Document(DocumentFile),
    }

    pub struct DirectoryFile {
        pub name: String,
        pub files: Vec<File>,
    }
    pub struct DocumentFile {
        pub name: String,
        pub size: u32,
    }

    impl File {
        pub fn get_size(self) -> u32 {
            match self {
                File::Directory(dir) => dir.get_size(),
                File::Document(doc) => doc.size,
            }
        }
    }

    impl DirectoryFile {
        pub fn get_size(self) -> u32 {
            todo!()
        }
    }
}

// mod directory {
//     use super::*;

//     impl DirectoryFile {
//         pub fn new(name: &str) -> Self {
//             DirectoryFile {
//                 name: name.to_string(),
//                 files: Vec::new(),
//             }
//         }

//         pub fn add_file(self, file: File) {}
//     }

//     #[cfg(test)]
//     mod directory_test {
//         use super::*;

//         #[test]
//         fn test_new() {
//             let name = "cool dir bro";
//             let dir = DirectoryFile::new(name);
//             assert_eq!(dir.name, name.to_string());
//             assert!(dir.files.is_empty());
//         }

//         #[test]
//         fn test_add_file() {
//             let dir = DirectoryFile::new("parent");
//             let file = File::Document(DocumentFile {
//                 name: "fileA".to_owned(),
//                 size: 1024,
//             });
//             dir.add_file(file);
//         }
//     }
// }
