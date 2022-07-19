use pyo3::prelude::*;

pub mod header;
pub mod section;

use section::Section;

#[pyclass]
pub struct Config {
    pub config: fpm::Config,
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
    pub fn state_name(&self) -> PyResult<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return Ok(match i {
                ftd::Interpreter::StuckOnProcessor { .. } => "stuck_on_processor".to_string(),
                ftd::Interpreter::StuckOnForeignVariable { .. } => {
                    "stuck_on_foreign_variable".to_string()
                }
                ftd::Interpreter::StuckOnImport { .. } => "stuck_on_import".to_string(),
                ftd::Interpreter::Done { .. } => "done".to_string(),
            });
        }
        Err(py_err(
            "ftd-sys:Interpreter:state_name this should not get called",
        ))
    }

    pub fn get_module_to_import(&self) -> PyResult<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnImport { module, .. } => Ok(module.to_string()),
                _ => Err(py_err(
                    "ftd-sys: get_module_to_import, this section should not get called",
                )),
            };
        }
        Err(py_err(
            "ftd-sys: get_module_to_import, wrong statement called",
        ))
    }

    pub fn continue_after_import(&self, id: &str, source: Option<String>) -> PyResult<()> {
        let source = source.unwrap_or_else(|| "".to_string());
        let interpreter = self.interpreter.replace(None).ok_or_else(|| {
            py_err("ftd-sys: continue_after_import, interpreter object should exists")
        })?;
        match interpreter {
            ftd::Interpreter::StuckOnImport { state, .. } => {
                let new_interpreter = state
                    .continue_after_import(id, source.as_str())
                    .map_err(|err| py_err(&err.to_string()))?;
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {
                return Err(py_err(
                    "ftd-sys: continue_after_import, this section should not get called",
                ))
            }
        };
        Ok(())
    }

    pub fn get_processor_section(&self) -> PyResult<Section> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnProcessor { section, .. } => Ok(Section {
                    section: section.clone(),
                }),
                _ => Err(py_err(
                    "ftd-sys: get_processor_section, this section should not get called",
                )),
            };
        }
        Err(py_err(
            "ftd-sys: get_processor_section, this statement should not get called",
        ))
    }

    pub fn resolve_processor(&self, section: &Section) -> PyResult<Option<FtdValue>> {
        let interpreter = self.interpreter.borrow();
        let state =
            if let Some(i) = interpreter.as_ref() {
                match i {
                    ftd::Interpreter::StuckOnProcessor { state, .. } => state,
                    _ => return Err(py_err(
                        "ftd-sys: resolve_processor, this should not get called something is wrong",
                    )),
                }
            } else {
                return Err(py_err(
                    "ftd-sys: resolve_processor, interpreter should exists",
                ));
            };

        let value = match fpm::library::process_sync(
            &self.config,
            &section.section,
            &self.document_id,
            &state.tdoc(&mut Default::default()),
        ) {
            Ok(value) => value,
            Err(e) => {
                return match e {
                    ftd::p1::Error::NotFound { .. } => Ok(None),
                    _ => Err(py_err(&e.to_string())),
                }
            }
        };

        Ok(Some(FtdValue { value }))
    }

    pub fn resolve_import(&self, module: &str) -> PyResult<String> {
        use tokio::runtime::Runtime;
        let rt = Runtime::new().map_err(|err| py_err(&err.to_string()))?;
        rt.block_on(async {
            let mut library = self.library.borrow_mut();
            let mut interpreter = self.interpreter.borrow_mut();
            let state = if let Some(ref mut i) = *interpreter {
                match i {
                    ftd::Interpreter::StuckOnImport { ref mut state, .. } => state,
                    _ => return Err(py_err("only stuck_on_import expected")),
                }
            } else {
                return Err(py_err("interpreter_expected"));
            };
            println!("resolving import: {}", module);
            let mut d = fpm::resolve_import(&mut library, state, module)
                .await
                .map_err(|e| {
                    eprintln!("Error: fpm-resolve-import {:?}", e);
                    py_err(&e.to_string())
                })?;
            println!("import resolved: {}", module);
            if d.is_empty() {
                d = "__import_resolved__".to_string()
            }
            Ok(d)
        })
    }

    pub fn continue_after_processor(&self, value: FtdValue) -> PyResult<()> {
        let interpreter = self.interpreter.replace(None).ok_or_else(|| {
            py_err("ftd-sys: continue_after_processor, interpreter should exists")
        })?;
        match interpreter {
            ftd::Interpreter::StuckOnProcessor { state, section } => {
                let new_interpreter = state
                    .continue_after_processor(&section, value.value)
                    .map_err(|err| py_err(&err.to_string()))?;
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {
                return Err(py_err(
                    "continue-after-processor, this should not get called",
                ))
            }
        };
        Ok(())
    }

    pub fn get_foreign_variable_to_resolve(&self) -> PyResult<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
                ftd::Interpreter::StuckOnForeignVariable { variable, .. } => {
                    Ok(variable.to_string())
                }
                _ => Err(py_err("stuck-on-fv, this should not get called")),
            };
        }
        Err(py_err("stuck-on-fv, something bad wrong"))
    }

    pub fn resolve_foreign_variable(
        &self,
        variable: String,
        base_url: Option<String>,
    ) -> PyResult<FtdValue> {
        let rt = tokio::runtime::Runtime::new().map_err(|err| py_err(&err.to_string()))?;

        rt.block_on(async {
            let base_url = base_url.unwrap_or_else(|| "/".to_string());
            let mut library = self.library.borrow_mut();
            let interpreter = self.interpreter.borrow();
            let state = if let Some(ref i) = interpreter.as_ref() {
                match i {
                    ftd::Interpreter::StuckOnForeignVariable { ref state, .. } => state,
                    _ => {
                        return Err(py_err(
                            "resolve_foreign_variable only StuckOnForeignVariable expected",
                        ))
                    }
                }
            } else {
                return Err(py_err("resolve_foreign_variable interpreter_expected"));
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
                py_err(&e.to_string())
            })?;
            println!("foreign variable: {}", variable);
            Ok(FtdValue { value: d })
        })
    }

    pub fn continue_after_foreign_variable(&self, variable: &str, value: FtdValue) -> PyResult<()> {
        let interpreter = self.interpreter.replace(None).ok_or_else(|| {
            py_err("ftd-sys: continue_after_foreign_variable, interpreter should exists")
        })?;
        match interpreter {
            ftd::Interpreter::StuckOnForeignVariable { state, .. } => {
                let new_interpreter = state
                    .continue_after_variable(variable, value.value)
                    .map_err(|err| py_err(&err.to_string()))?;
                self.interpreter.replace(Some(new_interpreter));
            }
            _ => {
                return Err(py_err(
                    "continue-after-foreign-variable, this should not get called",
                ))
            }
        };
        Ok(())
    }

    // From Option to Result
    pub fn render(&self) -> PyResult<String> {
        let interpreter = self.interpreter.borrow();
        if let Some(i) = interpreter.as_ref() {
            return match i {
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
                    Ok(file_content)
                }
                _ => Err(py_err("this should not get called something is wrong")),
            };
        }
        Ok("".to_string())
    }
}

