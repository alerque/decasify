use crate::*;
use mlua::prelude::*;

pub use crate::types::{InputLocale, Result, StyleGuide};

#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table().unwrap();
    let titlecase = lua.create_function(titlecase)?;
    exports.set("titlecase", titlecase).unwrap();
    let lowercase = lua.create_function(lowercase)?;
    exports.set("lowercase", lowercase).unwrap();
    let uppercase = lua.create_function(uppercase)?;
    exports.set("uppercase", uppercase).unwrap();
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let version = lua.create_string(version)?;
    exports.set("version", version).unwrap();
    Ok(exports)
}

fn titlecase<'a>(
    lua: &'a Lua,
    (input, locale, style): (LuaString<'a>, LuaValue<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: InputLocale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(InputLocale::EN),
        _ => InputLocale::EN,
    };
    let style: Option<StyleGuide> = match style {
        LuaValue::String(s) => s
            .to_string_lossy()
            .parse::<StyleGuide>()
            .map(Some)
            .unwrap_or(None),
        _ => None,
    };
    let output = to_titlecase(&input, locale, style);
    lua.create_string(output)
}

fn lowercase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: InputLocale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(InputLocale::EN),
        _ => InputLocale::EN,
    };
    let output = to_lowercase(&input, locale);
    lua.create_string(output)
}

fn uppercase<'a>(
    lua: &'a Lua,
    (input, locale): (LuaString<'a>, LuaValue<'a>),
) -> LuaResult<LuaString<'a>> {
    let input = input.to_string_lossy();
    let locale: InputLocale = match locale {
        LuaValue::String(s) => s.to_string_lossy().parse().unwrap_or(InputLocale::EN),
        _ => InputLocale::EN,
    };
    let output = to_uppercase(&input, locale);
    lua.create_string(output)
}
