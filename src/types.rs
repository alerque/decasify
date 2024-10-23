// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use std::{error, fmt, fmt::Display, str::FromStr};
use strum_macros::{Display, VariantNames};

#[cfg(feature = "pythonmodule")]
use pyo3::prelude::*;

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub type Result<T> = anyhow::Result<T>;

#[derive(Debug)]
pub struct Error(pub String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for Error {}

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
}

impl FromStr for Locale {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "English" | "en_en" => Ok(Locale::EN),
            "tr" | "Turkish" | "tr_tr" | "türkçe" => Ok(Locale::TR),
            _ => Err(anyhow::Error::new(Error("Invalid input language".into()))),
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

impl FromStr for Case {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        match s.to_ascii_lowercase().as_str().trim_end_matches("case") {
            "lower" => Ok(Case::Lower),
            "sentence" => Ok(Case::Sentence),
            "title" => Ok(Case::Title),
            "upper" => Ok(Case::Upper),
            _ => Err(anyhow::Error::new(Error("Unknown target case".into()))),
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

impl FromStr for StyleGuide {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> crate::Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "daringfireball" | "gruber" | "fireball" => Ok(StyleGuide::DaringFireball),
            "associatedpress" | "ap" => Ok(StyleGuide::AssociatedPress),
            "chicagoManualofstyle" | "chicago" | "cmos" => Ok(StyleGuide::ChicagoManualOfStyle),
            "default" | "languagedefault" | "language" | "none" | "" => {
                Ok(StyleGuide::LanguageDefault)
            }
            _ => Err(anyhow::Error::new(Error("Invalid style guide".into()))),
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

impl From<Option<StyleGuide>> for StyleGuide {
    fn from(style: Option<StyleGuide>) -> Self {
        match style {
            Some(style) => style,
            None => StyleGuide::LanguageDefault,
        }
    }
}
