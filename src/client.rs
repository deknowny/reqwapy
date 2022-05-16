use std::collections::HashMap;

use pyo3::prelude::*;
use reqwest;

use crate::response;
use crate::conversion;

#[pyclass(module = "reqwapy_core.client")]
struct Client {

    rw_client: reqwest::Client,

    #[pyo3(get)]
    base_url: Option<String>
}

impl Client {
    fn build_url(&self, end_part: &str) -> String {
        match &self.base_url {
            None => String::from(end_part),
            Some(base) => {
                let mut new_url = String::new();
                new_url.push_str(&base);
                new_url.push_str(end_part);
                new_url
            }
        }
    }
}

#[pymethods]
impl Client {
    #[new]
    fn new(
        _py: Python,
        base_url: Option<String>
    ) -> PyResult<Self> {
        let rw_client = reqwest::Client::new();
        Ok(
            Client {
                rw_client,
                base_url,
            }
        )
    }

    fn request_json<'rt>(
        &self,
        method: &str,
        url: &str,
        query: Option<HashMap<String, conversion::PySerde>>,
        json: Option<HashMap<String, conversion::PySerde>>,
        data: Option<HashMap<String, conversion::PySerde>>,
        before_json_parsed_cb: Option<PyObject>,
        py: Python<'rt>
    ) -> PyResult<&'rt PyAny> {
        let client = self.rw_client.clone();
        let full_url = self.build_url(url);
        let http_method = match reqwest::Method::from_bytes(method.as_bytes()) {
            Ok(parsed_method) => parsed_method,
            _ => panic!("Error occured while parsing")
        };

        pyo3_asyncio::tokio::future_into_py(py, async move {
            let mut request = client.request(http_method, full_url);

            if let Some(passed_json) = json {
                request = request.json(&passed_json);
            }
            if let Some(passed_data) = data {
                request = request.form(&passed_data);
            }
            if let Some(passed_query) = query {
                request = request.query(&passed_query);
            }

            let response = request.send().await.unwrap();

            if let Some(cb) = before_json_parsed_cb {
                let fut = Python::with_gil(|py| {
                    let py_response = response::RawResponse::new(&response);
                    let nonpy_awaitable = cb.call1(py, (py_response,))?;
                    let awaitable = nonpy_awaitable.as_ref(py);
                    pyo3_asyncio::tokio::into_future(awaitable)
                })?;
                fut.await?;
            }

            let json: conversion::PySerde = response.json().await.unwrap();

            Ok(Python::with_gil(|py| response::JSONResponse::new(json).into_py(py)))
        })
    }
}

pub fn init_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let submod = PyModule::new(py, "client")?;
    submod.add_class::<Client>()?;
    parent_module.add_submodule(submod)?;
    Ok(())
}
