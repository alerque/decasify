std = "max"
include_files = {
   "**/*.lua",
   "**/*.rockspec",
   ".busted",
   ".luacheckrc",
}
exclude_files = {
   ".install",
   ".lua",
   ".luarocks",
   "lua_modules",
}
files["**/*_spec.lua"] = {
   std = "+busted",
}
files["plugin/*.lua"] = {
   globals = {
      "vim",
   }
}
files["sile/*.lua"] = {
   std = "+sile",
}
max_line_length = false
