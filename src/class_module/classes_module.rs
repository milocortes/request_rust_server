use pyo3::prelude::*;
use crate::serder_module::serder_module_rs::manda_mensage;

#[pyclass]
pub struct Message{
    #[pyo3(get, set)]
    value: f64,
    #[pyo3(get, set)]
    best_vector: Vec<f64>
}

#[pymethods]
impl Message {

    #[new]
    fn new(value: f64, best_vector: Vec<f64>) -> Self {

        return Message {value, best_vector}
    }


    #[staticmethod]
    fn process_numbers(){
        println!("saying hello from Rust!");
        manda_mensage();

    }
}

