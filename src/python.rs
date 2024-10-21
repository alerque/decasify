// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use pyo3::prelude::*;

pub use crate::types::{Case, Locale, Result, StyleGuide};

#[pymodule]
fn decasify(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Case>()?;
    module.add_class::<Locale>()?;
    module.add_class::<StyleGuide>()?;
    module.add_function(wrap_pyfunction!(py_case, module)?)?;
    module.add_function(wrap_pyfunction!(py_titlecase, module)?)?;
    module.add_function(wrap_pyfunction!(py_lowercase, module)?)?;
    module.add_function(wrap_pyfunction!(py_uppercase, module)?)?;
    module.add_function(wrap_pyfunction!(py_sentencecase, module)?)?;
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    module.add::<&str, &str>("version", version)?;
    Ok(())
}

#[pyfunction]
#[pyo3(name = "case")]
#[pyo3(signature = (input, case, locale, style=StyleGuide::LanguageDefault))]
fn py_case(input: String, case: Case, locale: Locale, style: StyleGuide) -> PyResult<String> {
    Ok(to_case(&input, case, locale, style))
}

#[pyfunction]
#[pyo3(name = "titlecase")]
#[pyo3(signature = (input, locale, style=StyleGuide::LanguageDefault))]
fn py_titlecase(input: String, locale: Locale, style: StyleGuide) -> PyResult<String> {
    Ok(to_titlecase(&input, locale, style))
}

#[pyfunction]
#[pyo3(name = "lowercase")]
#[pyo3(signature = (input, locale))]
fn py_lowercase(input: String, locale: Locale) -> PyResult<String> {
    Ok(to_lowercase(&input, locale))
}

#[pyfunction]
#[pyo3(name = "uppercase")]
#[pyo3(signature = (input, locale))]
fn py_uppercase(input: String, locale: Locale) -> PyResult<String> {
    Ok(to_uppercase(&input, locale))
}

#[pyfunction]
#[pyo3(name = "sentencecase")]
#[pyo3(signature = (input, locale))]
fn py_sentencecase(input: String, locale: Locale) -> PyResult<String> {
    Ok(to_sentencecase(&input, locale))
}
