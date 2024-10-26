// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use mlua::prelude::*;

#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "case",
        LuaFunction::wrap_raw::<_, (Chunk, Case, Locale, StyleGuide)>(case),
    )?;
    exports.set(
        "titlecase",
        LuaFunction::wrap_raw::<_, (Chunk, Locale, StyleGuide)>(titlecase),
    )?;
    exports.set(
        "lowercase",
        LuaFunction::wrap_raw::<_, (Chunk, Locale)>(lowercase),
    )?;
    exports.set(
        "uppercase",
        LuaFunction::wrap_raw::<_, (Chunk, Locale)>(uppercase),
    )?;
    exports.set(
        "sentencecase",
        LuaFunction::wrap_raw::<_, (Chunk, Locale)>(sentencecase),
    )?;
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let version = lua.create_string(version)?;
    exports.set("version", version)?;
    Ok(exports)
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Chunk {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            _ => Ok("".into()),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Locale {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Case {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for StyleGuide {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => Ok(s.to_string_lossy().into()),
            LuaValue::Nil => Ok(Self::default()),
            _ => unimplemented!(),
        }
    }
}
