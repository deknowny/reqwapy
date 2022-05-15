use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use pyo3::prelude::*;


#[derive(Debug, FromPyObject, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PySerde {
    Object(HashMap<String, PySerde>),
    Number(isize),
    Float(f64),
    String(String),
    Boolean(bool),
    Null(Option<isize>),
    Array(Vec<PySerde>),
}

impl ToPyObject for PySerde {
    fn to_object(&self, py: Python<'_>) -> PyObject {
        match self {
            Self::Null(_) => Option::<isize>::None.to_object(py),
            Self::Boolean(inner) => inner.to_object(py),
            Self::String(inner) => inner.to_object(py),
            Self::Float(inner) => inner.to_object(py),
            Self::Number(inner) => inner.to_object(py),
            Self::Array(inner) => {
                let mut new_holder = vec![];
                for item in inner {
                    new_holder.push(item.to_object(py));
                }
                new_holder.to_object(py)
            }
            Self::Object(inner) => {
                let mut new_holder = HashMap::new();
                for (key, elem) in inner {
                    new_holder.insert(key, elem.to_object(py));
                }
                new_holder.to_object(py)
            }
        }
    }
}


#[derive(Debug, FromPyObject)]
pub enum PyIndex {
    Int(usize),
    Str(String)
}
