
use pyo3::prelude::*;

pub mod semaphore;


#[pymodule]
fn ralipyard(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_class::<semaphore::PySempahore>()?;
    Ok(())
}
