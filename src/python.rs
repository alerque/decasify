// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::types::*;
use pyo3::prelude::*;

#[pymodule]
fn decasify(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<Case>()?;
    module.add_class::<Locale>()?;
    module.add_class::<StyleGuide>()?;
    module.add_class::<Word>()?;
    module.add_class::<StyleGuideOptions>()?;
    module.add_function(wrap_pyfunction!(self::case, module)?)?;
    module.add_function(wrap_pyfunction!(self::titlecase, module)?)?;
    module.add_function(wrap_pyfunction!(self::lowercase, module)?)?;
    module.add_function(wrap_pyfunction!(self::uppercase, module)?)?;
    module.add_function(wrap_pyfunction!(self::sentencecase, module)?)?;
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    module.add::<&str, &str>("version", version)?;
    Ok(())
}

#[pyfunction]
#[pyo3(signature = (input, case, locale, style=StyleGuide::LanguageDefault(None), overrides=None))]
fn case(
    input: String,
    case: Case,
    locale: Locale,
    style: StyleGuide,
    overrides: Option<Vec<String>>,
) -> PyResult<String> {
    let style = if let Some(words) = overrides {
        StyleGuideBuilder::new(style)
            .overrides(words.into_iter().map(Word::from).collect())
            .build()
    } else {
        style
    };
    Ok(crate::case(&input, case, locale, style))
}

#[pyfunction]
#[pyo3(signature = (input, locale, style=StyleGuide::LanguageDefault(None), overrides=None))]
fn titlecase(
    input: String,
    locale: Locale,
    style: StyleGuide,
    overrides: Option<Vec<String>>,
) -> PyResult<String> {
    let style = if let Some(words) = overrides {
        StyleGuideBuilder::new(style)
            .overrides(words.into_iter().map(Word::from).collect())
            .build()
    } else {
        style
    };
    Ok(crate::titlecase(&input, locale, style))
}

#[pyfunction]
#[pyo3(signature = (input, locale))]
fn lowercase(input: String, locale: Locale) -> PyResult<String> {
    Ok(crate::lowercase(&input, locale))
}

#[pyfunction]
#[pyo3(signature = (input, locale))]
fn uppercase(input: String, locale: Locale) -> PyResult<String> {
    Ok(crate::uppercase(&input, locale))
}

#[pyfunction]
#[pyo3(signature = (input, locale))]
fn sentencecase(input: String, locale: Locale) -> PyResult<String> {
    Ok(crate::sentencecase(&input, locale))
}
