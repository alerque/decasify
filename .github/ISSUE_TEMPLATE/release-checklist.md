---
name: release checklist
about: make sure release steps get followed through for each tag
title: Release vX.Y.Z checklist
labels: ''
assignees: alerque

---

- [ ] Test, `make test`
- [ ] Tag with signature, `git tag -s vX.Y.Z`
- [ ] Push tag, `git push --follow-tags`
- [ ] Publish to crates.io, `cargo publish --locked`
- [ ] Publish to PyPi, `maven publish`
- [ ] Download LuaRocks artifact, `wget ...`
- [ ] Download PyPi artifact, `wget ...`
- [ ] Download Release artifacts, `gh release download vX.Y.Z`
- [ ] Sign all generated artifacts, `gpg -a --detach-sign X.src.rock X.tar.zst X.zip X.whl`
- [ ] Push artifacts and signatures, `gh release upload vX.Y.Z X.src.rock X.whl *.asc`
