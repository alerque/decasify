// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use std::result::Result;
use wasm_bindgen::prelude::*;

pub use crate::types::{Case, Locale, StyleGuide};

#[wasm_bindgen]
pub fn case(input: &str, case: Case, locale: Locale, style: StyleGuide) -> Result<String, JsError> {
    Ok(to_case(input, case, locale, style))
}

#[wasm_bindgen]
pub fn titlecase(input: &str, locale: Locale, style: StyleGuide) -> Result<String, JsError> {
    Ok(to_titlecase(input, locale, style))
}

#[wasm_bindgen]
pub fn lowercase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(to_lowercase(input, locale))
}

#[wasm_bindgen]
pub fn uppercase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(to_uppercase(input, locale))
}

#[wasm_bindgen]
pub fn sentencecase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(to_sentencecase(input, locale))
}
