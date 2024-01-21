use crate::{file::FileIO, device::{Device, Info}};

mod vm;
mod interpreter;
mod bytecode;
mod device;
mod file;
mod global;
mod js;

fn main() {
    let named_file = file::NamedFile::new("hapt");
    let handle = named_file.file().handle();
    println!("{}", handle.info());
    println!("{}", named_file.info());

    js::init();
}