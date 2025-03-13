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

/// Just a single word
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Word {
    pub word: String,
}

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
#[derive(Display, VariantNames, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[strum(serialize_all = "lowercase")]
#[non_exhaustive]
pub enum StyleGuide {
    #[strum(serialize = "ap")]
    AssociatedPress(Option<StyleGuideOptions>),
    #[strum(serialize = "cmos")]
    ChicagoManualOfStyle(Option<StyleGuideOptions>),
    #[strum(serialize = "gruber")]
    DaringFireball(Option<StyleGuideOptions>),
    #[strum(serialize = "default")]
    LanguageDefault(Option<StyleGuideOptions>),
    #[strum(serialize = "tdk")]
    TurkishLanguageInstitute(Option<StyleGuideOptions>),
}

impl Default for StyleGuide {
    fn default() -> Self {
        Self::LanguageDefault(None)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct StyleGuideOptions {
    pub overrides: Option<Vec<Word>>,
}

#[derive(Debug)]
pub struct StyleGuideBuilder {
    base: StyleGuide,
    overrides: Option<Vec<Word>>,
}

impl StyleGuide {
    pub fn with_options(&self, options: StyleGuideOptions) -> Self {
        match self {
            StyleGuide::AssociatedPress(_) => StyleGuide::AssociatedPress(Some(options)),
            StyleGuide::ChicagoManualOfStyle(_) => StyleGuide::ChicagoManualOfStyle(Some(options)),
            StyleGuide::DaringFireball(_) => StyleGuide::DaringFireball(Some(options)),
            StyleGuide::LanguageDefault(_) => StyleGuide::LanguageDefault(Some(options)),
            StyleGuide::TurkishLanguageInstitute(_) => {
                StyleGuide::TurkishLanguageInstitute(Some(options))
            }
        }
    }

    pub fn options(&self) -> Option<&StyleGuideOptions> {
        match self {
            StyleGuide::AssociatedPress(opts) => opts.as_ref(),
            StyleGuide::ChicagoManualOfStyle(opts) => opts.as_ref(),
            StyleGuide::DaringFireball(opts) => opts.as_ref(),
            StyleGuide::LanguageDefault(opts) => opts.as_ref(),
            StyleGuide::TurkishLanguageInstitute(opts) => opts.as_ref(),
        }
    }
}

impl StyleGuideBuilder {
    pub fn new(base: impl Into<StyleGuide>) -> Self {
        Self {
            base: base.into(),
            overrides: None,
        }
    }

    pub fn overrides(mut self, words: Vec<impl Into<Word>>) -> Self {
        let words: Vec<Word> = words.into_iter().map(|w| w.into()).collect();
        self.overrides = Some(words);
        self
    }

    pub fn build(self) -> StyleGuide {
        let options = StyleGuideOptions {
            overrides: self.overrides,
        };
        self.base.with_options(options)
    }
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
            "daringfireball" | "gruber" | "fireball" => Ok(StyleGuide::DaringFireball(None)),
            "associatedpress" | "ap" => Ok(StyleGuide::AssociatedPress(None)),
            "chicagoManualofstyle" | "chicago" | "cmos" => {
                Ok(StyleGuide::ChicagoManualOfStyle(None))
            }
            "tdk" | "turkishlanguageinstitute" => Ok(StyleGuide::TurkishLanguageInstitute(None)),
            "default" | "languagedefault" | "language" | "none" | "" => {
                Ok(StyleGuide::LanguageDefault(None))
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
            None => StyleGuide::LanguageDefault(None),
        }
    }
}
