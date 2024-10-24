// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use mlua::prelude::*;

pub use crate::types::{Case, Locale, Result, StyleGuide};

impl FromLua for Chunk {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            _ => Ok("".into()),
        }
    }
}

impl FromLua for Locale {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}

impl FromLua for Case {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}

impl FromLua for StyleGuide {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}

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

fn case(
    lua: &Lua,
    (input, case, locale, style): (LuaString, LuaValue, LuaValue, LuaValue),
) -> LuaResult<LuaString> {
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

fn titlecase(
    lua: &Lua,
    (input, locale, style): (LuaString, LuaValue, LuaValue),
) -> LuaResult<LuaString> {
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

fn lowercase(lua: &Lua, (input, locale): (LuaString, LuaValue)) -> LuaResult<LuaString> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_lowercase(&input, locale);
    lua.create_string(output)
}

fn uppercase(lua: &Lua, (input, locale): (LuaString, LuaValue)) -> LuaResult<LuaString> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_uppercase(&input, locale);
    lua.create_string(output)
}

fn sentencecase(lua: &Lua, (input, locale): (LuaString, LuaValue)) -> LuaResult<LuaString> {
    let input = input.to_string_lossy();
    let locale: Locale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(Locale::EN),
        _ => Locale::EN,
    };
    let output = to_sentencecase(&input, locale);
    lua.create_string(output)
}
