use crate::{file::FileIO, device::{Device, Info}};

mod vm;
mod interpreter;
mod bytecode;
mod device;
mod file;
mod global;

fn main() {
    let named_file = file::NamedFile::new("hapt");
    let file = named_file.file();
    let handle = file.handle();
    println!("{}", handle.info());
    println!("{}", file.info());
    println!("{}", named_file.info());
}
