use std::time;

use pyo3::prelude::*;
use pyo3::types::PyDelta;
use raliguard;
use ralipyard;


#[pyclass]
struct RateLimitGuard {
    py_semaphore: ralipyard::Semaphore
}


#[pymethods]
impl RateLimitGuard {
    #[new]
    fn new(semaphore: ralipyard::Semaphore) -> RateLimitGuard {
        RateLimitGuard {
            py_semaphore: semaphore
        }
    }
}
