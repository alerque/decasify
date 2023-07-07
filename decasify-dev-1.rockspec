-- DO NOT EDIT! Modify template decasify.rockspec.in and rebuild with `make decasify-dev-1.rockspec`

rockspec_format = "3.0"
package = "decasify"
version = "dev-1"

source = {
   url = "git+https://github.com/alerque/decasify.git",
   branch = "master"
}

description = {
   summary = "Lua wrapper around decasify string casing library",
   detailed = [[A Lua wrapper around the Rust decasify library,
      a tool to convert all-caps strings to title-case or other less aggressive tones that supports Turkish input.]],
   license = "GPL-3.0-only",
   homepage = "https://github.com/alerque/decasify",
   issues_url = "https://github.com/alerque/decasify/issues",
   maintainer = "Caleb Maclennan <caleb@alerque.com>",
   labels = { "i18n" }
}

dependencies = {
   "lua >= 5.1",
   "luarocks-build-rust-mlua"
}

build = {
   type = "rust-mlua",
   modules = {
      "decasify"
   }
}
