// SPDX-FileCopyrightText: © 2024 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use anyhow::Result;
use wasm_minimal_protocol::{initiate_protocol, wasm_func};

use decasify::{Case, Locale, StyleGuide, StyleOptions, StyleOptionsBuilder};

initiate_protocol!();

#[wasm_func]
pub fn decasify(
    data: &[u8],
    case: &[u8],
    lang: &[u8],
    style: &[u8],
    overrides: &[u8],
) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let case = Case::from(case);
    let locale = Locale::from(lang);
    let style = StyleGuide::from(style);
    let overrides_str = String::from_utf8(overrides.to_vec())?;
    let opts = if overrides_str.is_empty() {
        StyleOptions::default()
    } else {
        let overrides = overrides_str.split(',').map(String::from).collect();
        StyleOptionsBuilder::new().overrides(overrides).build()
    };
    Ok(decasify::case(&chunk, case, locale, style, opts).into_bytes())
}

#[wasm_func]
pub fn titlecase(data: &[u8], lang: &[u8], style: &[u8], overrides: &[u8]) -> Result<Vec<u8>> {
    let chunk = String::from_utf8(data.to_vec())?;
    let locale = Locale::from(lang);
    let style = StyleGuide::from(style);
    let overrides_str = String::from_utf8(overrides.to_vec())?;
    let opts = if overrides_str.is_empty() {
        StyleOptions::default()
    } else {
        let overrides = overrides_str.split(',').map(String::from).collect();
        StyleOptionsBuilder::new().overrides(overrides).build()
    };
    Ok(decasify::titlecase(&chunk, locale, style, opts).into_bytes())
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
