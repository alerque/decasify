" SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
" SPDX-License-Identifier: LGPL-3.0-only

" Ignore this file from NeoVIM which will run the Lua base plugin instead
if has('nvim') && (!exists('g:decasify_force_cli') || !g:decasify_force_cli)
  finish
endif

if exists('g:loaded_decasify')
  finish
endif

if executable('decasify') == 0
  echoerr 'decasify: external command not found in $PATH; plugin disabled'
  finish
endif

function! s:Decasify(startln, endln, ...) range abort
  let l:cmd = printf('%d,%d!decasify', a:startln, a:endln)

  let l:opts = ['case','locale','style','overrides']
  for l:opt in l:opts
    let l:val = get(b:, 'decasify_' . l:opt, get(g:, 'decasify_' . l:opt, ''))
    if empty(l:val)
      continue
    endif

    if type(l:val) == v:t_list
      let l:val_str = join(l:val, ' ')
    else
      let l:val_str = l:val
    endif

    let l:cmd .= ' --' . l:opt . ' ' . shellescape(l:val_str)
  endfor

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

let g:loaded_decasify = v:true
