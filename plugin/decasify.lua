-- SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
-- SPDX-License-Identifier: LGPL-3.0-only

-- Ignore this Lua file if somebody attempts to load it in VIM instead of NeoVIM
-- or if the vimscript one (that depends on the CLI instead of a LuaRock) is forced
if not vim or vim.g.decasify_force_cli then
   return
end

if vim.g.loaded_decasify then
   return
end

local decasify = require("decasify")

local function replace_visual_selection (buffer, callback)
   local lpos = vim.fn.getpos("'<")
   local rpos = vim.fn.getpos("'>")
   local lines = vim.api.nvim_buf_get_lines(buffer, lpos[2] - 1, rpos[2], true)
   for i, line in ipairs(lines) do
      if #lines == 1 then
         local recased = callback(line:sub(lpos[3], rpos[3]))
         lines[i] = line:sub(buffer, lpos[3] - 1) .. recased .. line:sub(rpos[3] + 1)
      elseif i == 1 then
         local recased = callback(line:sub(lpos[3]))
         lines[i] = line:sub(buffer, lpos[3] - 1) .. recased
      elseif i == #lines then
         local recased = callback(line:sub(buffer, rpos[3]))
         lines[i] = recased .. line:sub(rpos[3] + 1)
      else
         lines[i] = callback(line)
      end
   end
   vim.api.nvim_buf_set_lines(buffer, lpos[2] - 1, rpos[2], true, lines)
end

local function replace_line_range (buffer, args, callback)
   local first, last = args.line1, args.line2
   local lines = vim.api.nvim_buf_get_lines(buffer, first - 1, last, true)
   for i, line in ipairs(lines) do
      lines[i] = callback(line)
   end
   vim.api.nvim_buf_set_lines(buffer, first - 1, last, true, lines)
end

local function nvim_decasify (args, preview_ns, preview_buffer)
   local case = args.fargs[1] or vim.b.decasify_case or vim.g.decasify_case or nil
   local locale = args.fargs[2] or vim.b.decasify_locale or vim.g.decasify_locale or nil
   local style = args.fargs[3] or vim.b.decasify_style or vim.g.decasify_style or nil
   local overrides = vim.b.decasify_overrides or vim.g.decasify_overrides or {}
   local opts = {
      overrides = args.fargs[4] and vim.split(args.fargs[4], ",") or overrides,
   }
   local buffer = preview_ns and preview_buffer or vim.api.nvim_get_current_buf()
   local decase = function (input)
      return decasify.case(input, case, locale, style, opts)
   end
   -- https://www.petergundel.de/neovim/lua/hack/2023/12/17/get-neovim-mode-when-executing-a-command.html
   local smark = vim.api.nvim_buf_get_mark(buffer, "<")[2]
   local emark = vim.api.nvim_buf_get_mark(buffer, ">")[2]
   if args.count == -1 or smark > 1000000 or emark > 1000000 then
      replace_line_range(buffer, args, decase)
   else
      replace_visual_selection(buffer, decase)
   end
end

local function nvim_decasify_complete (arg_lead, cmd_line, _)
   local parts = vim.split(cmd_line, "%s+", { trimempty = true })
   table.remove(parts, 1)
   local trailing_space = cmd_line:sub(-1):match("%s") ~= nil
   local arg_index = #parts + (trailing_space and 1 or 0)
   local function filter (list)
      return vim.tbl_filter(function (item)
         return item:find("^" .. arg_lead)
      end, list)
   end
   if arg_index == 1 then
      return filter({ "lower", "sentence", "title", "upper" })
   elseif arg_index == 2 then
      return filter({ "en", "es", "tr" })
   elseif arg_index == 3 then
      return filter({ "ap", "cmos", "default", "fundeu", "grubber", "rae", "tdk" })
   else
      return {}
   end
end

vim.api.nvim_create_user_command("Decasify", nvim_decasify, {
   desc = "Pass lines to decasify for recasing prose",
   nargs = "*",
   range = true,
   preview = nvim_decasify,
   complete = nvim_decasify_complete,
})

vim.g.loaded_decasify = true
