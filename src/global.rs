use std::marker::PhantomPinned;

use crate::file::Handle;

pub const MAX_HANDLES: usize = 1024 * 16;
static mut FILE_HANDLES: [Handle<'static>; MAX_HANDLES] = [Handle(0, &PhantomPinned); MAX_HANDLES];
static mut HANDLE_INDEX: usize = 0;

pub fn register_handle<'a>(x: Handle) -> Option<&'a Handle<'a>> {
    if unsafe { HANDLE_INDEX } < MAX_HANDLES {
        unsafe {
            let handle: *mut Handle = std::mem::transmute(FILE_HANDLES.get_unchecked_mut(HANDLE_INDEX));
            *handle = x;
            HANDLE_INDEX += 1;
            Some(FILE_HANDLES.get_unchecked(HANDLE_INDEX - 1))
        }
    } else {
        None
    }
}

pub fn push_handle<'a>() -> Option<&'a Handle<'a>> {
    register_handle(Handle(unsafe { HANDLE_INDEX }, &PhantomPinned))
}

pub fn pop_handle<'a>(index: usize) -> Option<Handle<'a>> {
    if  index < unsafe { HANDLE_INDEX } {
        unsafe {
            let handle = FILE_HANDLES.get_unchecked(index);
            for range in [index..HANDLE_INDEX] {
                let handle_to_overwrite = FILE_HANDLES.get_unchecked_mut(range.start);
                *handle_to_overwrite = *FILE_HANDLES.get_unchecked(range.start + 1);
            }
            FILE_HANDLES[HANDLE_INDEX - 1] = Handle(0, &PhantomPinned);
            Some(*handle)
        }
    } else { None }
}

pub fn handle<'a>(index: usize) -> Option<Handle<'a>> {
    unsafe { FILE_HANDLES.get(index).copied() }
}