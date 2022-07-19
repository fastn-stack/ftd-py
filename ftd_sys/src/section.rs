use pyo3::prelude::*;

#[pyclass]
pub struct Section {
    pub section: ftd::p1::Section,
}

#[pyclass]
pub struct SubSection {
    pub section: ftd::p1::SubSection,
}

#[pymethods]
impl Section {
    pub fn caption(&self, doc_id: &str) -> PyResult<String> {
        self.section
            .caption(self.section.line_number, doc_id)
            .map_err(|e| crate::py_err(&e.to_string()))
    }

    pub fn body(&self, doc_id: &str) -> PyResult<String> {
        self.section
            .body(self.section.line_number, doc_id)
            .map_err(|e| crate::py_err(&e.to_string()))
    }

    pub fn header_string(&self, doc_id: &str, name: &str) -> PyResult<String> {
        self.section
            .header
            .string(doc_id, self.section.line_number, name)
            .map_err(|e| crate::py_err(&e.to_string()))
    }
}
