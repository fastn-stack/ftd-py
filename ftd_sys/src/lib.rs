use pyo3::prelude::*;

#[pyclass]
pub struct Section {
    pub section: ftd::p1::Section,
}

#[pyclass]
pub struct Config {
    pub config: fpm::Config,
}

#[pyclass]
pub struct SubSection {
    pub section: ftd::p1::SubSection,
}

#[pyclass]
#[derive(Clone)]
pub struct FtdValue {
    pub value: ftd::Value,
}

#[pyclass]
struct Interpreter {
    document_id: String,
    config: fpm::Config,
    interpreter: std::cell::RefCell<Option<ftd::Interpreter>>,
    library: std::cell::RefCell<fpm::Library2>,
}

#[pymethods]
impl Interpreter {
    pub fn hello(&self, name: String) -> String {
        println!("Printing name: {}", name);
        name
    }

    pub fn state_name(&self) -> Option<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return Some(match i {
                ftd::Interpreter::StuckOnProcessor { .. } => "stuck_on_processor".to_string(),
                ftd::Interpreter::StuckOnForeignVariable { .. } => {
                    "stuck_on_foreign_variable".to_string()
                }
                ftd::Interpreter::StuckOnImport { .. } => "stuck_on_import".to_string(),
                ftd::Interpreter::Done { .. } => "done".to_string(),
            });
        }
        None
    }

    pub fn get_module_to_import(&self) -> Option<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnImport { module, .. } => Some(module.to_string()),
                _ => None,
            };
        }
        None
    }

    pub fn continue_after_import(&self, id: &str, source: Option<String>) {
        let source = source.unwrap_or_else(|| "".to_string());
        let interpreter = self.interpreter.replace(None).unwrap(); // TODO:
        match interpreter {
            ftd::Interpreter::StuckOnImport { state, .. } => {
                let new_interpreter = state.continue_after_import(id, source.as_str()).unwrap(); // TODO: remove unwrap
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {}
        };
    }

    pub fn get_processor_section(&self) -> Option<Section> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnProcessor { section, .. } => Some(Section {
                    section: section.clone(),
                }), // TODO: Remove unwrap
                _ => None,
            };
        }
        None
    }

    // PyResult<Option<FtdValue>>
    pub fn resolve_processor(&self, section: &Section) -> FtdValue {
        let interpreter = self.interpreter.borrow();
        let state = if let Some(i) = interpreter.as_ref() {
            match i {
                ftd::Interpreter::StuckOnProcessor { state, .. } => Some(state),
                // TODO: Convert it into error
                _ => unimplemented!("this should not get called something is wrong"),
            }
        } else {
            None
        };

        let state = state.unwrap(); // TODO:

        let value = fpm::library::process_sync(
            &self.config,
            &section.section,
            &self.document_id,
            &state.tdoc(&mut Default::default()),
        )
        .unwrap();
        FtdValue { value }
    }

    pub fn resolve_import(&self, module: &str) -> PyResult<String> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let mut library = self.library.borrow_mut();
            let mut interpreter = self.interpreter.borrow_mut();
            let state = if let Some(ref mut i) = *interpreter {
                match i {
                    ftd::Interpreter::StuckOnImport { ref mut state, .. } => state,
                    _ => {
                        return Err(pyo3::exceptions::PyException::new_err(
                            "only stuck_on_import expected",
                        ))
                    }
                }
            } else {
                return Err(pyo3::exceptions::PyException::new_err(
                    "interpreter_expected",
                ));
            };
            println!("resolving import: {}", module);
            let d = fpm::resolve_import(&mut library, state, module)
                .await
                .map_err(|e| {
                    eprintln!("Error: fpm-resolve-import {:?}", e);
                    pyo3::exceptions::PyException::new_err(e.to_string())
                })?;
            println!("import resolved: {}", module);
            Ok(d)
        })
    }

    pub fn continue_after_processor(&self, value: FtdValue) -> PyResult<()> {
        let interpreter = self.interpreter.replace(None).unwrap(); // TODO:
        match interpreter {
            ftd::Interpreter::StuckOnProcessor { state, section } => {
                let new_interpreter = state
                    .continue_after_processor(&section, value.value)
                    .unwrap(); // TODO: remove unwrap
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {
                return Err(pyo3::exceptions::PyException::new_err(
                    "continue-after-processor, this should not get called",
                ))
            }
        };
        Err(pyo3::exceptions::PyException::new_err(
            "continue-after-processor, something bad wrong",
        ))
    }

    pub fn get_foreign_variable_to_resolve(&self) -> PyResult<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnForeignVariable { variable, .. } => {
                    Ok(variable.to_string())
                }
                _ => {
                    return Err(pyo3::exceptions::PyException::new_err(
                        "stuck-on-fv, this should not get called",
                    ))
                }
            };
        }
        Err(pyo3::exceptions::PyException::new_err(
            "stuck-on-fv, something bad wrong",
        ))
    }

    pub fn resolve_foreign_variable(
        &self,
        variable: String,
        base_url: Option<String>,
    ) -> PyResult<FtdValue> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().unwrap();

        rt.block_on(async {
            let base_url = base_url.unwrap_or_else(|| "/".to_string());
            let mut library = self.library.borrow_mut();
            let interpreter = self.interpreter.borrow();
            let state = if let Some(ref i) = interpreter.as_ref() {
                match i {
                    ftd::Interpreter::StuckOnForeignVariable { ref state, .. } => state,
                    _ => {
                        return Err(pyo3::exceptions::PyException::new_err(
                            "resolve_foreign_variable only StuckOnForeignVariable expected",
                        ))
                    }
                }
            } else {
                return Err(pyo3::exceptions::PyException::new_err(
                    "resolve_foreign_variable interpreter_expected",
                ));
            };
            println!("resolving foreign variable: {}", variable);
            let d = fpm::resolve_foreign_variable2(
                &variable,
                &self.document_id,
                state,
                &mut library,
                base_url.as_str(),
                false,
            )
            .await
            .map_err(|e| {
                eprintln!("Error: fpm-foreign-variable {:?}", e);
                pyo3::exceptions::PyException::new_err(e.to_string())
            })?;
            println!("foreign variable: {}", variable);
            Ok(FtdValue { value: d })
        })
    }

    pub fn continue_after_foreign_variable(&self, variable: &str, value: FtdValue) -> PyResult<()> {
        let interpreter = self.interpreter.replace(None).unwrap(); // TODO:
        match interpreter {
            ftd::Interpreter::StuckOnForeignVariable { state, .. } => {
                let new_interpreter = state
                    .continue_after_variable(variable, value.value)
                    .unwrap(); // TODO: remove unwrap
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {
                return Err(pyo3::exceptions::PyException::new_err(
                    "continue-after-foreign-variable, this should not get called",
                ))
            }
        };
        Ok(())
    }

    // From Option to Result
    pub fn render(&self) -> Option<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            match i {
                ftd::Interpreter::Done { document } => {
                    let doc_title = match &document.title() {
                        Some(x) => x.original.clone(),
                        _ => self.document_id.to_string(),
                    };
                    let ftd_doc = document.to_rt("main", &self.document_id);
                    let file_content = fpm::utils::replace_markers(
                        fpm::ftd_html(),
                        &self.config,
                        &fpm::utils::id_to_path(&self.document_id),
                        doc_title.as_str(),
                        "/",
                        &ftd_doc,
                    );
                    return Some(file_content);
                }
                // TODO: Convert it into error
                _ => unimplemented!("this should not get called something is wrong"),
            }
        }
        None
    }
}

