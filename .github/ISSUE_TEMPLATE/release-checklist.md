---
name: release checklist
about: make sure release steps get followed through for each tag
title: Release vX.Y.Z checklist
labels: ''
assignees: alerque
---

- [ ] Update changelog, `git cliff -o CHANGELOG.md -t vX.Y.Z`, add to stage
- [ ] Bump version in Cargo.toml, `cargo build`, then add to stage
- [ ] Generate new rockspe, `make rockspecs SEMVER=X.Y.Z`, add to stage
- [ ] Commit release point
- [ ] Test, `make test`
- [ ] Tag with signature, `git tag -s vX.Y.Z`
- [ ] Push tag, `git push --follow-tags`
- [ ] Publish to crates.io, `cargo publish --locked`
- [ ] Publish to PyPi, `maturin publish`
- [ ] Publish to NPMJS, `wasm-pack build --features wasm && wasm-pack publish`
- [ ] Download LuaRocks artifact, `wget https://luarocks.org/manifests/alerque/decasify-X.Y.Z-1.src.rock`
- [ ] Download PyPi artifact, `wget https://files.pythonhosted.org/packages/cp312/d/decasify/decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl`
- [ ] Download Release artifacts, `gh release download vX.Y.Z`
- [ ] Sign all generated artifacts, `ls decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl decasify-X.Y.Z-1.src.rock decasify-X.Y.Z.tar.zst decasify-X.Y.Z.zip | xargs -n1 gpg -a --detach-sign`
- [ ] Push artifacts and signatures, `gh release upload vX.Y.Z decasify-X.Y.Z-cp312-cp312-manylinux_2_34_x86_64.whl{,.asc} decasify-X.Y.Z-1.src.rock{,.asc} decasify-X.Y.Z.tar.zst.asc decasify-X.Y.Z.zip.asc`
