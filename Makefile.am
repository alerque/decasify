ACLOCAL_AMFLAGS = -I build-aux

.ONESHELL:
.SECONDARY:
.SECONDEXPANSION:
.DELETE_ON_ERROR:

licensedir = $(datarootdir)/licenses/$(TRANSFORMED_PACKAGE_NAME)
docdir = $(datarootdir)/doc/$(TRANSFORMED_PACKAGE_NAME)

dist_man_MANS = decasify.1
dist_doc_DATA = README.md
dist_license_DATA = LICENSE.md
decasify_SOURCES = Cargo.toml build.rs src/bin/decasify.rs src/lib.rs src/cli.rs
EXTRA_decasify_SOURCES = Cargo.lock .version
bin_PROGRAMS = decasify
EXTRA_DIST = build-aux/git-version-gen

BUILT_SOURCES = .version
CLEANFILES = $(bin_PROGRAMS) $(dist_man_MANS) $(BUILT_SOURCES) .version-prev

if DEBUG_RELEASE
CARGO_RELEASE_ARGS = --features full
else
CARGO_RELEASE_ARGS = --release --locked --features full
endif

CARGO_ENV = CARGO_TARGET_DIR=@abs_top_builddir@/target
RUST_BIN = @abs_top_builddir@/target/@RUST_TARGET_SUBDIR@/$(PACKAGE_NAME)
_RUST_OUT = @abs_top_builddir@/target/@RUST_TARGET_SUBDIR@/.cargo_out_dir
COMPLETIONS_OUT_DIR = completions

if ENABLE_BASH_COMPLETION
bashcompletiondir = $(BASH_COMPLETION_DIR)
nodist_bashcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_bashcompletion_DATA)
endif

if ENABLE_FISH_COMPLETION
fishcompletiondir = $(FISH_COMPLETION_DIR)
nodist_fishcompletion_DATA = $(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish
CLEANFILES += $(nodist_fishcompletion_DATA)
endif

if ENABLE_ZSH_COMPLETION
zshcompletiondir = $(ZSH_COMPLETION_DIR)
nodist_zshcompletion_DATA = $(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME)
CLEANFILES += $(nodist_zshcompletion_DATA)
endif

export DECASIFY_VERSION = v$(VERSION)
DECASIFY := $(PACKAGE_NAME)$(EXEEXT)

# Leave some tips for cargo to use so CLI knows where it is
export CONFIGURE_PREFIX = $(prefix)/
export CONFIGURE_DATADIR = $(datadir)/
export CONFIGURE_BINDIR = $(bindir)/

CARGO_VERBOSE = $(cargo_verbose_$(V))
cargo_verbose_ = $(cargo_verbose_$(AM_DEFAULT_VERBOSITY))
cargo_verbose_0 =
cargo_verbose_1 = --verbose

$(COMPLETIONS_OUT_DIR):
	$(MKDIR_P) $@

$(DECASIFY): $(RUST_BIN)
	$(INSTALL) $(RUST_BIN) $@

$(PACKAGE_NAME).1: $(RUST_BIN)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$@ $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME): $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).bash $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).elv: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).elv $@

$(COMPLETIONS_OUT_DIR)/$(TRANSFORMED_PACKAGE_NAME).fish: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/$(PACKAGE_NAME).fish $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME).ps1: $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/_$(PACKAGE_NAME).ps1 $@

$(COMPLETIONS_OUT_DIR)/_$(TRANSFORMED_PACKAGE_NAME): $(RUST_BIN) | $(COMPLETIONS_OUT_DIR)
	$(INSTALL) -D $$(cat $(_RUST_OUT))/$(COMPLETIONS_OUT_DIR)/_$(PACKAGE_NAME) $@

$(_RUST_OUT) $(RUST_BIN): $(decasify_SOURCES) $(EXTRA_decasify_SOURCES)
	cd $(top_srcdir)
	$(CARGO_ENV) $(CARGO) build $(CARGO_VERBOSE) $(CARGO_RELEASE_ARGS)
	$(CARGO_ENV) $(CARGO) build --quiet --message-format=json $(CARGO_RELEASE_ARGS) |
		$(JQ) -sr 'map(select(.reason == "build-script-executed")) | last | .out_dir' > $(_RUST_OUT)

_BRANCH_REF != $(AWK) '{print ".git/" $$2}' .git/HEAD 2>/dev/null ||:

.version: $(_BRANCH_REF)
	@if [ -e "$(srcdir)/.tarball-version" ]; then \
		printf "$(VERSION)" > $@; \
	else \
		touch "$@-prev"; \
		if [ -e "$@" ]; then \
			cp "$@" "$@-prev"; \
		fi; \
		./build-aux/git-version-gen "$(srcdir)/.tarball-version" > $@; \
		$(CMP) -s "$@" "$@-prev" || autoreconf configure.ac --force; \
	fi

dist-hook:
	printf "$(VERSION)" > "$(distdir)/.tarball-version"

clean-local:
	cd $(top_srcdir)
	$(CARGO_ENV) $(CARGO) clean

check-local: cargo-test check-version

cargo-test: $(PACKAGE_NAME)$(EXEEXT)
	cd $(srcdir)
	$(CARGO_ENV) $(CARGO) test --locked

check-version: $(DECASIFY) | .version
	./$(DECASIFY) --version | $(GREP) -Ff .version

installcheck-local:
	./$(TRANSFORMED_PACKAGE_NAME)$(EXEEXT) --version

.PHONY: lint
lint: rustfmt clippy

.PHONY: rustfmt
rustfmt:
	$(GIT) ls-files '*.rs' | $(XARGS) rustfmt --check

.PHONY: clippy
clippy:
	cd $(srcdir)
	$(CARGO_ENV) $(CARGO) clippy -- -D warnings

SEMVER = $(shell tomlq -r .package.version Cargo.toml)
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
