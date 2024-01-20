use std::borrow::Cow;

use crate::file::{Handle, not_implemented};

pub trait Device<'a>: Clone + Sized {
    fn get(&self) -> impl Device;
    fn handle(&self) -> Handle;
}

pub trait Interrupt {
    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle> {
        eprintln!("unhandled interrupt code: {}, data: {:?}", code, data);
        Err(*not_implemented())
    }
}

pub trait Info {
    fn info<'a>(&self) -> Cow<str>;
}

pub trait Named<'a> {
    fn name(&'a self) -> &'a str;
}