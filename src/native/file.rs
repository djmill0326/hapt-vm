use std::fs;
use crate::file::{Handle, FileIO};

struct NativeFile(fs::File, Handle);
static mut NATIVE_FILE_HANDLES: Vec<NativeFile> = Vew::new();

impl NativeFile {
    pub fn open<'a>(name: From<&'a str>) {
        let file = fs::File::open(name);
        let handle = Handle::new();
    }
}

impl<'a> FileIO for NativeFile {
    fn file(&self) -> &File {
        self
    }
}

impl<'a> Named<'a> for NamedFile<'a> {
    fn name(&'a self) -> &'a str {
        self.1
    }
}