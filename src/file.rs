use std::{marker::PhantomPinned, borrow::Cow};

use crate::device::{Device, Interrupt, Info};

#[derive(Clone, Copy, Debug)]
pub struct Handle<'a>(pub usize, pub &'a PhantomPinned);

impl<'a> Handle<'a> {
    pub fn new() -> Option<&'a Self> {
        crate::global::push_handle()
    }

    pub fn from(x: usize) -> Option<&'a Self> {
        crate::global::register_handle(Handle(x, &PhantomPinned))
    }

    pub fn from_str(x: impl Into<String>) -> Option<&'a Self> {
        let mut str: String = x.into();
        crate::global::register_handle(Handle(str.as_mut_ptr() as usize, &PhantomPinned))
    }
}

impl<'a> Info for Handle<'a> {
    fn info(&self) -> Cow<str> {
        format!("[file-handle/info] {:?}", self).into()
    }
}

fn not_implemented<'a>() -> &'a Handle<'a> {
    Handle::from(0).expect("uninit")
}

#[derive(Clone, Copy, Debug)]
pub struct File<'a>(pub Handle<'a>);

impl<'a> File<'a> {
    pub fn new() -> Self {
        File(*Handle::new().expect(
            "failed to get file handle"))
    }
}

impl<'a> Device<'a> for File<'a> {
    fn get(&self) -> impl Device {
        *self
    }

    fn handle(&self) -> Handle {
        self.0
    }
}

impl<'a> Interrupt for File<'a> {
    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle> {
        todo!()
    }
}

impl<'a> Info for File<'a> {
    fn info(&self) -> Cow<str> {
        format!("[file/info] {:?}", self).into()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NamedFile<'a>(pub File<'a>, pub &'a str);

impl<'a> NamedFile<'a> {
    pub fn new(name: impl Into<&'a str>) -> Self{
        NamedFile(File::new(), name.into())
    }
}

impl<'a> Interrupt for NamedFile<'a> {
    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle> {
        FileIO::interrupt(self, code, data)
    }
}

impl<'a> Info for NamedFile<'a> {
    fn info(&self) -> Cow<str> {
        format!("[file/info] {:?}: {:?}", self.1, self.0).into()
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub enum FileOptions {
    Info = 0,
    Read = 1,
    Write = 2,
    Flush = 3
}

pub trait FileIO: Interrupt + Info {
    fn file(&self) -> &File;

    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle> {
        let option: FileOptions = unsafe { std::mem::transmute(code as u32) };
        match option {
            FileOptions::Info => {
                Ok(Handle::from_str(self.info()).expect("failed to get handle for string").to_owned())
            },
            FileOptions::Read => {
                // io:read
                // TODO: implement
                Err(Handle(0, &PhantomPinned))
            },
            FileOptions::Write => {
                // io:write
                // TODO: implement
                Ok(Handle(0, &PhantomPinned))
            },
            FileOptions::Flush => {
                // io:flush
                // TODO: implement
                Ok(Handle(0, &PhantomPinned))
            },
            x => Ok(*not_implemented())
        }
    }

    fn io(&self, option: FileOptions, data: Option<Handle>) -> Result<Handle, Handle> {
        let code: u32 = unsafe { std::mem::transmute(option) };
        FileIO::interrupt(self, code as usize, data)
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