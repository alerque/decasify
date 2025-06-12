" SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
" SPDX-License-Identifier: LGPL-3.0-only

" Ignore this file from NeoVIM which will run the Lua base plugin instead
if has('nvim') && empty(g:decasify_force_cli)
  finish
endif

if executable('decasify') == 0
  echoerr 'decasify: external command not found in $PATH; plugin disabled'
  finish
endif

function! s:Decasify(startln, endln, ...) range abort
  let l:cmd = printf('%d,%d!decasify', a:startln, a:endln)

  if exists('b:decasify_case') && !empty(b:decasify_case)
    let l:cmd .= ' --case ' . shellescape(b:decasify_case)
  elseif exists('g:decasify_case') && !empty(g:decasify_case)
    let l:cmd .= ' --case ' . shellescape(g:decasify_case)
  endif

  if exists('b:decasify_locale') && !empty(b:decasify_locale)
    let l:cmd .= ' --locale ' . shellescape(b:decasify_locale)
  elseif exists('g:decasify_locale') && !empty(g:decasify_locale)
    let l:cmd .= ' --locale ' . shellescape(g:decasify_locale)
  endif

  if exists('b:decasify_style') && !empty(b:decasify_style)
    let l:cmd .= ' --style ' . shellescape(b:decasify_style)
  elseif exists('g:decasify_style') && !empty(g:decasify_style)
    let l:cmd .= ' --style ' . shellescape(g:decasify_style)
  endif

  if exists('b:decasify_overrides') && !empty(b:decasify_overrides)
    let l:cmd .= ' --overrides ' . shellescape(b:decasify_overrides)
  elseif exists('g:decasify_overrides') && !empty(g:decasify_overrides)
    let l:cmd .= ' --overrides ' . shellescape(g:decasify_overrides)
  endif

  " append any extra args passed by the user
  for l:arg in a:000
    let l:cmd .= ' ' . shellescape(l:arg)
  endfor

  execute l:cmd
endfunction

" :Decasify [args]            – current line
" :<range>Decasify [args]     – explicit range
" Visual-select then :Decasify [args]
" (passes all [args] to `decasify`)
command! -range -nargs=* Decasify call <SID>Decasify(<line1>, <line2>, <f-args>)
