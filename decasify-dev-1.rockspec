-- DO NOT EDIT! Modify template decasify.rockspec.in and rebuild with `make decasify-dev-1.rockspec`

rockspec_format = "3.0"
package = "decasify"
version = "dev-1"

source = {
   url = "git+https://github.com/alerque/decasify.git",
   branch = "master"
}

description = {
   summary = "Lua C module built from the Rust decasify crate to cast strings to title-case according to locale specific style guides including Turkish support",
   detailed = [[
      A Lua library exposed as a C module built from the Rust decasify crate.
      Provides casing functions for long strings (not just individual words)
      supporting the grammatical style guides of various locales including Turkish.
   ]],
   license = "GPL-3.0-only",
   homepage = "https://github.com/alerque/decasify",
   issues_url = "https://github.com/alerque/decasify/issues",
   maintainer = "Caleb Maclennan <caleb@alerque.com>",
   labels = { "i18n" }
}

dependencies = {
   "lua >= 5.1",
   -- v2.0 broke support for LuaRocks 3.1 under Lua 5.1, pin to old version until fixed
   -- https://github.com/khvzak/luarocks-build-rust-mlua/pull/10
   "luarocks-build-rust-mlua == 0.1.2-1"
}

build = {
   type = "rust-mlua",
   modules = {
      "decasify"
   }
}
