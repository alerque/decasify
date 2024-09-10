---
name: release checklist
about: make sure release steps get followed through for each tag
title: Release vX.Y.Z checklist
labels: ''
assignees: alerque
---

- [ ] Bump version in Cargo.toml, `cargo build`, then add manifest and lockfile to stage
- [ ] Generate new rockspec and update changelog, `make SEMVER=X.Y.Z rockspecs CHANGELOG.md`, add release rockspec and changelog to stage
- [ ] Commit release point, `git commit -m "chore: Release vX.Y.Z"`
- [ ] Test and lint, `make test lint`
- [ ] Tag with signature, `git tag -s vX.Y.Z`
- [ ] Push tag, `git push --follow-tags`
- [ ] Publish to crates.io, `cargo publish --locked`
- [ ] Publish to PyPi (needs exported API key), `maturin publish`
- [ ] Publish to NPMJS, `wasm-pack build --features wasm && wasm-pack publish`
- [ ] Download LuaRocks artifact, `wget https://luarocks.org/manifests/alerque/decasify-X.Y.Z-1.src.rock`
- [ ] Download PyPi artifact, `wget https://files.pythonhosted.org/packages/cp312/d/decasify/decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl`
- [ ] Download Release artifacts, `gh release download vX.Y.Z`
- [ ] Sign all generated artifacts, `ls decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl decasify-X.Y.Z-1.src.rock decasify-X.Y.Z.tar.zst decasify-X.Y.Z.zip | xargs -n1 gpg -a --detach-sign`
- [ ] Push artifacts and signatures, `gh release upload vX.Y.Z decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl{,.asc} decasify-X.Y.Z-1.src.rock{,.asc} decasify-X.Y.Z.tar.zst.asc decasify-X.Y.Z.zip.asc`
