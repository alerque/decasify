use std::{error, result};
use unicode_titlecase::StrTitleCase;

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn to_titlecase(string: &String, locale: &String) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale.as_str() {
        "tr" => to_titlecase_tr(words),
        "tr_TR" => to_titlecase_tr(words),
        _ => to_titlecase_en(words),
    }
}

pub fn to_titlecase_en(_words: Vec<&str>) -> String {
    String::from("English")
    // string.to_titlecase_lower_rest()
}

pub fn to_titlecase_tr(words: Vec<&str>) -> String {
    let mut words = words.iter();
    let mut output: Vec<String> = Vec::new();
    let first = words.next().unwrap();
    output.push(first.to_titlecase_tr_or_az_lower_rest());
    for word in words {
        match is_reserved_tr(word.to_string()) {
            true => {
                output.push(word.to_titlecase_tr_or_az_lower_rest());
            }
            false => output.push(word.to_string()),
        }
    }
    output.join(" ")
}

pub fn is_reserved_tr(_word: String) -> bool {
    false
    // “ve, ile, ya, veya, yahut, ki, da, de” sözleri ile “mı, mi, mu, mü” soru
}
