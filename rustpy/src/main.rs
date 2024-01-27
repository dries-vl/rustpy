#![feature(lazy_cell)]

use std::{fs, io, thread};
use std::collections::HashMap;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use std::sync::{Arc, mpsc, Mutex};
use std::time::SystemTime;

use cpython::{py_fn, py_module_initializer, PyObject, PyResult, Python};
use lazy_static::lazy_static;

use rustpy_macros::measure_time;

use crate::button::Button;
use crate::button::button_result::ButtonResult;
use crate::dylib_reloader::start_http_server;

mod button;
mod menu;
mod dylib_reloader;
mod reload_dylib;
mod my_dy_lib;

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
    // Create a channel
    let (sender, receiver) = mpsc::channel();

    // Start the HTTP server in a new thread with the sender
    my_dy_lib::reload();
    start_http_server(sender);

    let receiver = Arc::new(Mutex::new(receiver));

    // Worker thread
    let worker_receiver = Arc::clone(&receiver);
    thread::spawn(move || {
        while let Ok(msg) = worker_receiver.lock().unwrap().recv() {
            // Process message
            if msg == "reload" {
                my_dy_lib::reload();
            }
        }
    });

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

        let result = my_dy_lib::add_one(15);

        // Respond with a predefined message
        println!("Your response: '{:?}'", result);
        py.run(input, None, None).unwrap();

        // Flush the stdout buffer to ensure immediate display
        io::stdout().flush().unwrap();
    }
}
