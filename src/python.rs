use crate::*;
use pyo3::prelude::*;

pub use crate::types::{InputLocale, Result, StyleGuide};

#[pymodule]
fn decasify(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<InputLocale>()?;
    module.add_class::<StyleGuide>()?;
    module.add_function(wrap_pyfunction!(py_titlecase, module)?)?;
    module.add_function(wrap_pyfunction!(py_lowercase, module)?)?;
    module.add_function(wrap_pyfunction!(py_uppercase, module)?)?;
    Ok(())
}

#[pyfunction]
#[pyo3(name = "titlecase")]
#[pyo3(signature = (input, locale, style=None))]
fn py_titlecase(input: String, locale: InputLocale, style: Option<StyleGuide>) -> PyResult<String> {
    Ok(to_titlecase(&input, locale, style))
}

#[pyfunction]
#[pyo3(name = "lowercase")]
#[pyo3(signature = (input, locale))]
fn py_lowercase(input: String, locale: InputLocale) -> PyResult<String> {
    Ok(to_lowercase(&input, locale))
}

#[pyfunction]
#[pyo3(name = "uppercase")]
#[pyo3(signature = (input, locale))]
fn py_uppercase(input: String, locale: InputLocale) -> PyResult<String> {
    Ok(to_uppercase(&input, locale))
}
