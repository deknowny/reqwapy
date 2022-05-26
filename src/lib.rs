use pyo3::prelude::*;

pub mod client;
pub mod response;
pub mod conversion;
pub mod rate_limit;


#[pymodule]
fn reqwapy(py: Python, module: &PyModule) -> PyResult<()> {
    client::init_module(py, module)?;
    response::init_module(py, module)?;
    Ok(())
}
