use crate::*;
use std::result::Result;
use wasm_bindgen::prelude::*;

pub use crate::types::{InputLocale, StyleGuide};

#[wasm_bindgen]
pub fn titlecase(
    input: &str,
    locale: InputLocale,
    style: Option<StyleGuide>,
) -> Result<String, JsError> {
    Ok(to_titlecase(input, locale, style))
}

#[wasm_bindgen]
pub fn lowercase(input: &str, locale: InputLocale) -> Result<String, JsError> {
    Ok(to_lowercase(input, locale))
}

#[wasm_bindgen]
pub fn uppercase(input: &str, locale: InputLocale) -> Result<String, JsError> {
    Ok(to_uppercase(input, locale))
}

#[wasm_bindgen]
pub fn sentencecase(input: &str, locale: InputLocale) -> Result<String, JsError> {
    Ok(to_sentencecase(input, locale))
}
