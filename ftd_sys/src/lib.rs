use pyo3::prelude::*;

#[pyclass]
pub struct Section {
    pub section: ftd::p1::Section
}

#[pyclass]
pub struct SubSection {
    pub section: ftd::p1::SubSection
}

#[pyclass]
struct Interpreter {
    document_id: String,
    config: fpm::Config,
    interpreter: std::cell::RefCell<Option<ftd::Interpreter>>
}

#[pymethods]
impl Interpreter {
    pub fn hello(&self, name: String) -> String {
        println!("Printing name: {}", name);
        name
    }

    pub fn state_name(&self) -> Option<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = &*interpreter {
            return Some(match i {
                ftd::Interpreter::StuckOnProcessor { .. } => "stuck_on_processor".to_string(),
                ftd::Interpreter::StuckOnForeignVariable { .. } => "stuck_on_foreign_variable".to_string(),
                ftd::Interpreter::StuckOnImport { .. } => "stuck_on_import".to_string(),
                ftd::Interpreter::Done { .. } => "done".to_string(),
            })
        }
        None
    }

    pub fn get_module_to_import(&self) -> Option<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = &*interpreter {
            return match i {
                ftd::Interpreter::StuckOnImport { module, .. } => Some(module.to_string()),
                _ => None
            }
        }
        None
    }

    pub fn continue_after_import(&self, id: &str, source: &str) {
        let interpreter = self.interpreter.replace(None).unwrap(); // TODO:
        match interpreter {
            ftd::Interpreter::StuckOnImport {state, ..} => {
                let new_interpreter = state.continue_after_import(id, source).unwrap(); // TODO: remove unwrap
                self.interpreter.replace(Some(new_interpreter));
            },
            _ => {}
        };
    }

    pub fn get_processor_section(&self) -> Option<Section> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnProcessor { section, .. } => Some(Section { section: section.clone()}), // TODO: Remove unwrap
                _ => None
            }
        }
        None
    }

    pub fn resolve_processor(&self, section: &Section) {
        let interpreter = self.interpreter.borrow();
        let state = if let Some(i) = interpreter.as_ref() {
            match i {
                ftd::Interpreter::StuckOnProcessor { state, .. } => Some(state),
                _ => unimplemented!(""),
            }
        } else { None };

        let state = state.unwrap(); // TODO:

        fpm::library::process_sync(
            &self.config,
            &section.section,
            &self.document_id,
            &state.tdoc(&mut Default::default())).unwrap();
    }


    pub fn continue_after_processor(&self, _value: &str) {
        // let section = serde_json::from_str().unwrap();
        // _value is json serialised value
    }

    pub fn get_foreign_variable_to_resolve(&self) -> String {
        todo!()
    }
}



fn fpm_config() -> fpm::Config {
    let config = futures::executor::block_on(fpm::Config::read2(None, false));
    config.unwrap() //TODO: Handle Unwrap
}

#[pyfunction]
fn interpret(name: &str, source: &str)-> PyResult<Interpreter>  {
    let s = ftd::interpret(name, source).map_err(|e| {
        eprintln!("{:?}", e);
        pyo3::exceptions::PyTypeError::new_err(e.to_string())
    })?;
    Ok(Interpreter {
        document_id: name.to_string(),
        interpreter: std::cell::RefCell::new(Some(s)),
        config: fpm_config()
    })
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
