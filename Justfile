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
	./config.status
	maturin build --frozen
	# cp /home/caleb/.local/share/cargo/wheels/decasify-{{semver}}-cp312-cp312-manylinux_2_39_x86_64.whl .
	wasm-pack build --features wasm
	echo cargo publish --locked
	echo git push --atomic origin master v{{semver}}
	echo gh release download v{{semver}}
	echo wget wget https://files.pythonhosted.org/packages/cp312/d/decasify/decasify-$SEMVER-cp312-cp312-manylinux_2_34_x86_64.whl
	echo ls decasify-$SEMVER-cp312-cp312-manylinux_2_34_x86_64.whl decasify{,.nvim}-$SEMVER-1.src.rock decasify-$SEMVER.{tar.zst,zip} | xargs -n1 gpg -a --detach-sign
	echo gh release upload v$SEMVER decasify-$SEMVER-cp312-cp312-manylinux_2_34_x86_64.whl{,.asc} decasify{,.nvim}-$SEMVER-1.src.rock{,.asc} decasify-$SEMVER.{tar.zst,zip}.asc
	echo make CARCH=x86_64 decasify{,.nvim}-{{semver}}-1.src.rock decasify-{{semver}}-1.x86_64.rock
	echo maturin publish --locked
	echo wasm-pack publish
	echo gpg -a --detatch-sign decasify-{{semver}}.{tar.zst,zip}
	echo gh upload create v{{semver}} -t "Decasify v{{semver}}" *.rock{,.asc} *.whl{,.asc} *.{tar.zst,zip}.asc

# vim: set ft=just
