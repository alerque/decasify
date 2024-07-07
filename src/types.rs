use std::{error, fmt, result, str::FromStr};
use strum_macros::{Display, VariantNames};

#[cfg(feature = "pythonmodule")]
use pyo3::prelude::*;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct DecasifyError(String);

impl fmt::Display for DecasifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for DecasifyError {}

/// Locale selector to change language support rules of case functions.
#[derive(Default, Display, VariantNames, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
pub enum InputLocale {
    #[default]
    EN,
    TR,
}

/// Target case selector.
#[derive(Default, Display, VariantNames, Debug, Clone, PartialEq)]
pub enum TargetCase {
    Lower,
    Sentence,
    #[default]
    Title,
    Upper,
}

/// Style guide selector to change grammar and context rules used for title casing.
#[derive(Default, Display, VariantNames, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "pythonmodule", pyclass(eq, eq_int))]
pub enum StyleGuide {
    #[strum(serialize = "ap")]
    AssociatedPress,
    #[strum(serialize = "cmos")]
    ChicagoManualOfStyle,
    #[strum(serialize = "gruber")]
    #[default]
    DaringFireball,
}

impl FromStr for InputLocale {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "English" | "en_en" => Ok(InputLocale::EN),
            "tr" | "Turkish" | "tr_tr" | "türkçe" => Ok(InputLocale::TR),
            _ => Err(Box::new(DecasifyError("Invalid input language".into()))),
        }
    }
}

impl FromStr for TargetCase {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "lower" => Ok(TargetCase::Lower),
            "sentence" => Ok(TargetCase::Sentence),
            "title" => Ok(TargetCase::Title),
            "upper" => Ok(TargetCase::Upper),
            _ => Err(Box::new(DecasifyError("Unknown target case".into()))),
        }
    }
}

impl FromStr for StyleGuide {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        match s.to_ascii_lowercase().as_str() {
            "daringfireball" | "gruber" | "fireball" => Ok(StyleGuide::DaringFireball),
            "associatedpress" | "ap" => Ok(StyleGuide::AssociatedPress),
            "chicagoManualofstyle" | "chicago" | "cmos" => Ok(StyleGuide::ChicagoManualOfStyle),
            _ => Err(Box::new(DecasifyError("Invalid style guide".into()))),
        }
    }
}
