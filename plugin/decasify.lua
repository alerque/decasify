if vim.g.loaded_decasify then
   return
end

local decasify = require("decasify")

vim.api.nvim_create_user_command("Decasify", function (args)
   local first, last = args.line1, args.line2
   local lines = vim.api.nvim_buf_get_lines(0, first - 1, last, true)
   for i, line in ipairs(lines) do
      lines[i] = decasify.titlecase(line)
   end
   vim.api.nvim_buf_set_lines(0, first - 1, last, true, lines)
end, { range = true })

vim.g.loaded_decasify = true
