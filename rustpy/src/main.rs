use std::collections::HashMap;
use std::{fs, io, thread};
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};

use cpython::{py_fn, py_module_initializer, PyObject, PyResult, Python};
use lazy_static::lazy_static;
use libloading::{Library, Symbol};

use rustpy_macros::measure_time;

use crate::button::Button;
use crate::button::button_result::ButtonResult;

mod button;
mod menu;

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

fn main() {

    let lib = unsafe { Library::new("target/debug/rustpy_dylib.dll") }.expect("Failed to load library");
    type AddOne = unsafe extern "C" fn(i32) -> i32;
    let mut add_one: Symbol<AddOne> = unsafe {
        lib.get(b"add_one").expect("Failed to load function")
    };
    drop(lib);

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
        let lib = unsafe { Library::new("target/debug/rustpy_dylib.dll") }.expect("Failed to load library");
        add_one = unsafe { lib.get(b"add_one").expect("Failed to load function") };
        let result = unsafe { add_one(15) };
        drop(lib); // lib is released by the process, can replace the dll with the new one
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

        // Respond with a predefined message
        println!("Your response: '{:?}'", result);
        py.run(input, None, None).unwrap();

        // Flush the stdout buffer to ensure immediate display
        io::stdout().flush().unwrap();
    }
}
