use pyo3::prelude::*;
use ::inox2d::formats::inp::parse_inp;
use ::inox2d::model::Model;
use ::inox2d::INOCHI2D_SPEC_VERSION;
use std::fs::File;
use std::io::BufReader;

#[pyclass]
struct InoxContext {
    model: Option<Model>,
}

#[pymethods]
impl InoxContext {
    #[new]
    fn new() -> Self {
        InoxContext { model: None }
    }

    fn load_model(&mut self, path: String) -> PyResult<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let model = parse_inp(reader).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        self.model = Some(model);
        Ok(())
    }
}

#[pymodule]
fn inox2d(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<InoxContext>()?;
    // Add aliases
    m.add("Context", m.getattr("InoxContext")?)?;
    m.add("Renderer", m.getattr("InoxContext")?)?;
    m.add("INOCHI2D_SPEC_VERSION", INOCHI2D_SPEC_VERSION)?;
    Ok(())
}
