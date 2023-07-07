use regex::Regex;
use std::{error, result};
use unicode_titlecase::StrTitleCase;

#[cfg(feature = "luamodule")]
use mlua::prelude::*;

#[cfg(feature = "luamodule")]
#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    let titlecase = lua.create_function(titlecase)?;
    exports.set("titlecase", titlecase).unwrap();
    Ok(exports)
}

#[cfg(feature = "luamodule")]
fn titlecase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaString<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale = locale.to_string_lossy();
    let output = to_titlecase(&input, &locale);
    lua.create_string(&output)
}

#[cfg(feature = "cli")]
pub mod cli;

pub type Result<T> = result::Result<T, Box<dyn error::Error>>;

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
    let mut words = words.iter().peekable();
    let mut output: Vec<String> = Vec::new();
    let first = words.next().unwrap();
    output.push(first.to_titlecase_lower_rest());
    while let Some(word) = words.next() {
        if words.peek().is_none() {
            output.push(word.to_titlecase_lower_rest());
        } else {
            match is_reserved_en(word.to_string()) {
                true => output.push(word.to_string().to_lowercase()),
                false => {
                    output.push(word.to_titlecase_lower_rest());
                }
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
    let word = word.as_str();
    let article = Regex::new(r"^(a|an|the)$").unwrap();
    let congunction = Regex::new(r"^(for|and|nor|but|or|yet|so|both|either|neither|not only|whether|after|although|as|as if|as long as|as much as|as soon as|as though|because|before|by the time|even if|even though|if|in order that|in case|in the event that|lest|now that|once|only|only if|provided that|since|so|supposing|that|than|though|till|unless|until|when|whenever|where|whereas|wherever|whether or not|while)$").unwrap();
    let preposition = Regex::new(r"^(about|above|across|after|against|along|among|around|at|before|behind|between|beyond|but|by|concerning|despite|down|during|except|following|for|from|in|including|into|like|near|of|off|on|onto|out|over|past|plus|since|throughout|to|towards|under|until|up|upon|up|to|with|within|without)$").unwrap();
    article.is_match(word) || congunction.is_match(word) || preposition.is_match(word)
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
