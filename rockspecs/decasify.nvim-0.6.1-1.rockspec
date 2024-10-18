rockspec_format = "1.0"
package = "decasify.nvim"
version = "0.6.1-1"

source = {
   url = "git+https://github.com/alerque/decasify.git",
   dir = "decasify",
   tag = "v0.6.1"
}

description = {
   summary = "NeoVIM plugin wrapping decasify crate to cast strings to title-case according to locale specific style guides including Turkish support",
   detailed = [[
      A NeoVIM plugin that wraps the decasify library into an editor command that can easily be used in bindings to
      provide casing functions for long strings (not just individual words) supporting the grammatical style guides of
      various locales including Turkish.
   ]],
   license = "LGPL-3.0-only",
   homepage = "https://github.com/alerque/decasify",
}

dependencies = {
   "lua >= 5.1",
   "decasify"
}

build = {
   type = "builtin",
   copy_directories = {
      "plugin",
   }
}
