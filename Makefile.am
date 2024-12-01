ACLOCAL_AMFLAGS = -I build-aux
AM_DISTCHECK_CONFIGURE_FLAGS = --enable-developer-mode

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)
licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = decasify
decasify_SOURCES  = src/bin/decasify.rs src/content.rs src/cli.rs src/lib.rs src/types.rs src/traits.rs
decasify_SOURCES += src/lua.rs src/python.rs src/wasm.rs
decasify_SOURCES += src/en.rs src/tr.rs
EXTRA_decasify_SOURCES = tests/cli.rs tests/lib.rs
EXTRA_DIST = pyproject.toml spec/decasify_spec.lua tests/test_all.py plugin/decasify.lua sile/decasify.lua
dist_doc_DATA = README.md CHANGELOG.md
dist_license_DATA = LICENSE.md
nodist_man_MANS =

BUILT_SOURCES =

CLEANFILES =

CARGO_FEATURE_ARGS = --features full

DISTCLEANFILES = @AMINCLUDE@

# A classical use of the autoconf-archive include macro would expand
# INC_AMINCLUDE here, but the perl script that inlines include statements
# runs before the automake that organizes logic and performs substitution.
# Consequentially with a substitution here it becomes impossible to use
# automake conditionals and substitutions in the included Makefile fragments.
# By entering the expanded value directly we are ready in time for the inlining
# functionality and hence can use conditionals in included makefile fragments.
include $(top_srcdir)/aminclude.am

SEMVER := $(shell $(CARGO) read-manifest | $(JQ) -r .version)
ROCKREV = 1
TAG := v$(SEMVER)

LUAROCKS_ARGS ?= --tree lua_modules
BUSTEDFLAGS ?=

DEV_SPEC := $(PACKAGE_NAME)-dev-$(ROCKREV).rockspec
DEV_ROCK := $(PACKAGE_NAME)-dev-$(ROCKREV).src.rock
DEV_ROCK_BIN := $(PACKAGE_NAME)-dev-$(ROCKREV).$(CARCH).rock

REL_SPEC := rockspecs/$(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).rockspec
REL_ROCK := $(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).src.rock
REL_ROCK_BIN := $(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).$(CARCH).rock

DEV_SPEC_NVIM := $(PACKAGE_NAME).nvim-dev-$(ROCKREV).rockspec
DEV_ROCK_NVIM := $(PACKAGE_NAME).nvim-dev-$(ROCKREV).src.rock

REL_SPEC_NVIM := rockspecs/$(PACKAGE_NAME).nvim-$(SEMVER)-$(ROCKREV).rockspec
REL_ROCK_NVIM := $(PACKAGE_NAME).nvim-$(SEMVER)-$(ROCKREV).src.rock

DEV_SPEC_SILE := $(PACKAGE_NAME).sile-dev-$(ROCKREV).rockspec
DEV_ROCK_SILE := $(PACKAGE_NAME).sile-dev-$(ROCKREV).src.rock

REL_SPEC_SILE := rockspecs/$(PACKAGE_NAME).sile-$(SEMVER)-$(ROCKREV).rockspec
REL_ROCK_SILE := $(PACKAGE_NAME).sile-$(SEMVER)-$(ROCKREV).src.rock

EXTRA_DIST += decasify.rockspec.in decasify.nvim.rockspec.in decasify.sile.rockspec.in
EXTRA_DIST += $(REL_SPEC) $(REL_SPEC_NVIM) $(REL_SPEC_SILE)

typst-universe: typst/decasify.wasm typst/example.pdf

typst/decasify.wasm: CARGO_TARGET_TRIPLE = wasm32-unknown-unknown
typst/decasify.wasm: CARGO_WASM_DIR = @builddir@/target/$(CARGO_TARGET_TRIPLE)
typst/decasify.wasm: CARGO_FEATURE_ARGS =
typst/decasify.wasm: CARGO_RELEASE_ARGS = --release --locked
typst/decasify.wasm: typst/src/lib.rs
	$(CARGO_ENV) $(CARGO) build $(CARGO_VERBOSE) $(CARGO_FEATURE_ARGS) $(CARGO_RELEASE_ARGS) --target $(CARGO_TARGET_TRIPLE) -p typst
	$(WASMOPT) -Oz $(CARGO_WASM_DIR)/release/typst.wasm -o typst/$(TRANSFORMED_PACKAGE_NAME).wasm

typst/example.pdf: typst/example.typ typst/decasify.wasm
	$(TYPST) compile $<

check: busted pytest

PHONY_DEVELOPER_TARGETS = rockspecs srcrocks rocks install-luarock busted pytest release-preview
.PHONY: $(PHONY_DEVELOPER_TARGETS)

