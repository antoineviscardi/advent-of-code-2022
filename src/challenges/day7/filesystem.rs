struct Filesystem {
    root: Directory,
}

struct Directory {
    name: String,
    files: Vec<File>,
    directories: Vec<Directory>,
}

struct File {
    name: String,
    size: u32,
}

impl Filesystem {
    pub fn new() -> Self {
        let root = Directory::new(String::from("/"));
        Filesystem { root }
    }
}

impl Directory {
    pub fn new(name: String) -> Self {
        Directory {
            name,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    pub fn compute_size(&self) -> u32 {
        let files_size: u32 = self.files.iter().map(|f| f.size).sum();
        let directories_size: u32 = self.directories.iter().map(|d| d.compute_size()).sum();
        files_size + directories_size
    }
}
