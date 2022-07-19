use pyo3::prelude::*;

// TODO: Probably not required, because application will not have `ftd_sys::Header`, they will have
// ftd::Header, because it is coming from ftd::Section

#[pyclass]
pub struct Header {
    header: ftd::p1::Header,
}

impl Header {
    pub fn string(&self, doc_id: &str, line_number: usize, name: &str) -> PyResult<String> {
        self.header
            .string(doc_id, line_number, name)
            .map_err(|e| crate::py_err(&e.to_string()))
    }
}
