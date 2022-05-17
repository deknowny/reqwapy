use std::collections::HashMap;

use pyo3::prelude::*;
use pyo3::exceptions::{PyKeyError, PyIndexError, PyTypeError};
use reqwest;

use crate::conversion;

#[derive(Clone)]
#[pyclass]
pub struct RawResponse {
    #[pyo3(get)]
    status: u16,

    #[pyo3(get)]
    version: String,

    #[pyo3(get)]
    final_url: String,

    #[pyo3(get)]
    headers: HashMap<String, String>
}

impl RawResponse {
    pub fn new(response: &reqwest::Response) -> PyResult<Self> {
        Ok(RawResponse {
            status: response.status().as_u16(),
            version: String::from(match response.version() {
                reqwest::Version::HTTP_09 => "HTTP/0.9",
                reqwest::Version::HTTP_10 => "HTTP/1.0",
                reqwest::Version::HTTP_11 => "HTTP/1.1",
                reqwest::Version::HTTP_2=> "HTTP/2.0",
                reqwest::Version::HTTP_3 => "HTTP/3.0",
                _ => unreachable!(),
            }),
            final_url: String::from(response.url().as_str()),
            headers: {
                let mut py_headers = HashMap::new();
                for (key, value) in response.headers().iter() {
                    py_headers.insert(
                        String::from(key.as_str()),
                        String::from(value.to_str().unwrap())
                    );
                }
                py_headers
            },
        })
    }
}

#[pyclass]
pub struct TextResponse {
    #[pyo3(get)]
    pub text: String,
    pub raw_response: RawResponse,
}


#[pyclass]
pub struct JSONResponse {
    pub content: conversion::PySerde,
    pub raw_response: RawResponse,
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
