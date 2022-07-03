use pyo3::prelude::*;

#[pyclass]
#[derive(Clone)]
struct Interpreter {
    pub interpreter: ftd::Interpreter
}

#[pymethods]
impl Interpreter {
    pub fn hello(&self, name: String) -> String {
        println!("Printing name: {}", name);
        name
    }

    pub fn get_state(&self) -> String {
        match &self.interpreter {
            ftd::Interpreter::StuckOnProcessor {..} => "stuck_on_processor".to_string(),
            ftd::Interpreter::StuckOnForeignVariable {..} =>  "stuck_on_foreign_variable".to_string(),
            ftd::Interpreter::StuckOnImport {..} =>  "stuck_on_import".to_string(),
            ftd::Interpreter::Done {..} =>  "done".to_string(),
        }
    }

    pub fn get_module_to_import(&self) -> Option<String> {
        match &self.interpreter {
            ftd::Interpreter::StuckOnImport {module, ..} => Some(module.to_string()),
            _ => None
        }
    }

    pub fn continue_after_import(self_: Py<Interpreter>, py: pyo3::Python, id: &str, source: &str) {
        let d: Interpreter = self_.extract(py).unwrap();
        // Stuck here, because continue_after_import need object with ownership and from python
        // unable to pass an object with ownership, only referenced object can be passed
        // match &mut self.interpreter {
        //     ftd::Interpreter::StuckOnImport {ref mut state, ..} => {
        //         state.continue_after_import(id, source).unwrap();
        //     },
        //     _ => {}
        // };
    }


    pub fn get_foreign_variable_to_resolve(&self) -> String {
        todo!()
    }

    pub fn get_processor_section(&self) -> String {
        // returns json serialised section
        todo!()
    }

    pub fn continue_after_processor(&self, _value: &str) {
        // _value is json serialised value
    }
}



#[pyfunction]
fn interpret(name: &str, source: &str)-> PyResult<Interpreter>  {
    let s = ftd::interpret(name, source).map_err(|e| {
        eprintln!("{:?}", e);
        pyo3::exceptions::PyTypeError::new_err(e.to_string())
    })?;
    Ok(Interpreter {interpreter: s})
}

/// A Python module implemented in Rust.
#[pymodule]
fn ftd_sys(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(interpret, m)?)?;
    // m.add_function(wrap_pyfunction!(render, m)?)?;
    Ok(())
}

// #[pyfunction]
// fn render(
//     py: pyo3::Python,
//     root: Option<String>,
//     file: String,
//     base_url: String,
//     data: String,
// ) -> PyResult<&PyAny> {
//     // dbg!(&data, data.get_type(), data.get_type_ptr());
//     // for k in data.iter()? {
//     //     let g = k?;
//     //     dbg!(&g, data.get_item(g));
//     // }
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let config = {
//             let mut config = match fpm::Config::read(root.clone()).await {
//                 Ok(c) => c,
//                 Err(e) => {
//                     eprintln!("{:?}", e);
//                     return Ok(Python::with_gil(|py| py.None()));
//                 }
//             };
//             if config.attach_data_string(data.as_str()).is_err() {
//                 return Ok(Python::with_gil(|py| py.None()));
//             }
//             config
//         };
//         let html = match fpm::render(&config, file.as_str(), base_url.as_str()).await {
//             Ok(data) => data,
//             Err(e) => {
//                 eprintln!("{:?}", e);
//                 return Ok(Python::with_gil(|py| py.None()));
//             }
//         };
//         Ok(Python::with_gil(|py| html.into_py(py)))
//     })
// }

// #[pyfunction]
// fn fpm_build(
//     py: pyo3::Python,
//     root: Option<String>,
//     file: Option<String>,
//     base_url: Option<String>,
//     ignore_failed: Option<bool>,
// ) -> PyResult<&PyAny> {
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let config = match fpm::Config::read(root).await {
//             Ok(c) => c,
//             _ => {
//                 return Ok(Python::with_gil(|py| py.None()));
//             }
//         };
//         fpm::build(
//             &config,
//             file.as_deref(),
//             base_url.unwrap_or_else(|| "/".to_string()).as_str(), // unwrap okay because base is required
//             ignore_failed.unwrap_or(false),
//         )
//         .await
//         .ok();
//         Ok(Python::with_gil(|py| py.None()))
//     })
// }
