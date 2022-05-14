use pyo3::prelude::*;
use reqwest;

use crate::response;

#[pyclass(module = "reqwapy_core.client")]
struct Client {

    rw_client: reqwest::Client,

    #[pyo3(get)]
    base_url: Option<String>
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

    fn request_json<'rt>(&self, url: &str, before_json_parsed_cb: Option<PyObject>, py: Python<'rt>) -> PyResult<&'rt PyAny> {
        let client = self.rw_client.clone();


        let full_url = match &self.base_url {
            None => String::from(url),
            Some(base) => {
                let mut new_url = String::new();
                new_url.push_str(&base);
                new_url.push_str(url);
                new_url
            }
        };
        pyo3_asyncio::tokio::future_into_py(py, async move {
            let response = client
                .request(reqwest::Method::POST, full_url)
                .send()
                .await.unwrap();

            if let Some(cb) = before_json_parsed_cb {
                let fut = Python::with_gil(|py| {
                    let py_response = response::RawResponse::new(&response);
                    let nonpy_awaitable = cb.call1(py, (py_response,))?;
                    let awaitable = nonpy_awaitable.as_ref(py);
                    pyo3_asyncio::tokio::into_future(awaitable)
                })?;
                fut.await?;
            }

            let json = response.json().await.unwrap();

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
