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
  for idx in range(0, len(a:000)-1)
    if l:idx == 0
      let l:cmd .= ' --case ' . a:000[l:idx]
    elseif l:idx == 1
      let l:cmd .= ' --locale ' . a:000[l:idx]
    elseif l:idx == 2
      let l:cmd .= ' --style ' . a:000[l:idx]
    endif
  endfor

  execute l:cmd
endfunction

function! s:DecasifyComplete(arg_lead, cmd_line, _) abort
  let l:parts = split(a:cmd_line, '\s\+', 0)
  if !empty(l:parts)
    call remove(l:parts, 0)
  endif
  let l:trailing_space = a:cmd_line[-1:] =~# '\s'
  let l:arg_index = len(l:parts) + (l:trailing_space ? 1 : 0)
  if l:arg_index == 1
    let l:candidates = ['lower', 'sentence', 'title', 'upper']
  elseif l:arg_index == 2
    let l:candidates = ['en', 'es', 'tr']
  elseif l:arg_index == 3
    let l:candidates = ['ap', 'cmos', 'default', 'grubber', 'rae', 'tdk']
  else
    let l:candidates = []
  endif
  return filter(l:candidates, { _, v -> v =~? '^' . a:arg_lead })
endfunction

" :Decasify [args]            – current line
" :<range>Decasify [args]     – explicit range
" Visual-select then :Decasify [args]
" (passes all [args] to `decasify`)
command! -range -nargs=* -complete=customlist,<SID>DecasifyComplete Decasify call <SID>Decasify(<line1>, <line2>, <f-args>)

let g:loaded_decasify = v:true
