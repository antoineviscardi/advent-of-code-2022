pub struct Filesystem {
    root: Directory,
}

impl Filesystem {
    pub fn init() -> Self {
        Filesystem {
            root: Directory::new("/"),
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

pub struct Directory {
    name: String,
    files: Vec<File>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Directory {
            name: name.to_string(),
            files: Vec::new(),
        }
    }
}

#[cfg(test)]
mod directory_test {
    use super::*;

    #[test]
    fn test_new_works() {
        let name = "cool dir bro";
        let dir = Directory::new(name);
        assert_eq!(dir.name, name.to_string());
        assert!(dir.files.is_empty());
    }
}

enum File {
    Directory,
    Document,
}