if DEVELOPER_MODE

release-preview:
	$(GITCLIFF) --unreleased

CHANGELOG.md:
	$(GITCLIFF) -p $@ --unreleased -t $(TAG)

decasify-%.md: CHANGELOG.md
	$(SED) -e '/^## \[$*\]/,/^## \[/!d' $< | \
		$(SED) -e '1s/^/Release v$*\n/' | \
		$(SED) -e '/^#/d' -e '/^$$/d' | \
		$(SED) -e 's/\*//g' | \
		$(SED) -e '1a\\' > $@

rockspecs: $(DEV_SPEC) $(DEV_SPEC_NVIM) $(DEV_SPEC_SILE) $(REL_SPEC) $(REL_SPEC_NVIM) $(REL_SPEC_SILE)

srcrocks: $(DEV_ROCK) $(DEV_ROCK_NVIM) $(DEV_ROCK_SILE) $(REL_ROCK) $(REL_ROCK_NVIM) $(REL_ROCK_SILE)

rocks: $(DEV_ROCK_BIN) $(REL_ROCK_BIN)

install-luarock: $(DEV_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) make $<

busted: install-luarock
	eval $$($(LUAROCKS) $(LUAROCKS_ARGS) path)
	set -f; IFS=';'
	packagepath=($${LUA_PATH})
	packagecpath=($${LUA_CPATH})
	$(BUSTED) -v --lpath="$${packagepath[*]};;" --cpath="$${packagecpath[*]};;" $(BUSTEDFLAGS) .

pytest:
	$(UV) venv
	$(UV) pip install pip
	source .venv/bin/activate
	$(MATURIN) develop
	env PYTHONPATH=.venv/lib/python3.12/site-packages/decasify $(PYTEST)

define rockpec_template =
	$(SED) -e "s/@""PACKAGE_NAME""@/$(PACKAGE_NAME)/g" \
		-e "s/@SEMVER@/$(SEMVER)/g" \
		-e "s/@ROCKREV@/$(ROCKREV)/g" \
		-e "s/@TAG@/$(TAG)/g" \
		$< > $@
endef

define dev_rockspec_props =
	$(SED) -i \
		-e '/tag =/s/tag/branch/' \
		-e "1i -- DO NOT EDIT! Modify template $< and rebuild with \`make $@\`\n" \
		$@
endef

define rel_rockspec_props =
	$(SED) -i -e '/url = "git/a\   dir = "@PACKAGE_NAME@",' $@
endef

define downgrade_rockspec =
	$(SED) -i \
		-e '/rockspec_format/s/3.0/1.0/' \
		-e '/issues_url/d' \
		-e '/maintainer/d' \
		-e '/labels/d' \
		$@
endef

$(DEV_SPEC) $(DEV_SPEC_NVIM) $(DEV_SPEC_SILE): override private SEMVER := dev
$(DEV_SPEC) $(DEV_SPEC_NVIM) $(DEV_SPEC_SILE): override private TAG := master

$(DEV_SPEC): $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	$(dev_rockspec_props)

$(DEV_SPEC_NVIM): $(PACKAGE_NAME).nvim.rockspec.in
	$(rockpec_template)
	$(dev_rockspec_props)

$(DEV_SPEC_SILE): $(PACKAGE_NAME).sile.rockspec.in
	$(rockpec_template)
	$(dev_rockspec_props)

$(REL_SPEC): $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	$(rel_rockspec_props)
	$(downgrade_rockspec)

$(REL_SPEC_NVIM): $(PACKAGE_NAME).nvim.rockspec.in
	$(rockpec_template)
	$(rel_rockspec_props)

$(REL_SPEC_SILE): $(PACKAGE_NAME).sile.rockspec.in
	$(rockpec_template)
	$(rel_rockspec_props)
	$(downgrade_rockspec)

$(DEV_ROCK): $(DEV_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

$(DEV_ROCK_BIN): $(DEV_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) make --sign --pack-binary-rock -- $<

$(DEV_ROCK_NVIM): $(DEV_SPEC_NVIM)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

$(DEV_ROCK_SILE): $(DEV_SPEC_SILE)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

$(REL_ROCK): $(REL_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

$(REL_ROCK_BIN): $(REL_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) make --sign --pack-binary-rock -- $<

$(REL_ROCK_NVIM): $(REL_SPEC_NVIM)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

$(REL_ROCK_SILE): $(REL_SPEC_SILE)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack --sign -- $<

else !DEVELOPER_MODE

$(PHONY_DEVELOPER_TARGETS):
	@: $(error "Please reconfigure using --enable-developer-mode to use developer tooling")

endif !DEVELOPER_MODE
