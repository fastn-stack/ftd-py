use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn fpm_build(
    py: pyo3::Python,
    file: Option<String>,
    base_url: Option<String>,
    ignore_failed: Option<bool>,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let config = match fpm::Config::read().await {
            Ok(c) => c,
            _ => {
                return Ok(Python::with_gil(|py| py.None()));
            }
        };
        fpm::build(
            &config,
            file.as_deref(),
            base_url.unwrap_or("/".to_string()).as_str(), // unwrap okay because base is required
            ignore_failed.unwrap_or(false),
        )
        .await
        .ok();
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn ftd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(fpm_build, m)?)?;
    Ok(())
}
