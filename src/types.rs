// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use snafu::prelude::*;
use std::convert::{Infallible, TryFrom};
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use strum_macros::{Display, VariantNames};

#[cfg(feature = "pythonmodule")]
use pyo3::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Invalid input language '{input}'"))]
    Locale { input: String },

    #[snafu(display("Invalid target case '{input}'"))]
    Case { input: String },

    #[snafu(display("Invalid preferred style guide '{input}'"))]
    StyleGuide { input: String },

    #[snafu(display("Invalid style options '{input}'"))]
    StyleOptions { input: String },
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl Debug for Error {
    fn fmt(&self, fmt: &mut Formatter) -> std::fmt::Result {
        Display::fmt(self, fmt)
    }
}

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Just a single word
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq))]
#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
pub struct Word {
    pub word: String,
}

// WARNING: These enums can't change order when adding new variants because some modules cast them
// to integers, and that would make for ABI breakage. The variants can be re-sorted (alphabetically
// or logically or whatever) when a major version with no ABI compatibility guarantees is okay.

/// Locale selector to change language support rules of case functions.
#[derive(Default, Display, VariantNames, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
pub enum Locale {
    #[default]
    EN,
    TR,
    ES,
}

/// Target case selector.
#[derive(Default, Display, VariantNames, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
pub enum Case {
    Lower,
    Sentence,
    #[default]
    Title,
    Upper,
}

/// Style guide selector to change grammar and context rules used for title casing.
#[derive(Display, VariantNames, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
#[derive(Default)]
pub enum StyleGuide {
    #[strum(serialize = "ap")]
    AssociatedPress,
    #[strum(serialize = "cmos")]
    ChicagoManualOfStyle,
    #[strum(serialize = "gruber")]
    DaringFireball,
    #[strum(serialize = "default")]
    #[default]
    LanguageDefault,
    #[strum(serialize = "tdk")]
    TurkishLanguageInstitute,
    #[strum(serialize = "rae")]
    RealAcademiaEspanola,
    #[strum(serialize = "fundeu")]
    FundeuRealAcademiaEspanola,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq))]
#[cfg_attr(feature = "wasm", wasm_bindgen(getter_with_clone))]
pub struct StyleOptions {
    pub overrides: Option<Vec<Word>>,
}

impl FromStr for StyleOptions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "default" | "none" | "" => Ok(StyleOptions::default()),
            input => StyleOptionsSnafu { input }.fail()?,
        }
    }
}

impl TryFrom<&str> for StyleOptions {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for StyleOptions {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::from_str(&s)
    }
}

impl TryFrom<&String> for StyleOptions {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&[u8]> for StyleOptions {
    type Error = Error;
    fn try_from(s: &[u8]) -> Result<Self> {
        let s = String::from_utf8_lossy(s);
        Self::from_str(&s)
    }
}

#[derive(Debug)]
pub struct StyleOptionsBuilder {
    overrides: Option<Vec<Word>>,
}

impl Default for StyleOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StyleOptionsBuilder {
    pub fn new() -> Self {
        Self { overrides: None }
    }

    pub fn overrides(mut self, words: Vec<impl Into<Word>>) -> Self {
        let words: Vec<Word> = words.into_iter().map(|w| w.into()).collect();
        self.overrides = Some(words);
        self
    }

    pub fn build(self) -> StyleOptions {
        StyleOptions {
            overrides: self.overrides,
        }
    }
}

impl FromStr for Locale {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "english" | "en_en" => Ok(Locale::EN),
            "es" | "spanish" | "es_es" | "espanol" | "español" => Ok(Locale::ES),
            "tr" | "turkish" | "tr_tr" | "turkce" | "türkçe" => Ok(Locale::TR),
            input => LocaleSnafu { input }.fail()?,
        }
    }
}

impl TryFrom<&str> for Locale {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Locale {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::from_str(&s)
    }
}

impl TryFrom<&String> for Locale {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&[u8]> for Locale {
    type Error = Error;

    fn try_from(s: &[u8]) -> Result<Self> {
        let s = String::from_utf8_lossy(s);
        Self::from_str(&s)
    }
}

impl FromStr for Case {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str().trim_end_matches("case") {
            "lower" => Ok(Case::Lower),
            "sentence" => Ok(Case::Sentence),
            "title" => Ok(Case::Title),
            "upper" => Ok(Case::Upper),
            input => CaseSnafu { input }.fail()?,
        }
    }
}

impl TryFrom<&str> for Case {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for Case {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::from_str(&s)
    }
}

impl TryFrom<&String> for Case {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<&[u8]> for Case {
    type Error = Error;

    fn try_from(s: &[u8]) -> Result<Self> {
        let s = String::from_utf8_lossy(s);
        Self::from_str(&s)
    }
}

impl FromStr for StyleGuide {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "daringfireball" | "gruber" | "fireball" => Ok(StyleGuide::DaringFireball),
            "associatedpress" | "ap" => Ok(StyleGuide::AssociatedPress),
            "chicagoManualofstyle" | "chicago" | "cmos" => Ok(StyleGuide::ChicagoManualOfStyle),
            "fundeu" | "fundeurealacademiaespanola" => Ok(StyleGuide::FundeuRealAcademiaEspanola),
            "rae" | "realacademiaespanola" => Ok(StyleGuide::RealAcademiaEspanola),
            "tdk" | "turkishlanguageinstitute" => Ok(StyleGuide::TurkishLanguageInstitute),
            "default" | "languagedefault" | "language" | "none" | "" => {
                Ok(StyleGuide::LanguageDefault)
            }
            input => StyleGuideSnafu { input }.fail()?,
        }
    }
}

impl TryFrom<&str> for StyleGuide {
    type Error = Error;
    fn try_from(s: &str) -> Result<Self> {
        Self::from_str(s)
    }
}

impl TryFrom<String> for StyleGuide {
    type Error = Error;
    fn try_from(s: String) -> Result<Self> {
        Self::from_str(&s)
    }
}

impl TryFrom<&String> for StyleGuide {
    type Error = Error;
    fn try_from(s: &String) -> Result<Self> {
        Self::from_str(s)
    }
}

impl From<Option<StyleGuide>> for StyleGuide {
    fn from(style: Option<StyleGuide>) -> Self {
        match style {
            Some(style) => style,
            None => StyleGuide::LanguageDefault,
        }
    }
}

impl TryFrom<&[u8]> for StyleGuide {
    type Error = Error;

    fn try_from(s: &[u8]) -> Result<Self> {
        let s = String::from_utf8_lossy(s);
        Self::from_str(&s)
    }
}
