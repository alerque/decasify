use regex::Regex;
use std::{error, result};
use unicode_titlecase::StrTitleCase;

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

pub fn to_titlecase(string: &str, locale: &str) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale {
        "tr" => to_titlecase_tr(words),
        "tr_TR" => to_titlecase_tr(words),
        _ => to_titlecase_en(words),
    }
}

pub fn to_titlecase_en(words: Vec<&str>) -> String {
    let mut words = words.iter();
    let mut output: Vec<String> = Vec::new();
    let first = words.next().unwrap();
    output.push(first.to_titlecase_lower_rest());
    for word in words {
        match is_reserved_en(word.to_string()) {
            true => output.push(word.to_string().to_lowercase()),
            false => {
                output.push(word.to_titlecase_lower_rest());
            }
        }
    }
    output.join(" ")
}

pub fn to_titlecase_tr(words: Vec<&str>) -> String {
    let mut words = words.iter();
    let mut output: Vec<String> = Vec::new();
    let first = words.next().unwrap();
    output.push(first.to_titlecase_tr_or_az_lower_rest());
    for word in words {
        match is_reserved_tr(word.to_string()) {
            true => output.push(word.to_string().to_lowercase()),
            false => {
                output.push(word.to_titlecase_tr_or_az_lower_rest());
            }
        }
    }
    output.join(" ")
}

pub fn is_reserved_en(word: String) -> bool {
    let word = word.to_lowercase();
    let congunction = Regex::new(r"^(and|or)$").unwrap();
    congunction.is_match(word.as_str())
}

pub fn is_reserved_tr(word: String) -> bool {
    let baglac = Regex::new(
        r"^([Vv][Ee]|[İi][Ll][Ee]|[Yy][Aa]|[Vv][Ee]|[Yy][Aa][Hh][Uu][Tt]|[Kk][İi]|[Dd][AaEe])$",
    )
    .unwrap();
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])").unwrap();
    let word = word.as_str();
    baglac.is_match(word) || soruek.is_match(word)
}
