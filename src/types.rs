use std::{error, fmt, result, str::FromStr};
use strum_macros::{Display, EnumVariantNames};

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
struct DecasifyError(String);

impl fmt::Display for DecasifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl error::Error for DecasifyError {}

#[derive(Default, Display, EnumVariantNames, Debug, Clone, PartialEq)]
pub enum InputLocale {
    #[default]
    EN,
    TR,
}

#[derive(Default, Display, EnumVariantNames, Debug, Clone, PartialEq)]
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
