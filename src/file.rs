use std::marker::PhantomPinned;

use crate::device::Device;

#[derive(Clone, Copy, Debug)]
pub struct Handle<'a>(pub usize, pub &'a PhantomPinned);

impl<'a> Handle<'a> {
    fn new() -> Option<&'a Self> {
        crate::global::push_handle()
    }

    fn from(x: usize) -> Option<&'a Self> {
        crate::global::register_handle(Handle(x, &PhantomPinned))
    }
}

#[derive(Clone, Copy, Debug)]
pub struct File<'a>(pub Handle<'a>);

impl<'a> File<'a> {
    fn new() -> Self {
        File(*Handle::new().expect("failed to get file handle"))
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FileOptions {
    Read = 0,
    Write = 1,
    Flush = 2
}

impl<'a> Device<'a> for File<'a> {
    fn get(&self) -> impl Device {
        *self
    }

    fn handle(&self) -> Handle {
        self.0
    }

    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle> {
        let option: FileOptions = unsafe { std::mem::transmute(code as u32) };
        match option {
            FileOptions::Read => {
                // io:read
                // TODO: implement
                Ok(Handle(0, &PhantomPinned))
            },
            FileOptions::Write => {
                // io:write
                // TODO: implement
                Ok(self.0)
            },
            FileOptions::Flush => {
                // io:flush
                // TODO: implement
                Ok(self.0)
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NamedFile<'a>(pub File<'a>, pub &'a str);

impl<'a> NamedFile<'a> {
    fn new(name: impl Into<&'a str>) -> Self{
        NamedFile(File::new(), name.into())
    }
}

pub trait FileIO {
    fn file(&self) -> &File;

    fn io(&self, option: FileOptions, data: Option<Handle>) -> Result<Handle, Handle> {
        let code: u32 = unsafe { std::mem::transmute(option) };
        self.file().interrupt(code as usize, data)
    }

    fn read(&self, data: Handle) -> Result<Handle, Handle> {
        self.io(FileOptions::Read, Some(data))
    }

    fn write(&self, data: Handle) -> Result<Handle, Handle> {
        self.io(FileOptions::Write, Some(data))
    }

    fn flush(&self) -> Result<Handle, Handle> {
        self.io(FileOptions::Flush, None)
    }
}

impl<'a> FileIO for File<'a> {
    fn file(&self) -> &File {
        self
    }
}

impl<'a> FileIO for NamedFile<'a> {
    fn file(&self) -> &File {
        &self.0
    }
}

pub trait Named<'a> {
    fn name(&'a self) -> &'a str;
}

impl<'a> Named<'a> for NamedFile<'a> {
    fn name(&'a self) -> &'a str {
        self.1
    }
}