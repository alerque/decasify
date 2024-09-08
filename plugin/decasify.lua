if vim.g.loaded_decasify then
   return
end

log.trace("Loading decasify")

vim.api.nvim_create_user_command("Decasify", function ()
   print("Run decasify")
end)


print("Loaded decasify")

vim.g.loaded_decasify = true
