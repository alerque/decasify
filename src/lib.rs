// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod content;
mod traits;
mod types;

pub use content::Chunk;
#[cfg(feature = "unstable-trait")]
pub use traits::Decasify;
pub use types::{Case, Locale, StyleGuide, StyleOptions, StyleOptionsBuilder, Word};

#[cfg(feature = "cli")]
#[doc(hidden)]
pub mod cli;

#[cfg(feature = "luamodule")]
#[doc(hidden)]
pub mod lua;

#[cfg(feature = "pythonmodule")]
#[doc(hidden)]
pub mod python;

#[cfg(feature = "wasm")]
#[doc(hidden)]
pub mod wasm;

mod en;
mod tr;

/// Convert a string to a specific case following typesetting conventions for a target locale
pub fn case(
    chunk: impl Into<Chunk>,
    case: impl Into<Case>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
    opts: impl Into<StyleOptions>,
) -> String {
    let chunk: Chunk = chunk.into();
    let case: Case = case.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    let opts: StyleOptions = opts.into();
    match case {
        Case::Lower => lowercase(chunk, locale),
        Case::Upper => uppercase(chunk, locale),
        Case::Sentence => sentencecase(chunk, locale),
        Case::Title => titlecase(chunk, locale, style, opts),
    }
}

/// Convert a string to title case following typesetting conventions for a target locale
pub fn titlecase(
    chunk: impl Into<Chunk>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
    opts: impl Into<StyleOptions>,
) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    let opts: StyleOptions = opts.into();
    match locale {
        Locale::EN => en::titlecase(chunk, style, opts),
        Locale::TR => tr::titlecase(chunk, style, opts),
    }
}

/// Convert a string to lower case following typesetting conventions for a target locale
pub fn lowercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::lowercase(chunk),
        Locale::TR => tr::lowercase(chunk),
    }
}

/// Convert a string to upper case following typesetting conventions for a target locale
pub fn uppercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::uppercase(chunk),
        Locale::TR => tr::uppercase(chunk),
    }
}

/// Convert a string to sentence case following typesetting conventions for a target locale
pub fn sentencecase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => en::sentencecase(chunk),
        Locale::TR => tr::sentencecase(chunk),
    }
}

fn get_override<F>(word: &Word, overrides: &Option<Vec<Word>>, case_fn: F) -> Option<Word>
where
    F: Fn(&String) -> String,
{
    let word_lower = case_fn(&word.word);
    overrides.as_ref().and_then(|words| {
        words
            .iter()
            .find(|w| case_fn(&w.word) == word_lower)
            .cloned()
    })
}
