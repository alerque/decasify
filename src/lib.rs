use mlua::prelude::*;

use regex::Regex;
use std::{error, result};
use unicode_titlecase::StrTitleCase;

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    let titlecase = lua.create_function(titlecase)?;
    exports.set("titlecase", titlecase).unwrap();
    Ok(exports)
}

fn titlecase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaString<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale = locale.to_string_lossy();
    let output = to_titlecase(&input, &locale);
    lua.create_string(&output)
}

/// Convert a string to title case following typestting conventions for a target locale
pub fn to_titlecase(string: &str, locale: &str) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale {
        "tr" => to_titlecase_tr(words),
        "tr_TR" => to_titlecase_tr(words),
        _ => to_titlecase_en(words),
    }
}

fn to_titlecase_en(words: Vec<&str>) -> String {
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

fn to_titlecase_tr(words: Vec<&str>) -> String {
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

fn is_reserved_en(word: String) -> bool {
    let word = word.to_lowercase();
    let congunction = Regex::new(r"^(and|or)$").unwrap();
    congunction.is_match(word.as_str())
}

fn is_reserved_tr(word: String) -> bool {
    let baglac = Regex::new(
        r"^([Vv][Ee]|[İi][Ll][Ee]|[Yy][Aa]|[Vv][Ee]|[Yy][Aa][Hh][Uu][Tt]|[Kk][İi]|[Dd][AaEe])$",
    )
    .unwrap();
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])").unwrap();
    let word = word.as_str();
    baglac.is_match(word) || soruek.is_match(word)
}
