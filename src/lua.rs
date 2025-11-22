// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::*;
use mlua::prelude::*;

use crate::types::{Error, Result};

macro_rules! impl_into_luaresult {
    ($($t:ty),*) => {
        $(
            impl Into<LuaResult<$t>> for $t {
                fn into(self) -> LuaResult<$t> {
                    Ok(self)
                }
            }
        )*
    };
}

impl_into_luaresult!(Locale, Case, StyleGuide, StyleOptions);

impl IntoLua for Error {
    fn into_lua(self, _: &Lua) -> LuaResult<LuaValue> {
        Ok(LuaValue::Error(Box::new(LuaError::external(self))))
    }
}

impl From<Error> for LuaError {
    fn from(err: Error) -> LuaError {
        LuaError::RuntimeError(err.to_string())
    }
}

impl TryFrom<LuaString> for Locale {
    type Error = Error;
    fn try_from(s: LuaString) -> Result<Self> {
        s.to_string_lossy().try_into()
    }
}

impl TryFrom<LuaString> for Case {
    type Error = Error;
    fn try_from(s: LuaString) -> Result<Self> {
        s.to_string_lossy().try_into()
    }
}

impl TryFrom<LuaString> for StyleGuide {
    type Error = Error;
    fn try_from(s: LuaString) -> Result<Self> {
        s.to_string_lossy().try_into()
    }
}

#[mlua::lua_module]
fn decasify(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set(
        "case",
        LuaFunction::wrap_raw::<_, (Chunk, Case, Locale, StyleGuide, StyleOptions)>(case),
    )?;
    exports.set(
        "titlecase",
        LuaFunction::wrap_raw::<_, (Chunk, Locale, StyleGuide, StyleOptions)>(titlecase),
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
    let mt = lua.create_table()?;
    let decasify = lua.create_function(
        move |_,
              (_, chunk, case_, locale, styleguide, styleoptions): (
            LuaTable,
            Chunk,
            Case,
            Locale,
            StyleGuide,
            Option<StyleOptions>,
        )| {
            Ok(case(
                chunk,
                case_,
                locale,
                styleguide,
                styleoptions.unwrap_or_default(),
            )?)
        },
    )?;
    mt.set("__call", decasify)?;
    exports.set_metatable(Some(mt))?;
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let version = lua.create_string(version)?;
    exports.set("version", version)?;
    Ok(exports)
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Chunk {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        let chunk = match value {
            LuaValue::String(s) => s.to_string_lossy(),
            _ => String::from(""),
        }
        .into();
        Ok(chunk)
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Locale {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => s.try_into()?,
            LuaValue::Nil => Self::default(),
            _ => value.to_string().unwrap_or_default().try_into()?,
        }
        .into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for Case {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => s.try_into()?,
            LuaValue::Nil => Self::default(),
            _ => value.to_string().unwrap_or_default().try_into()?,
        }
        .into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for StyleGuide {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::String(s) => s.try_into()?,
            LuaValue::Nil => Self::default(),
            _ => value.to_string().unwrap_or_default().try_into()?,
        }
        .into()
    }
}

#[cfg_attr(docsrs, doc(cfg(feature = "luamodule")))]
impl FromLua for StyleOptions {
    fn from_lua(value: LuaValue, _: &Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Table(t) => {
                let mut builder = StyleOptionsBuilder::new();
                if let Ok(overrides_table) = t.get::<LuaTable>("overrides") {
                    let overrides: Vec<Word> = overrides_table
                        .sequence_values::<String>()
                        .collect::<LuaResult<Vec<_>>>()?
                        .into_iter()
                        .map(|s| s.into())
                        .collect();
                    builder = builder.overrides(overrides);
                }
                builder.build()
            }
            LuaValue::Nil => Self::default(),
            _ => value.to_string().unwrap_or_default().try_into()?,
        }
        .into()
    }
}
