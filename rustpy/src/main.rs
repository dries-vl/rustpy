use std::{fs, io};
use std::collections::HashMap;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;

use cpython::{py_fn, py_module_initializer, PyObject, PyResult, Python};
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

use rustpy_macros::measure_time;

use crate::button::Button;
use crate::button::button_result::ButtonResult;
use crate::dylib_reloader::start_http_server;

mod button;
mod menu;
mod dylib_reloader;

const CONSTANT: i32 = 1;

lazy_static! {
    static ref BUTTONS: Arc<Mutex<HashMap<String, Button>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[measure_time]
fn add_button(button: Button) -> ButtonResult {
    let mut buttons = BUTTONS.lock().unwrap();
    buttons.insert(button.id.clone(), button);
    ButtonResult::BROKEN
}

fn operate_on_button(id: &str) -> Option<i32> {
    let buttons = BUTTONS.lock().unwrap();
    if let Some(button) = buttons.get(id) {
        button.do_something();
        Some(CONSTANT)
    } else {
        None
    }
}

// Define a Rust function that will be exposed to Python
fn rust_function(_py: Python, value: i32) -> PyResult<i32> {
    operate_on_button("button1");
    Ok(value * rustpy_dylib::add_one(1))
}

py_module_initializer!(myrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rust_function", py_fn!(py, rust_function(value: i32)))?;
    Ok(())
});

/// Adds two numbers together.
///
/// # Examples
///
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub const fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn get_last_modified_time(file_path: &str) -> SystemTime {
    fs::metadata(file_path)
        .and_then(|metadata| metadata.modified())
        .unwrap_or(SystemTime::UNIX_EPOCH)
}

fn main() {
    start_http_server();

    // Initialize the dylib
    let mut lib = unsafe { Library::new("target/debug/rustpy_dylib.dll") }.expect("Failed to load library");
    let mut last_modified = SystemTime::UNIX_EPOCH;
    type AddOne = unsafe extern "C" fn(i32) -> i32;
    let mut add_one: Symbol<AddOne> = unsafe { lib.get(b"add_one").expect("Failed to load function") };

    add_button(Button::new("button1".to_string()));
    add_button(Button::new("button2".to_string()));

    let gil = Python::acquire_gil();
    let py = gil.python();

    // Initialize your Rust library and add it to sys.modules
    let myrustlib = unsafe { PyInit_myrustlib() };
    let myrustlib_object = unsafe { PyObject::from_borrowed_ptr(py, myrustlib) };
    let sys = py.import("sys").unwrap();
    let binding = sys.get(py, "modules").unwrap();
    let modules = binding.cast_as::<cpython::PyDict>(py).unwrap();
    modules.set_item(py, "myrustlib", myrustlib_object).unwrap();

    // Load the Python code that uses the Rust library
    let python_script = fs::read_to_string(Path::new("rustpy/python_scripts/test.py"))
        .expect("Failed to read Python file");

    // Run the python code on the interpreter
    py.run(&*python_script, None, None).unwrap();

    // Example CLI interface
    loop {
        println!("Please enter a python command (or type 'exit' to quit):");

        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Trim the newline character at the end
        let input = input.trim();

        // Check for 'exit' command
        if input.eq_ignore_ascii_case("exit") {
            break;
        }

        // Reload the dylib
        let current_modified = get_last_modified_time("target/debug/rustpy_dylib.dll");
        if current_modified > last_modified {
            // Drop the old library and load the new one
            drop(lib);
            lib = unsafe { Library::new("target/debug/rustpy_dylib.dll") }.expect("Failed to load library");
            add_one = unsafe { lib.get(b"add_one").expect("Failed to load function") };
            last_modified = current_modified;
        }
        let result = unsafe { add_one(15) };

        // Respond with a predefined message
        println!("Your response: '{:?}'", result);
        py.run(input, None, None).unwrap();

        // Flush the stdout buffer to ensure immediate display
        io::stdout().flush().unwrap();
    }
}
