use libloading::{Library, Symbol};

use crate::reload_dylib::build_dylib;

pub static mut LIBRARY: Option<Library> = None;

type AddOne = fn(i32) -> i32;
pub static mut YOUR_FUNCTION: Option<Symbol<AddOne>> = None;

pub fn reload() {
    unsafe { unload(); }
    build_dylib("rustpy-dylib");
    unsafe { load(); }
}

unsafe fn load() {
    let lib = Library::new("target/debug/rustpy_dylib.dll").expect("Failed to load library");
    LIBRARY = Some(lib);

    let library_ref = LIBRARY.as_ref().expect("Library is empty");
    let func = library_ref.get(b"add_one").expect("Failed to load function");
    YOUR_FUNCTION = Some(func);
}

unsafe fn unload() {
    LIBRARY = None;
    YOUR_FUNCTION = None;
}

pub fn add_one(value: i32) -> i32 {
    unsafe { YOUR_FUNCTION.as_ref().expect("Function is empty")(value) }
}
