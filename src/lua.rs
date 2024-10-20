// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use mlua::prelude::*;

pub use crate::types::{Case, Locale, Result, StyleGuide};

#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    let case = lua.create_function(case)?;
    exports.set("case", case).unwrap();
    let titlecase = lua.create_function(titlecase)?;
    exports.set("titlecase", titlecase).unwrap();
    let lowercase = lua.create_function(lowercase)?;
    exports.set("lowercase", lowercase).unwrap();
    let uppercase = lua.create_function(uppercase)?;
    exports.set("uppercase", uppercase).unwrap();
    let sentencecase = lua.create_function(sentencecase)?;
    exports.set("sentencecase", sentencecase).unwrap();
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let version = lua.create_string(version)?;
    exports.set("version", version).unwrap();
    Ok(exports)
}

fn case<'a>(
    lua: &'a Lua,
    (input, case, locale, style): (LuaString<'a>, LuaValue<'a>, LuaValue<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let case: Case = match case {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Case::Title),
        _ => Case::Title,
    };
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let style: StyleGuide = match style {
        LuaValue::String(s) => s
            .to_string_lossy()
            .parse()
            .unwrap_or(StyleGuide::LanguageDefault),
        _ => StyleGuide::LanguageDefault,
    };
    let output = to_case(&input, case, locale, style);
    lua.create_string(output)
}

fn titlecase<'a>(
    lua: &'a Lua,
    (input, locale, style): (LuaString<'a>, LuaValue<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let style: StyleGuide = match style {
        LuaValue::String(s) => s
            .to_string_lossy()
            .parse()
            .unwrap_or(StyleGuide::LanguageDefault),
        _ => StyleGuide::LanguageDefault,
    };
    let output = to_titlecase(&input, locale, style);
    lua.create_string(output)
}

fn lowercase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_lowercase(&input, locale);
    lua.create_string(output)
}

fn uppercase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_uppercase(&input, locale);
    lua.create_string(output)
}

fn sentencecase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_sentencecase(&input, locale);
    lua.create_string(output)
}
