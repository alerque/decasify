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

release semver: pristine
	make rockspecs/decasify{,.nvim}-{{semver}}-1.rockspec
	sed -i -e '/^version/s/".*"/"{{semver}}"/' Cargo.toml
	env SEMVER={{semver}} sed -i -e "/^decasify =/s/\".*\"/\"${SEMVER%\.*}\"/" README.md
	make decasify-{{semver}}.md
	git checkout cl -- CHANGELOG.md
	make decasify-{{semver}}.md
	# make SEMVER={{semver}} CHANGELOG.md decasify-{{semver}}.md
	cargo build
	git add Cargo.{toml,lock} README.md CHANGELOG.md rockspecs/decasify{,.nvim}-{{semver}}-1.rockspec
	git commit -m "chore: Release v{{semver}}"
	git tag -s v{{semver}} -F decasify-{{semver}}.md
	./config.status && make
	maturin build --frozen
	wasm-pack build --features wasm
	echo git push --atomic origin master v{{semver}}
	echo maturin publish --locked
	echo cargo publish --locked
	echo wasm-pack publish
	echo gh release download v{{semver}}
	echo wget wget https://luarocks.org/manifests/alerque/decasify{,.nvim}-{{semver}}-1.src.rock
	echo wget wget https://files.pythonhosted.org/packages/cp312/d/decasify/decasify-{{semver}}-cp312-cp312-manylinux_2_34_x86_64.whl
	echo 'ls decasify-{{semver}}-cp312-cp312-manylinux_2_34_x86_64.whl decasify{,.nvim}-{{semver}}-1.src.rock decasify-{{semver}}.{tar.zst,zip} | xargs -n1 gpg -a --detach-sign'
	echo gh release upload v{{semver}} decasify-{{semver}}-cp312-cp312-manylinux_2_34_x86_64.whl{,.asc} decasify{,.nvim}-{{semver}}-1.src.rock{,.asc} decasify-{{semver}}.{tar.zst,zip}.asc
	echo make CARCH=x86_64 decasify{,.nvim}-{{semver}}-1.src.rock decasify-{{semver}}-1.x86_64.rock

# vim: set ft=just
