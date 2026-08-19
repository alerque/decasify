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
sed := require('sed')
stylua := require('stylua')
taplo := require('taplo')
vim := require('vim')
wasm-pack := require('wasm-pack')
wget := require('wget')

export DECASIFY_BINARY := justfile_directory() + "/decasify"

# By default Just will re-use the user's $SHELL. In order to make use of script
# rules and more advanced shell features we need a more predictable runtime
# environment. This setup is a little more strict than the default shell options
# to make sure we abort if a command in the middle of a job fails, etc.
# Zed shell is used here because this is my personal preference, c.f. BDFL.
# See the Teamtype project for an example accomplishing this using bash.
set script-interpreter := ['zsh', '+o', 'nomatch', '-eu']
set shell := ['zsh', '+o', 'nomatch', '-ecu']

set default-list
set default-script
set positional-arguments
set unstable

# With positional arguments enabled, we can pass all the arguments to the bash
# shell in a way that will get expanded to the original 'word' breakdown. However,
# when we do this blindly in all cases and the job's positional arguments happen
# to be empty the shell decides we must have wanted a placeholder for an empty
# string argument — a construct that is invalid for many of our commands. The
# solution is to decide up front whether we have any positional arguments at all
# and then either not pass anything or pass them in a way that will get expanded
# properly. As a caveat we can't use this workaround for nested jobs that pass
# positional arguments to other jobs since one layer of quoting is lost, but we
# don't need to because none of those happen to use spaces in arguments anyway.
maybe-pass(args) := if args != "" { '"$@"' } else { "" }

# Python wheels target
pyver := "cp314"

nuke-n-pave:
    {{ git }} clean -dxff -e .husky -e .fonts -e .sources -e node_modules -e target -e completions
    ./bootstrap.sh

dev-conf: nuke-n-pave
    ./configure --enable-developer-mode --enable-debug
    {{ make }}

rel-conf: nuke-n-pave
    ./configure --enable-developer-mode
    {{ make }}

[group('build')]
build:
    {{ make }} {{ recipe_name() }}

[group('check')]
check:
    {{ make }} {{ recipe_name() }}

[group('lint')]
lint:
    {{ make }} {{ recipe_name() }}

perfect:
    {{ make }} build check lint

restyle:
    {{ git }} ls-files '*.lua' '*.lua.in' '*.rockspec.in' .busted .luacov .luacheckrc build-aux/config.ld | xargs {{ stylua }} --respect-ignores
    {{ git }} ls-files '*.rs' '*.rs.in' | xargs {{ rustfmt }} --edition 2021 --config skip_children=true
    {{ git }} ls-files '*.toml' | xargs {{ taplo }} format

[doc('Block execution if Git working tree isn’t pristine.')]
[private]
pristine: typst-pristine
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

packages:
    {{ make }} {{ recipe_name() }}

node-package:
    {{ make }} {{ recipe_name() }}

python-package:
    {{ make }} {{ recipe_name() }}

[doc('Rebuild SILE package (makes sure tracked documentation is up to date).')]
[private]
sile-package:
    {{ make }} {{ recipe_name() }}

[doc('Rebuild Typst package (makes sure tracked documentation is up to date).')]
[private]
typst-package:
    {{ make }} {{ recipe_name() }}

# This task will build (if necessary) and run the Decasify CLI from this repository.
# This is especially useful for manual testing and can be used from anywhere by invoking the Justfile externally,
# e.g. with an alias such as:
#
#     alias decasify='just --justfile ~/path/to/decasify/Justfile decasify'
#
# Build and run Decasify CLI for testing (can be used from outside the project).
[no-cd]
decasify *ARGS: build
    $DECASIFY_BINARY {{ maybe-pass(ARGS) }}

[no-cd]
preview-vim *ARGS: (preview vim + ' --clean' 'plugin/decasify.vim' ARGS)

[no-cd]
[private]
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
    {{ sed }} -i -e "/^decasify =/s#\".*\"#\"${${:-{{ semver }}}%\.*}\"#" README.md
    {{ sed }} -i -e '/^#import/s#".*"#"@preview/decasify:{{ semver }}"#' README.md
    {{ make }} SEMVER={{ semver }} rockspecs CHANGELOG.md decasify-{{ semver }}.md -B
    {{ git }} add -f Cargo.{toml,lock} README.md CHANGELOG.md rockspecs/decasify{,.nvim,.sile}-{{ semver }}-1.rockspec
    {{ git }} commit -m 'chore: Release v{{ semver }}'
    {{ git }} tag -s v{{ semver }} -F decasify-{{ semver }}.md
    {{ just }} build packages
    {{ git }} diff-files --quiet || exit 1
    ./config.status && {{ make }}
    {{ git }} push --atomic origin master v{{ semver }}
    {{ maturin }} publish --locked
    {{ cargo }} publish --locked
    {{ wasm-pack }} publish

post-release semver: keys (release-typst semver)
    {{ wget }} https://files.pythonhosted.org/packages/{{ pyver }}/d/decasify/decasify-{{ semver }}-{{ pyver }}-{{ pyver }}-manylinux_2_34_x86_64.whl
    {{ wget }} https://luarocks.org/manifests/alerque/decasify{,.nvim,.sile}-{{ semver }}-1.src.rock
    {{ gh }} release download v{{ semver }} --skip-existing
    ls decasify-{{ semver }}-{{ pyver }}-{{ pyver }}-manylinux_2_34_x86_64.whl decasify{,.nvim,.sile}-{{ semver }}-1.src.rock decasify-{{ semver }}.{tar.zst,zip} | xargs -n1 {{ gpg }} -a --detach-sign
    {{ gh }} release upload v{{ semver }} decasify-{{ semver }}-{{ pyver }}-{{ pyver }}-manylinux_2_34_x86_64.whl{,.asc} decasify{,.nvim,.sile}-{{ semver }}-1.src.rock{,.asc} decasify-{{ semver }}.{tar.zst,zip}.asc

[private]
typst-release semver: pristine keys typst-package
    {{ make }} SEMVER={{ semver }} {{ recipe_name() }}

[working-directory('../typst/packages/packages/')]
release-typst semver: typst-pristine keys (typst-release semver)
    mkdir -p preview/decasify/{{ semver }}
    {{ rsync }} -av --delete {../../../decasify/,}preview/decasify/{{ semver }}/
    {{ git }} add preview/decasify/{{ semver }}
    {{ git }} commit -m 'decasify:{{ semver }}' ||:
