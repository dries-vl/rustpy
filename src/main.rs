extern crate cpython;

use cpython::{Python, PyResult, py_module_initializer, py_fn, PyObject};

// Define a Rust structure
struct MyRustStruct {
    data: i32,
}

// Define a Rust function that will be exposed to Python
fn rust_function(py: Python, value: i32) -> PyResult<i32> {
    Ok(value * 2)
}

py_module_initializer!(myrustlib, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "rust_function", py_fn!(py, rust_function(value: i32)))?;
    Ok(())
});

fn main() {
    let gil = Python::acquire_gil();
    let py = gil.python();

    // Initialize your Rust library and add it to sys.modules
    // Initialize your Rust library and add it to sys.modules
    let myrustlib = unsafe { PyInit_myrustlib() };
    let myrustlib_object = unsafe { PyObject::from_borrowed_ptr(py, myrustlib) };
    let sys = py.import("sys").unwrap();
    let binding = sys.get(py, "modules").unwrap();
    let modules = binding.cast_as::<cpython::PyDict>(py).unwrap();
    modules.set_item(py, "myrustlib", myrustlib_object).unwrap();

    // Load the Python code that uses the Rust library
    py.run(r#"
import myrustlib
result = myrustlib.rust_function(42)
print(f"Result from Rust: {result}")
"#, None, None).unwrap();
}
