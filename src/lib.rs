use pyo3::prelude::*;

mod qtmcore;
use qtmcore::qtmcore_inner::Core;

#[pymodule]
#[allow(unused_variables)]
fn libqtmcalc(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Core>()?;
    Ok(())
}
