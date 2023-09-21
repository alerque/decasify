ACLOCAL_AMFLAGS = -I build-aux

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)
docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = decasify
decasify_SOURCES = src/bin/decasify.rs src/lib.rs src/cli.rs src/types.rs
EXTRA_decasify_SOURCES =
EXTRA_DIST =
dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
dist_man_MANS =

BUILT_SOURCES =

CLEANFILES =

CARGO_FEATURE_ARGS = --features full

include $(top_srcdir)/build-aux/rust_boilerplate.mk

SEMVER = $(shell $(AWK) -F\" '/^version/ { print $$2 }' Cargo.toml)
ROCKREV = 1
TAG ?= v$(SEMVER)

LUAROCKS_ARGS ?= --local --tree lua_modules

DEV_SPEC = $(PACKAGE_NAME)-dev-$(ROCKREV).rockspec
DEV_ROCK = $(PACKAGE_NAME)-dev-$(ROCKREV).src.rock
REL_SPEC = rockspecs/$(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).rockspec
REL_ROCK = $(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).src.rock

EXTRA_decasify_SOURCES += $(REL_SPEC) $(DEV_SPEC)

.PHONY: rockspecs
rockspecs: $(DEV_SPEC) $(REL_SPEC)

.PHONY: install-luarock
install-luarock: $(DEV_SPEC)
	luarocks $(LUAROCKS_ARGS) make $<

define rockpec_template =
	sed -e "s/@""PACKAGE_NAME""@/$(PACKAGE_NAME)/g" \
		-e "s/@SEMVER@/$(SEMVER)/g" \
		-e "s/@ROCKREV@/$(ROCKREV)/g" \
		-e "s/@TAG@/$(TAG)/g" \
		$< > $@
endef

$(DEV_SPEC): SEMVER = dev
$(DEV_SPEC): TAG = master
$(DEV_SPEC): $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	sed -i \
		-e '/tag =/s/tag/branch/' \
		-e "1i -- DO NOT EDIT! Modify template $< and rebuild with \`make $@\`\n" \
		$@

rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: SEMVER = $*
rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: TAG = v$*
rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	sed -i \
		-e '/rockspec_format/s/3.0/1.0/' \
		-e '/url = "git/a\   dir = "@PACKAGE_NAME@",' \
		-e '/issues_url/d' \
		-e '/maintainer/d' \
		-e '/labels/d' \
		$@

$(PACKAGE_NAME)-dev-$(ROCKREV).src.rock: $(DEV_SPEC)
	luarocks $(LUAROCKS_ARGS) pack $<

$(PACKAGE_NAME)-%.src.rock: rockspecs/$(PACKAGE_NAME)-%.rockspec
	luarocks $(LUAROCKS_ARGS) pack $<