fn fpm_config(root: Option<String>) -> PyResult<Config> {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        fpm::Config::read2(root, false)
            .await
            .map(|config| Config { config })
            .map_err(|err| {
                eprintln!("fpm_config {:?}", err);
                pyo3::exceptions::PyTypeError::new_err(err.to_string())
            })
    })
}

// TODO: Result<String>
fn file_content(config: &fpm::Config, id: &str) -> std::result::Result<String, String> {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let file = config
            .get_file_by_id(id, &config.package)
            .await
            .map_err(|e| e.to_string())?;
        match file {
            fpm::File::Ftd(f) => Ok(f.content),
            _ => Err("This block should not executed".to_string()),
        }
    })
}

#[pyfunction]
fn interpret(
    id: &str,
    root: Option<String>,
    base_url: Option<String>,
    _data: Option<String>,
) -> PyResult<Interpreter> {
    let config = fpm_config(root)?;
    let source = file_content(&config.config, id)
        .map_err(|err| pyo3::exceptions::PyTypeError::new_err(err.to_string()))?;

    let s = ftd::interpret(id, &source).map_err(|e| {
        eprintln!("{:?}", e);
        pyo3::exceptions::PyTypeError::new_err(e.to_string())
    })?;

    Ok(Interpreter {
        document_id: id.to_string(),
        interpreter: std::cell::RefCell::new(Some(s)),
        config: config.config.clone(),
        library: std::cell::RefCell::new(fpm::library::Library2 {
            packages_under_process: vec![config.config.package.name.to_string()],
            config: config.config,
            markdown: None,
            document_id: id.to_string(),
            translated_data: Default::default(),
            base_url: base_url.unwrap_or_else(|| "/".to_string()),
        }),
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
// fn interpret<'a>(py: pyo3::Python<'a>, name: &'a str, source: &'a str)-> PyResult<&'a PyAny> {
//     pyo3_asyncio::tokio::future_into_py(py, async move {
//         let s = ftd::interpret(name, source).map_err(|e| {
//             eprintln!("{:?}", e);
//             pyo3::exceptions::PyTypeError::new_err(e.to_string())
//         })?;
//
//         let i = Interpreter {
//             document_id: name.to_string(),
//             interpreter: std::cell::RefCell::new(Some(s)),
//             config: fpm_config()
//         };
//         Ok(Python::with_gil(|py| i.into_py(py)))
//     })
//
// }

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
