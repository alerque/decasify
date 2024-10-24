// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#![doc = include_str!("../README.md")]

pub mod content;
pub mod types;

pub use content::{Chunk, Segment};
pub use types::{Case, Locale, Result, StyleGuide};

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "luamodule")]
pub mod lua;

#[cfg(feature = "pythonmodule")]
pub mod python;

#[cfg(feature = "wasm")]
pub mod wasm;

mod en;
mod tr;

/// Convert a string to a specific case following typesetting conventions for a target locale
pub fn to_case(
    chunk: impl Into<Chunk>,
    case: impl Into<Case>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
) -> String {
    let chunk: Chunk = chunk.into();
    let case: Case = case.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    match case {
        Case::Lower => to_lowercase(chunk, locale),
        Case::Upper => to_uppercase(chunk, locale),
        Case::Sentence => to_sentencecase(chunk, locale),
        Case::Title => to_titlecase(chunk, locale, style),
    }
}

/// Convert a string to title case following typesetting conventions for a target locale
pub fn to_titlecase(
    chunk: impl Into<Chunk>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    match locale {
        Locale::EN => en::to_titlecase(chunk, style),
        Locale::TR => tr::to_titlecase(chunk, style),
    }
}

/// Convert a string to lower case following typesetting conventions for a target locale
pub fn to_lowercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::to_lowercase(chunk),
        Locale::TR => tr::to_lowercase(chunk),
    }
}

/// Convert a string to upper case following typesetting conventions for a target locale
pub fn to_uppercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::to_uppercase(chunk),
        Locale::TR => tr::to_uppercase(chunk),
    }
}

/// Convert a string to sentence case following typesetting conventions for a target locale
pub fn to_sentencecase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::to_sentencecase(chunk),
        Locale::TR => tr::to_sentencecase(chunk),
    }
}
