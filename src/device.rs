use std::borrow::Cow;

use crate::file::Handle;

pub trait Device<'a>: Clone + Sized {
    fn get(&self) -> impl Device;
    fn handle(&self) -> Handle;
}

pub trait Interrupt {
    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle>;
}

pub trait Info {
    fn info<'a>(&self) -> Cow<str>;
}

pub trait Named<'a> {
    fn name(&'a self) -> &'a str;
}