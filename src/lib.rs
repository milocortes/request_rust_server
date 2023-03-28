#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod class_module;
mod serder_module;

use class_module::classes_module::Message;
use serder_module::serder_module_rs::__pyo3_get_function_send_request_py;


#[pymodule]
fn request_server(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Message>()?;
    m.add_wrapped(wrap_pyfunction!(send_request_py))?;
    Ok(())
}