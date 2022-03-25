
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn fpm_build(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py_with_locals(
        py,
        pyo3_asyncio::tokio::get_current_locals(py)?,
        async move {
            let config = match fpm::Config::read().await {
                Ok(c) => c,
                _ => {return  Ok(());}
            };
            fpm::build(
                &config,
                None,
                "/", // unwrap okay because base is required
                false,
            ).await.ok();
            Ok(())
        })

}

/// A Python module implemented in Rust.
#[pymodule]
fn ftd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(fpm_build, m)?)?;
    Ok(())
}