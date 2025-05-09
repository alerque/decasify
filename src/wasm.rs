// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;

use std::result::Result;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn case(
    input: &str,
    case: Case,
    locale: Locale,
    style: StyleGuide,
    opts: Option<StyleOptions>,
) -> Result<String, JsError> {
    let opts = opts.unwrap_or_default();
    Ok(crate::case(input, case, locale, style, opts))
}

#[wasm_bindgen]
pub fn titlecase(
    input: &str,
    locale: Locale,
    style: StyleGuide,
    opts: Option<StyleOptions>,
) -> Result<String, JsError> {
    let opts = opts.unwrap_or_default();
    Ok(crate::titlecase(input, locale, style, opts))
}

#[wasm_bindgen]
pub fn lowercase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(crate::lowercase(input, locale))
}

#[wasm_bindgen]
pub fn uppercase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(crate::uppercase(input, locale))
}

#[wasm_bindgen]
pub fn sentencecase(input: &str, locale: Locale) -> Result<String, JsError> {
    Ok(crate::sentencecase(input, locale))
}
