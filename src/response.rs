use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::exceptions::PyKeyError;
use reqwest;
use serde_json;

use crate::conversion;


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
    content: conversion::PySerde,
}

impl JSONResponse {
    pub fn new(content: conversion::PySerde) -> Self {
        JSONResponse { content }
    }
}



#[pymethods]
impl JSONResponse {
    #[args(fields = "*")]
    fn select(&self, py: Python, fields: Vec<conversion::PyIndex>) -> PyResult<PyObject> {
        let mut current_value = &self.content;
        for field in fields {
            match current_value {
                conversion::PySerde::Object(object) => {
                    match field {
                        conversion::PyIndex::Int(_) => panic!("It's an object, not an array"),
                        conversion::PyIndex::Str(index) => current_value = object.get(index.as_str()).unwrap()
                    }
                },
                conversion::PySerde::Array(array) => {
                    match field {
                        conversion::PyIndex::Str(_) => panic!("It's an array, not an object"),
                        conversion::PyIndex::Int(index) => current_value = array.get(index).unwrap()
                    }
                },
                _ => panic!("It's not subscribtaleacdascda")
            };
        }
        Ok(Python::with_gil(|py| current_value.to_object(py)))
    }
}



pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "response")?;
    submod.add_class::<RawResponse>()?;
    submod.add_class::<TextResponse>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
