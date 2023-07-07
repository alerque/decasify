# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_test.yml)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_list.yml)
[![Flake Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/nix.yml?branch=master&label=Flake&logo=NixOS)](https://github.com/alerque/decasify/actions/workflows/nix.yml)
[![Luacheck Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/nix.yml?branch=master&label=Luacheck&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/luacheck.yml)

A CLI utility, Rust crate, and Lua module to cast strings to title-case according to locale specific style guides including Turkish support.

This project was born out of frustration with ALL CAPS TITLES in Markdown that no tooling seemed to properly support casting to title-casing strings, particularly coming from Turkish.
Many tools can handle casing single words, and many others can handle English strings, but nothing seemed to be out there for full Turkish strings.

Input may be either shell arguments or STDIN.
Currently defaults to title-casing, others to come later.
Currently defaults to English rules, but the Turkish ones are actually more complete because that's my main use case.

``` console
$ decasify -l tr ILIK SU VE İTEN RÜZGARLAR
Ilık Su ve İten Rüzgarlar
$ echo ILIK SU VE İTEN RÜZGARLAR | decasify -l tr
Ilık Su ve İten Rüzgarlar
```

## Use as a binary

First, check your distro for packages, e.g. for Arch Linux get it [from the AUR](https://aur.archlinux.org/packages/decasify).

Otherwise for most *nix platforms you can run it directly or install it to a shell using Nix Flakes:

``` console
$ nix run github:alerque/decasify
```

To install from source, grab the tarball or Git clone:

```console
# If using a Git clone (not needed for tarball releases):
$ ./bootstrap.sh
$ ./configure
$ make
$ sudo make install
```
Of course all the usual autotools options apply such as setting a prefix to install to.
Note the source installation will include a man page, and shell completions.

Of course the bare binary can also be installed with Cargo:

```console
$ cargo install --features cli decasify
```

## Use as Rust crate

In your `Cargo.toml` file.

```toml
[dependencies]
decasify = "0.1"
```

```rust
use decasify::to_titlecase;

fn main() {
    let input = "ILIK SU VE İTEN RÜZGARLAR";
    let output = to_titlecase(input, "tr");
    eprintln!{"{output}"};
}
```

## Use as Lua rock

Depend on the LuaRock in your project or install with `luarocks install decasify`:

```lua
dependencies = {
   "decasify"
}

Then import ande use the provided function:

```lua
local decasify = require("decasify")
local input = "ILIK SU VE İTEN RÜZGARLAR"
local output = decasify.titlecase(input, "tr")
print(output)
```
