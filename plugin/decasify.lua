if vim.g.loaded_decasify then
   return
end

local decasify = require("decasify")

vim.api.nvim_create_user_command("Decasify", function (args)
   local first, last = args.line1, args.line2
   local caser = args.fargs[1]:gsub("case$", "") .. "case"
   if type(decasify[caser]) ~= "function" then
      vim.notify(("Decasify doesn't know what case '%s' is."):format(caser))
      return false
   end
   local lines = vim.api.nvim_buf_get_lines(0, first - 1, last, true)
   for i, line in ipairs(lines) do
      lines[i] = decasify[caser](line)
   end
   vim.api.nvim_buf_set_lines(0, first - 1, last, true, lines)
end, { desc = "Pass lines to decasify for recasing prose", nargs = "+", range = true })

vim.g.loaded_decasify = true
