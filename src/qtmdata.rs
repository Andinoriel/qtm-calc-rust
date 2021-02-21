#[allow(dead_code)]
use pyo3::prelude::*;

use crate::qtmcore::Qtm;

#[pyclass]
pub struct QtmData {}

#[pymethods]
impl QtmData {}

#[cfg(test)]
mod tests {}
