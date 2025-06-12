" SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
" SPDX-License-Identifier: LGPL-3.0-only

" Ignore this file from NeoVIM which will run the Lua base plugin instead
if has('nvim')
  finish
endif

if executable('decasify') == 0
  echoerr 'decasify: external command not found in $PATH; plugin disabled'
  finish
endif

" :Decasify [args]            – current line
" :<range>Decasify [args]     – explicit range
" Visual-select then :Decasify [args]
" (passes all [args] to `decasify`)
command! -range -nargs=* Decasify <line1>,<line2>!decasify <f-args>
