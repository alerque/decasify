use anyhow::Result;
use decasify::{Case, Locale, StyleGuide};
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

#[wasm_func]
pub fn decasify(data: &[u8], case: &[u8], lang: &[u8], style: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let case = Case::from(case);
    let locale = Locale::from(lang);
    let style = StyleGuide::from(style);
    Ok(decasify::case(&chunk, case, locale, style).into_bytes())
}

#[wasm_func]
pub fn titlecase(data: &[u8], lang: &[u8], style: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let locale = Locale::from(lang);
    let style = StyleGuide::from(style);
    Ok(decasify::titlecase(&chunk, locale, style).into_bytes())
}

#[wasm_func]
pub fn lowercase(data: &[u8], lang: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let locale = Locale::from(lang);
    Ok(decasify::lowercase(&chunk, locale).into_bytes())
}

#[wasm_func]
pub fn uppercase(data: &[u8], lang: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let locale = Locale::from(lang);
    Ok(decasify::uppercase(&chunk, locale).into_bytes())
}

#[wasm_func]
pub fn sentencecase(data: &[u8], lang: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let locale = Locale::from(lang);
    Ok(decasify::sentencecase(&chunk, locale).into_bytes())
}
