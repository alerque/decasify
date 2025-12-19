cargo := require('cargo')
cargo-set-version := require('cargo-set-version')
gh := require('gh')
git := require('git')
gpg := require('gpg')
just := just_executable()
luarocks := require('luarocks')
make := require('make')
maturin := require('maturin')
nvim := require('nvim')
rsync := require('rsync')
rustfmt := require('rustfmt')
stylua := require('stylua')
taplo := require('taplo')
vim := require('vim')
wasm-pack := require('wasm-pack')
wget := require('wget')

set script-interpreter := ['zsh', '+o', 'nomatch', '-eu']
set shell := ['zsh', '+o', 'nomatch', '-ecu']
set unstable := true

[default]
[private]
@list:
    {{ just }} --list --unsorted

nuke-n-pave:
    {{ git }} clean -dxff -e .husky -e .fonts -e .sources -e node_modules -e target -e completions
    ./bootstrap.sh

dev-conf: nuke-n-pave
    ./configure --enable-developer-mode --enable-debug
    {{ make }}

rel-conf: nuke-n-pave
    ./configure --enable-developer-mode
    {{ make }}

build:
    {{ make }} build

check:
    {{ make }} check

lint:
    {{ make }} lint

perfect:
    {{ make }} build check lint

restyle:
    {{ git }} ls-files '*.lua' '*.lua.in' '*.rockspec.in' .busted .luacov .luacheckrc build-aux/config.ld | xargs {{ stylua }} --respect-ignores
    {{ git }} ls-files '*.rs' '*.rs.in' | xargs {{ rustfmt }} --edition 2021 --config skip_children=true
    {{ git }} ls-files '*.toml' | xargs {{ taplo }} format

[doc('Block execution if Git working tree isn’t pristine.')]
[private]
pristine: sile-package typst-package
    # Make sure Git's status cache is warmed up
    {{ git }} diff --shortstat
    # Ensure there are no changes in staging
    {{ git }} diff-index --quiet --cached HEAD || exit 1
    # Ensure there are no changes in the working tree
    {{ git }} diff-files --quiet || exit 1

[doc('Block execution if Git working tree for Typst packages isn’t pristine.')]
[private]
[working-directory('../typst/packages')]
typst-pristine:
    # Ensure there are no changes in staging
    {{ git }} diff-index --quiet --cached HEAD || exit 1
    # Ensure there are no changes in the working tree
    {{ git }} diff-files --quiet || exit 1

[doc('Rebuild SILE package (makes sure tracked documentation is up to date).')]
[private]
sile-package:
    {{ make }} sile-package

[doc('Rebuild Typst package (makes sure tracked documentation is up to date).')]
[private]
typst-package:
    {{ make }} typst-package

[no-cd]
preview-nvim *ARGS: (preview nvim + ' --clean' 'plugin/decasify.lua' ARGS)

[no-cd]
preview-vim *ARGS: (preview vim + ' --clean' 'plugin/decasify.vim' ARGS)

[no-cd]
[private]
[script]
preview vimcmd plugin *ARGS:
    {{ make }} decasify rockspecs
    {{ luarocks }} --tree lua_modules --lua-version 5.1 make decasify-dev-1.rockspec
    eval $({{ luarocks }} --tree lua_modules --lua-version 5.1 path)
    {{ vimcmd }} \
    	-c {{ quote("let &runtimepath=\"" + justfile_directory() + ",\" . &runtimepath") }} \
    	-c 'source {{ plugin }}' \
    	{{ ARGS }}

[doc('Block execution if we don’t have access to private keys.')]
[private]
keys:
    {{ gpg }} -a --sign > /dev/null <<< 'test'
    test -v MATURIN_PYPI_TOKEN

release semver: pristine keys
    {{ cargo-set-version }} set-version {{ semver }}
    {{ taplo }} format Cargo.toml
    sed -i -e '/^decasify =/s#".*"#"${${:-{{ semver }}}%\.*}"#' README.md
    sed -i -e '/^#import/s#".*"#"@preview/decasify:{{ semver }}"#' README.md
    {{ make }} SEMVER={{ semver }} rockspecs CHANGELOG.md decasify-{{ semver }}.md -B
    {{ git }} add -f Cargo.{toml,lock} README.md CHANGELOG.md rockspecs/decasify{,.nvim,.sile}-{{ semver }}-1.rockspec
    {{ git }} commit -m 'chore: Release v{{ semver }}'
    {{ git }} tag -s v{{ semver }} -F decasify-{{ semver }}.md
    {{ cargo }} build
    {{ git }} diff-files --quiet || exit 1
    ./config.status && {{ make }}
    {{ make }} typst-package
    {{ maturin }} build --frozen
    {{ wasm-pack }} build --features wasm
    {{ git }} push --atomic origin master v{{ semver }}
    {{ maturin }} publish --locked
    {{ cargo }} publish --locked
    {{ wasm-pack }} publish

post-release semver: keys (release-typst semver)
    {{ wget }} https://files.pythonhosted.org/packages/cp313/d/decasify/decasify-{{ semver }}-cp313-cp313-manylinux_2_34_x86_64.whl
    {{ wget }} https://luarocks.org/manifests/alerque/decasify{,.nvim,.sile}-{{ semver }}-1.src.rock
    {{ gh }} release download v{{ semver }} --skip-existing
    ls decasify-{{ semver }}-cp313-cp313-manylinux_2_34_x86_64.whl decasify{,.nvim,.sile}-{{ semver }}-1.src.rock decasify-{{ semver }}.{tar.zst,zip} | xargs -n1 {{ gpg }} -a --detach-sign
    {{ gh }} release upload v{{ semver }} decasify-{{ semver }}-cp313-cp313-manylinux_2_34_x86_64.whl{,.asc} decasify{,.nvim,.sile}-{{ semver }}-1.src.rock{,.asc} decasify-{{ semver }}.{tar.zst,zip}.asc

[private]
typst-release semver: pristine keys
    {{ make }} SEMVER={{ semver }} typst-release

[working-directory('../typst/packages/packages/')]
release-typst semver: typst-pristine keys (typst-release semver)
    mkdir -p preview/decasify/{{ semver }}
    {{ rsync }} -av --delete {../../../decasify/,}preview/decasify/{{ semver }}/
    {{ git }} add preview/decasify/{{ semver }}
    {{ git }} commit -m 'decasify:{{ semver }}' ||:
