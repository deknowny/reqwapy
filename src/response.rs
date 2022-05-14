use std::sync::Arc;
use std::boxed::Box;
use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::types::{PyList};
use tokio::sync::Mutex;
use pyo3::exceptions::PyKeyError;
use pyo3_asyncio;
use reqwest;
use serde_json;


fn serde_value_to_pyobject(value: &serde_json::Value, py: Python) -> PyObject {
    match value {
        serde_json::Value::Null => Option::<isize>::None.to_object(py),
        serde_json::Value::Bool(inner) => inner.to_object(py),
        serde_json::Value::String(inner) => inner.to_object(py),
        serde_json::Value::Number(inner) => match inner.is_f64() {
            true => inner.as_f64().to_object(py),
            false => inner.as_i64().to_object(py)
        },
        serde_json::Value::Array(inner) =>
            inner
            .iter()
            .map(|x| serde_value_to_pyobject(x, py))
            .collect::<Vec<_>>()
            .to_object(py),
        serde_json::Value::Object(inner) => {
            let mut new_holder = HashMap::new();
            for (key, elem) in inner {
                new_holder.insert(key, serde_value_to_pyobject(elem, py));
            }
            new_holder.to_object(py)
        }
    }
}


#[pyclass]
pub struct RawResponse {
    #[pyo3(get)]
    status: u16
}

impl RawResponse {
    pub fn new(response: &reqwest::Response) -> Self {
        RawResponse {
            status: response.status().as_u16()
        }
    }
}

#[pyclass]
pub struct TextResponse {
    #[pyo3(get)]
    text: String
}

impl TextResponse {
    pub fn new(text: String) -> Self {
        TextResponse { text }
    }
}


#[pyclass]
pub struct JSONResponse {
    content: serde_json::Value,
}

impl JSONResponse {
    pub fn new(content: serde_json::Value) -> Self {
        JSONResponse { content }
    }
}



#[pymethods]
impl JSONResponse {
    fn select(&self, py: Python, fields: Vec<&str>) -> PyResult<PyObject> {
        let mut current_value = &self.content;
        for field in fields {
            match current_value.get(field) {
                None => return Err(
                    PyKeyError::new_err(format!("No such key: {}", field))
                ),
                Some(new_val) => current_value = new_val
            };
        }
        Ok(serde_value_to_pyobject(current_value, py))
    }
}



pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "response")?;
    submod.add_class::<RawResponse>()?;
    submod.add_class::<TextResponse>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
