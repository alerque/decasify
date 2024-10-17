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
	make rockspecs/decasify-{{semver}}-1.rockspec rockspecs/decasify.nvim-{{semver}}-1.rockspec
	git add action.yml README.md rockspecs/decasify-{{semver}}-1.rockspec rockspecs/decasify.nvim-{{semver}}-1.rockspec
	git commit -m "chore: Release v{{semver}}"
	git tag v{{semver}}
	git push --atomic upstream master v{{semver}}
	luarocks pack rockspecs/decasify-{{semver}}-1.rockspec
	luarocks pack rockspecs/decasify.nvim-{{semver}}-1.rockspec
	gh release create v{{semver}} -t "Decasify v{{semver}}" decasify-{{semver}}-1.src.rock decasify.nvim-{{semver}}-1.src.rock

# vim: set ft=just
