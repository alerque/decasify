use std::{error, result};
use unicode_titlecase::{StrTitleCase};

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn to_titlecase (string: &String, locale: &String) -> String {
    match locale.as_str() {
        "tr" => string.to_titlecase_tr_or_az_lower_rest(),
        "tr_TR" => string.to_titlecase_tr_or_az_lower_rest(),
        _ => string.to_titlecase_lower_rest(),
    }

}
