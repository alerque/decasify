-- DO NOT EDIT! Modify template decasify.sile.rockspec.in and rebuild with `make decasify.sile-dev-1.rockspec`

rockspec_format = "3.0"
package = "decasify.sile"
version = "dev-1"

source = {
   url = "git+https://github.com/alerque/decasify.git",
   branch = "master",
}

description = {
   summary = "SILE package wrapping decasify crate to cast strings to title-case according to locale specific style guides including Turkish support",
   detailed = [[
      A SILE package that wraps the decasify library into typesetting commands that can easily be used to re-case
      prose content (not just individual words) supporting the grammatical style guides of
      various locales including Turkish.
   ]],
   license = "LGPL-3.0-only",
   homepage = "https://github.com/alerque/decasify",
   issues_url = "https://github.com/alerque/decasify/issues",
   maintainer = "Caleb Maclennan <caleb@alerque.com>",
   labels = { "i18n" },
}

dependencies = {
   "lua >= 5.1",
   "decasify",
}

build = {
   type = "builtin",
   modules = {
      ["sile.packages.decasify"] = "sile/decasify.lua",
   },
}
