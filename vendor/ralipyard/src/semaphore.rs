use std::{sync, time};

use pyo3::prelude::*;
use pyo3::create_exception;
use pyo3::types::{PyDelta, PyDeltaAccess};


create_exception!(raliguard, DeltaIsNegativeError, pyo3::exceptions::PyValueError);
create_exception!(raliguard, ExcessVerbosityError, pyo3::exceptions::PyValueError);


#[pyclass(name = "Semaphore")]
pub struct PySempahore {
    semaphore: sync::Arc<sync::Mutex<raliguard::Semaphore>>
}

#[pymethods]
impl PySempahore {
    #[new]
    pub fn new(access_times: u64, per_period: &PyDelta) -> PyResult<PySempahore> {
        let delta_secs = per_period.get_seconds();
        if delta_secs < 0 {
            return Err(DeltaIsNegativeError::new_err("Timedelta seconds cannot be negative"));
        }

        let delta_microseconds = per_period.get_microseconds();
        if delta_microseconds < 0 {
            return Err(DeltaIsNegativeError::new_err("Timedelta meicroseconds cannot be negative"));
        }

        if delta_microseconds == 0 && delta_secs == 0 {
            return Err(
                // TODO: Multiline
                ExcessVerbosityError::new_err(
                    "Rate limit semaphore does not make \
                    any sense with no required delay. \
                    Set a delay for datetime.timdelta"
                )
            );
        }

        let duration = time::Duration::new(delta_secs as u64, 1000*delta_microseconds as u32);
        Ok(PySempahore {
            semaphore: sync::Arc::new(
                sync::Mutex::new(
                    raliguard::Semaphore::new(access_times, duration)
                )
            )
        })
    }

    pub fn calc_delay(&self) -> f64 {
        let local_sem = self.semaphore.clone();
        let mut sem = local_sem.lock().unwrap();
        match sem.calc_delay() {
            None => 0.0,
            Some(delay) => delay.as_secs_f64()
        }
    }
}