fn py_err(err: &str) -> PyErr {
    pyo3::exceptions::PyTypeError::new_err(err.to_string())
}

fn fpm_config(root: Option<String>, data: Option<String>) -> PyResult<Config> {
    use tokio::runtime::Runtime;
    let rt = Runtime::new().map_err(|err| py_err(&err.to_string()))?;
    rt.block_on(async {
        let mut config = fpm::Config::read2(root, false)
            .await
            .map(|config| Config { config })
            .map_err(|err| {
                eprintln!("fpm_config {:?}", err);
                py_err(&err.to_string())
            })?;
        if let Some(data) = data {
            config
                .config
                .attach_data_string(data.as_str())
                .map_err(|err| {
                    eprintln!("fpm_config attach_data_string {:?}", err);
                    py_err(&err.to_string())
                })?;
        }
        Ok(config)
    })
}

fn file_content(config: &fpm::Config, id: &str) -> Result<String, String> {
    let rt = tokio::runtime::Runtime::new().map_err(|err| err.to_string())?;
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
    data: Option<String>,
) -> PyResult<Interpreter> {
    let config = fpm_config(root, data)?;
    let source = file_content(&config.config, id).map_err(|err| py_err(&err))?;

    let s = ftd::interpret(id, &source).map_err(|e| {
        eprintln!("{:?}", e);
        py_err(&e.to_string())
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

#[pyfunction]
fn get_file_content(root: &str, path: &str) -> PyResult<(Vec<u8>, String)> {
    let rt = tokio::runtime::Runtime::new().map_err(|err| py_err(&err.to_string()))?;
    let mut config = fpm_config(Some(root.to_string()), None)?.config;
    rt.block_on(async {
        let f = match config.get_file_and_package_by_id(path).await {
            Ok(f) => f,
            Err(e) => {
                eprintln!("ftd-sys error: path: {}, {:?}", path, e);
                return Err(py_err(&e.to_string()));
            }
        };
        config.current_document = Some(f.get_id());

        Ok(match f {
            fpm::File::Ftd(document) => (document.content.into_bytes(), "ftd".to_string()),
            fpm::File::Code(document) => (
                document.content.into_bytes(),
                guess_mime_type(&document.id).as_ref().to_string(),
            ),
            fpm::File::Image(document) => (
                document.content,
                guess_mime_type(&document.id).as_ref().to_string(),
            ),
            fpm::File::Markdown(document) => (
                document.content.into_bytes(),
                guess_mime_type(&document.id).as_ref().to_string(),
            ),
            fpm::File::Static(document) => (
                document.content,
                guess_mime_type(&document.id).as_ref().to_string(),
            ),
        })
    })
}

fn guess_mime_type(path: &str) -> mime_guess::Mime {
    mime_guess::from_path(path).first_or_octet_stream()
}

#[pyfunction]
fn string_to_value(value: String) -> PyResult<FtdValue> {
    Ok(FtdValue {
        value: ftd::Value::String {
            text: value,
            source: ftd::TextSource::Default,
        },
    })
}

#[pyfunction]
fn object_to_value(
    data: String,
    section: &Section,
    interpreter: &Interpreter,
) -> PyResult<FtdValue> {
    let interpreter = interpreter.interpreter.borrow();
    let state = {
        if let Some(i) = interpreter.as_ref() {
            match i {
                ftd::Interpreter::StuckOnProcessor { state, .. } => state,
                ftd::Interpreter::StuckOnForeignVariable { state, .. } => state,
                ftd::Interpreter::StuckOnImport { state, .. } => state,
                ftd::Interpreter::Done { .. } => {
                    return Err(py_err(
                        "ftd-sys: object_to_value, done should not get called",
                    ));
                }
            }
        } else {
            return Err(py_err(
                "ftd-sys: object_to_value, interpreter should not be none",
            ));
        }
    };
    let mut d = Default::default();
    let doc = state.tdoc(&mut d);
    let data = &serde_json::from_str::<serde_json::Value>(&data)
        .map_err(|e| py_err(&format!("ftd-sys: object_to_value, err: {}", e)))?;
    // TODO: remove expect
    // replace from_json to from_pyany
    let value = doc
        .from_json(&data, &section.section)
        .map_err(|e| py_err(&format!("ftd-sys: object_to_value, err: {:?}", e)))?;
    Ok(FtdValue { value })
}

/// A Python module implemented in Rust.
#[pymodule]
fn ftd_sys(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(interpret, m)?)?;
    m.add_function(wrap_pyfunction!(get_file_content, m)?)?;
    m.add_function(wrap_pyfunction!(string_to_value, m)?)?;
    m.add_function(wrap_pyfunction!(object_to_value, m)?)?;
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
