ACLOCAL_AMFLAGS = -I build-aux
AM_DISTCHECK_CONFIGURE_FLAGS = --enable-developer-mode

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)
licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)

bin_PROGRAMS = decasify
decasify_SOURCES = src/bin/decasify.rs src/cli.rs src/lib.rs src/types.rs
EXTRA_decasify_SOURCES = tests/cli.rs
EXTRA_DIST =
dist_doc_DATA = README.md
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

SEMVER = $(shell $(AWK) -F\" '/^version/ { print $$2 }' $(top_srcdir)/Cargo.toml)
ROCKREV = 1
TAG ?= v$(SEMVER)

LUAROCKS_ARGS ?= --tree lua_modules

DEV_SPEC = $(PACKAGE_NAME)-dev-$(ROCKREV).rockspec
DEV_ROCK = $(PACKAGE_NAME)-dev-$(ROCKREV).src.rock
REL_SPEC = rockspecs/$(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).rockspec
REL_ROCK = $(PACKAGE_NAME)-$(SEMVER)-$(ROCKREV).src.rock

EXTRA_decasify_SOURCES += $(REL_SPEC) $(DEV_SPEC)

PHONY_DEVELOPER_TARGETS = rockspecs srcrocks install-luarock busted
.PHONY: $(PHONY_DEVELOPER_TARGETS)

if DEVELOPER_MODE

rockspecs: $(DEV_SPEC) $(REL_SPEC)

srcrocks: $(DEV_ROCK) $(REL_ROCK)

install-luarock: $(DEV_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) make $<

busted: install-luarock
	$$(eval $(LUAROCKS) $(LUAROCKS_ARGS) path)
	busted -c -v

define rockpec_template =
	$(SED) -e "s/@""PACKAGE_NAME""@/$(PACKAGE_NAME)/g" \
		-e "s/@SEMVER@/$(SEMVER)/g" \
		-e "s/@ROCKREV@/$(ROCKREV)/g" \
		-e "s/@TAG@/$(TAG)/g" \
		$< > $@
endef

$(DEV_SPEC): SEMVER = dev
$(DEV_SPEC): TAG = master
$(DEV_SPEC): $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	$(SED) -i \
		-e '/tag =/s/tag/branch/' \
		-e "1i -- DO NOT EDIT! Modify template $< and rebuild with \`make $@\`\n" \
		$@

rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: SEMVER = $*
rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: TAG = v$*
rockspecs/$(PACKAGE_NAME)-%-$(ROCKREV).rockspec: $(PACKAGE_NAME).rockspec.in
	$(rockpec_template)
	$(SED) -i \
		-e '/rockspec_format/s/3.0/1.0/' \
		-e '/url = "git/a\   dir = "@PACKAGE_NAME@",' \
		-e '/issues_url/d' \
		-e '/maintainer/d' \
		-e '/labels/d' \
		$@

$(PACKAGE_NAME)-dev-$(ROCKREV).src.rock: $(DEV_SPEC)
	$(LUAROCKS) $(LUAROCKS_ARGS) pack $<

$(PACKAGE_NAME)-%.src.rock: rockspecs/$(PACKAGE_NAME)-%.rockspec
	$(LUAROCKS) $(LUAROCKS_ARGS) pack $<

else !DEVELOPER_MODE

$(PHONY_DEVELOPER_TARGETS):
	@: $(error "Please reconfigure using --enable-developer-mode to use developer tooling")

endif !DEVELOPER_MODE
