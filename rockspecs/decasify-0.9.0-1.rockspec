rockspec_format = "1.0"
package = "decasify"
version = "0.9.0-1"

source = {
   url = "git+https://github.com/alerque/decasify.git",
   dir = "decasify",
   tag = "v0.9.0",
}

description = {
   summary = "Lua C module built from the Rust decasify crate to cast strings to title-case according to locale specific style guides including Turkish support",
   detailed = [[
      A Lua library exposed as a C module built from the Rust decasify crate.
      Provides casing functions for long strings (not just individual words)
      supporting the grammatical style guides of various locales including Turkish.
   ]],
   license = "LGPL-3.0-only",
   homepage = "https://github.com/alerque/decasify",
}

dependencies = {
   "lua >= 5.1",
   "luarocks-build-rust-mlua >= 0.2.3-1",
}

build = {
   type = "rust-mlua",
   modules = {
      "decasify",
   },
}
