use pyo3::prelude::*;

mod qtmcore;
mod qtmdata;
mod utility;

use qtmcore::Qtm;
use qtmdata::QtmData;

#[pymodule]
#[allow(unused_variables)]
fn libqtmcalc(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Qtm>()?;
    m.add_class::<QtmData>()?;
    Ok(())
}
