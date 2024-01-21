extern crate cpython;

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

use cpython::{py_fn, py_module_initializer, PyObject, PyResult, Python};
use lazy_static::lazy_static;

use rustpy_macros::measure_time;

#[derive(Debug)]
#[repr(C)]
struct Button {
    id: String,
    // Other fields
}

// todo: add field
impl Button {
    fn new(id: String) -> Self {
        Self { id }
    }

    fn do_something(&self) {
        println!("Doing something with button {}", self.id);
    }
}

lazy_static! {
    static ref BUTTONS: Arc<Mutex<HashMap<String, Button>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[measure_time]
fn add_button(button: Button) {
    let mut buttons = BUTTONS.lock().unwrap();
    buttons.insert(button.id.clone(), button);
}

fn operate_on_button(id: &str) -> Option<()> {
    let buttons = BUTTONS.lock().unwrap();
    if let Some(button) = buttons.get(id) {
        button.do_something();
        Some(())
    } else {
        None
    }
}

// Define a Rust function that will be exposed to Python
fn rust_function(_py: Python, value: i32) -> PyResult<i32> {
    operate_on_button("button1");
    Ok(value * 2)
}

py_module_initializer!(myrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rust_function", py_fn!(py, rust_function(value: i32)))?;
    Ok(())
});

fn main() {

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
    let python_script = fs::read_to_string("rustpy/python_scripts/test.py")
        .expect("Failed to read Python file");

    // Run the python code on the interpreter
    py.run(&*python_script, None, None).unwrap();
}
