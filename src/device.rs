use crate::file::Handle;

pub trait Device<'a>: Clone + Sized {
    fn get(&self) -> impl Device;
    fn handle(&self) -> Handle;
    fn interrupt(&self, code: usize, data: Option<Handle>) -> Result<Handle, Handle>;
}