set ignore-comments := true
set shell := ["zsh", "+o", "nomatch", "-ecu"]
set unstable := true
set script-interpreter := ["zsh", "+o", "nomatch", "-eu"]

_default:
	@just --list --unsorted

[private]
[doc('Block execution if Git working tree isn’t pristine.')]
pristine:
	# Ensure there are no changes in staging
	git diff-index --quiet --cached HEAD || exit 1
	# Ensure there are no changes in the working tree
	git diff-files --quiet || exit 1

[private]
[doc('Block execution if Git working tree for Typst packages for isn’t pristine.')]
[working-directory: "../typst/packages"]
typst-pristine:
	# Ensure there are no changes in staging
	git diff-index --quiet --cached HEAD || exit 1
	# Ensure there are no changes in the working tree
	git diff-files --quiet || exit 1

[private]
[doc('Block execution if we don’t have access to private keys.')]
keys:
	gpg -a --sign > /dev/null <<< "test"
	test -v MATURIN_PYPI_TOKEN

release semver: pristine keys
	cargo-set-version set-version {{semver}}
	taplo format Cargo.toml
	sed -i -e "/^decasify =/s/\".*\"/\"${${:-{{semver}}}%\.*}\"/" README.md
	make SEMVER={{semver}} rockspecs CHANGELOG.md decasify-{{semver}}.md -B
	git add -f Cargo.{toml,lock} README.md CHANGELOG.md rockspecs/decasify{,.nvim,.sile}-{{semver}}-1.rockspec
	git commit -m "chore: Release v{{semver}}"
	git tag -s v{{semver}} -F decasify-{{semver}}.md
	cargo build
	git diff-files --quiet || exit 1
	./config.status && make
	make typst-package
	maturin build --frozen
	wasm-pack build --features wasm
	git push --atomic origin master v{{semver}}
	maturin publish --locked
	cargo publish --locked
	wasm-pack publish

post-release semver: keys (release-typst semver)
	wget https://files.pythonhosted.org/packages/cp313/d/decasify/decasify-{{semver}}-cp313-cp313-manylinux_2_34_x86_64.whl
	wget https://luarocks.org/manifests/alerque/decasify{,.nvim,.sile}-{{semver}}-1.src.rock
	gh release download v{{semver}} --skip-existing
	ls decasify-{{semver}}-cp313-cp313-manylinux_2_34_x86_64.whl decasify{,.nvim,.sile}-{{semver}}-1.src.rock decasify-{{semver}}.{tar.zst,zip} | xargs -n1 gpg -a --detach-sign
	gh release upload v{{semver}} decasify-{{semver}}-cp313-cp313-manylinux_2_34_x86_64.whl{,.asc} decasify{,.nvim,.sile}-{{semver}}-1.src.rock{,.asc} decasify-{{semver}}.{tar.zst,zip}.asc

[private]
typst-release semver: pristine keys
	make SEMVER={{semver}} typst-release

[working-directory: "../typst/packages/packages/"]
release-typst semver: typst-pristine keys (typst-release semver)
	mkdir -p preview/decasify/{{semver}}
	rsync -av --delete {../../../decasify/,}preview/decasify/{{semver}}/
	git add preview/decasify/{{semver}}
	git commit -m "decasify:{{semver}}" ||:

# vim: set ft=just
