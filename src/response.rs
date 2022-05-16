use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::exceptions::{PyKeyError, PyIndexError, PyTypeError};
use reqwest;

use crate::conversion;


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
    // TODO: refactor
    #[args(fields = "*")]
    fn select(&self, fields: Vec<conversion::PyIndex>) -> PyResult<PyObject> {
        let mut current_value = &self.content;
        for field in fields {
            match current_value {
                conversion::PySerde::Object(object) => {
                    match field {
                        conversion::PyIndex::Int(index) => return Err(
                            PyTypeError::new_err(
                                format!(
                                    r#"Cannot access the index ({})
                                    because accessible is an object with string keys"#,
                                    index
                                )
                            )
                        ),
                        conversion::PyIndex::Str(index) =>
                            match object.get(index.as_str()) {
                                None => return Err(
                                    PyKeyError::new_err(
                                        format!("No such key: {}", &index)
                                    )
                                ),
                                Some(new_value) => current_value = new_value
                            }
                    }
                },
                conversion::PySerde::Array(array) => {
                    match field {
                        conversion::PyIndex::Str(index) => return Err(
                            PyTypeError::new_err(
                                format!(
                                    r#"Cannot access the key ({})
                                    because accessible is an array"#,
                                    index
                                )
                            )
                        ),
                        conversion::PyIndex::Int(index) => match array.get(index) {
                            None => return Err(
                                PyIndexError::new_err(
                                    format!(
                                        "Index out of length: {} (length is {})",
                                        &index, array.len()
                                    )
                                )
                            ),
                            Some(new_value) => current_value = new_value
                        }
                    }
                },
                _ => return Err(
                    PyTypeError::new_err("Accessible is not subscriptable")
                )
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
