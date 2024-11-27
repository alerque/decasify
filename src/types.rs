// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use std::str::FromStr;
use strum_macros::{Display, VariantNames};

use snafu::prelude::*;

#[cfg(feature = "pythonmodule")]
use pyo3::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("Invalid input language {}", input))]
    Locale { input: String },

    #[snafu(display("Invalid target case {}", input))]
    Case { input: String },

    #[snafu(display("Invalid preferred style guide {}", input))]
    StyleGuide { input: String },
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Locale selector to change language support rules of case functions.
#[derive(Default, Display, VariantNames, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
pub enum Locale {
    #[default]
    EN,
    TR,
}

/// Target case selector.
#[derive(Default, Display, VariantNames, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
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
#[derive(Default, Display, VariantNames, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
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
}

impl FromStr for Locale {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "english" | "en_en" => Ok(Locale::EN),
            "tr" | "turkish" | "tr_tr" | "türkçe" => Ok(Locale::TR),
            input => LocaleSnafu { input }.fail()?,
        }
    }
}

impl From<&str> for Locale {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl From<String> for Locale {
    fn from(s: String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&String> for Locale {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&[u8]> for Locale {
    fn from(s: &[u8]) -> Self {
        let s = String::from_utf8(s.to_vec()).unwrap();
        Self::from_str(s.as_ref()).unwrap()
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

impl From<&str> for Case {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl From<String> for Case {
    fn from(s: String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&String> for Case {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&[u8]> for Case {
    fn from(s: &[u8]) -> Self {
        let s = String::from_utf8(s.to_vec()).unwrap();
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl FromStr for StyleGuide {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "daringfireball" | "gruber" | "fireball" => Ok(StyleGuide::DaringFireball),
            "associatedpress" | "ap" => Ok(StyleGuide::AssociatedPress),
            "chicagoManualofstyle" | "chicago" | "cmos" => Ok(StyleGuide::ChicagoManualOfStyle),
            "tdk" | "turkishlanguageinstitute" => Ok(StyleGuide::TurkishLanguageInstitute),
            "default" | "languagedefault" | "language" | "none" | "" => {
                Ok(StyleGuide::LanguageDefault)
            }
            input => StyleGuideSnafu { input }.fail()?,
        }
    }
}

impl From<&str> for StyleGuide {
    fn from(s: &str) -> Self {
        Self::from_str(s).unwrap()
    }
}

impl From<String> for StyleGuide {
    fn from(s: String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&String> for StyleGuide {
    fn from(s: &String) -> Self {
        Self::from_str(s.as_ref()).unwrap()
    }
}

impl From<&[u8]> for StyleGuide {
    fn from(s: &[u8]) -> Self {
        let s = String::from_utf8(s.to_vec()).unwrap();
        Self::from_str(s.as_ref()).unwrap()
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
