use decasify::to_titlecase;
use mlua::prelude::*;

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
