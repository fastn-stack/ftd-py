use pyo3::prelude::*;

#[pyfunction]
fn render(
    py: pyo3::Python,
    root: Option<String>,
    file: String,
    base_url: String,
    data: String,
) -> PyResult<&PyAny> {
    // dbg!(&data, data.get_type(), data.get_type_ptr());
    // for k in data.iter()? {
    //     let g = k?;
    //     dbg!(&g, data.get_item(g));
    // }
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let config = {
            let mut config = match fpm::Config::read(root.clone()).await {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{:?}", e);
                    return Ok(Python::with_gil(|py| py.None()));
                }
            };
            if config.attach_data_string(data.as_str()).is_err() {
                return Ok(Python::with_gil(|py| py.None()));
            }
            config
        };
        let html = match fpm::render(&config, file.as_str(), base_url.as_str()).await {
            Ok(data) => data,
            Err(e) => {
                eprintln!("{:?}", e);
                return Ok(Python::with_gil(|py| py.None()));
            }
        };
        Ok(Python::with_gil(|py| html.into_py(py)))
    })
}

#[pyfunction]
fn fpm_build(
    py: pyo3::Python,
    root: Option<String>,
    file: Option<String>,
    base_url: Option<String>,
    ignore_failed: Option<bool>,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let config = match fpm::Config::read(root).await {
            Ok(c) => c,
            _ => {
                return Ok(Python::with_gil(|py| py.None()));
            }
        };
        fpm::build(
            &config,
            file.as_deref(),
            base_url.unwrap_or_else(|| "/".to_string()).as_str(), // unwrap okay because base is required
            ignore_failed.unwrap_or(false),
        )
        .await
        .ok();
        Ok(Python::with_gil(|py| py.None()))
    })
}

/// A Python module implemented in Rust.
#[pymodule]
fn ftd_sys(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fpm_build, m)?)?;
    m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}
