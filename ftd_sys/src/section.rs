use pyo3::prelude::*;

#[pyclass]
pub struct Section {
    pub section: ftd::p1::Section,
}

#[pyclass]
pub struct SubSection {
    pub section: ftd::p1::SubSection,
}
