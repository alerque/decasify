# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_test.yml)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_list.yml)
[![Flake Run Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/nix.yml?branch=master&label=Flake&logo=NixOS)](https://github.com/alerque/decasify/actions/workflows/nix.yml)
[![Lua Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/luacheck.yml?branch=master&label=Luacheck&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/luacheck.yml)
[![Lua Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/busted.yml?branch=master&label=Busted&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/busted.yml)  
[![GitHub tag (latest)](https://img.shields.io/github/v/tag/alerque/decasify)](https://github.com/alerque/decasify/releases)
[![Crates.io (latest)](https://img.shields.io/crates/v/decasify)](https://crates.io/crates/decasify)
[![LuaRocks (latest)](https://img.shields.io/luarocks/v/alerque/decasify)](https://luarocks.org/modules/alerque/decasify)

A CLI utility, Rust crate, Lua Rock, and Python module to cast strings to title-case according to locale specific style guides including Turkish support.

This project was born out of frustration with ALL CAPS TITLES in Markdown that no tooling seemed to properly support casting to title-cased strings, particularly coming from Turkish.
Many tools can handle casing single words, and some others can handle English strings, but nothing seemed to be out there for full Turkish strings.

Currently defaults to title-casing, others to come later.
Currently defaults to English rules, but the Turkish ones are actually more complete because that's my main use case.
Currently defaults to the Gruber style rules, but others are available

For English, three style guides are known: Associated Press (AP), Chicago Manual of Style (CMOS), and John Grubber's Daring Fireball (Gruber).
The Gruber style is by far the most complete.
The CMOS style handles a number of parts of speech has punctuation related issues.
The AP style is largely unimplemented.
Contributions are welcome for better style guide support or further languages.

``` console
$ decasify -l tr ILIK SU VE İTEN RÜZGARLAR
Ilık Su ve İten Rüzgarlar
$ echo ILIK SU VE İTEN RÜZGARLAR | decasify -l tr
Ilık Su ve İten Rüzgarlar
$ echo foo BAR AND baz: an alter ego | decasify -l en -s gruber
Foo BAR and Baz: An Alter Ego
```

## Use as a CLI tool

Use of the CLI is pretty simple.
Input may be either shell arguments or STDIN.

```console
$ decasify --help
A CLI tool to convert all-caps strings to title-case or other less aggressive tones that supports
Turkish input

Usage: decasify [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Input string

Options:
  -l, --locale <LOCALE>  Locale [default: EN] [possible values: EN, TR]
  -c, --case <CASE>      Target case [default: Title] [possible values: Lower, Sentence, Title,
                         Upper]
  -s, --style <STYLE>    Style Guide [possible values: ap, cmos, gruber]
  -h, --help             Print help
  -V, --version          Print version
```

First, check your distro for packages, e.g. for Arch Linux get it [from the AUR](https://aur.archlinux.org/packages/decasify).

Otherwise for many platforms you can run it directly or install it to a shell using Nix Flakes:

``` console
$ nix run github:alerque/decasify
```

To do a full install from source, grab the tarball attached to the [latest release](https://github.com/alerque/decasify/releases/latest) or use Git to clone the repository.
Don't use the "source code" zip/tar.gz files linked from releases, go for the `tar.zst` source file.
If you use a Git close, first run `./bootstrap.sh` after checkout.
This isn't needed in the source release tarballs.
Next, configure and install with:

```console
$ ./configure
$ make
$ sudo make install
```

Note that installing from source has the advantage of include a man page and shell completions.
All the usual autotools options apply, see `--help` for details.
The most commonly used option especially for distro packagers is probably `--prefix /usr` to change the install location from the default of `/usr/local`.

Of course the bare binary can also be installed directly with Cargo:

```console
$ cargo install --features cli decasify
```

## Use as Rust crate

In your `Cargo.toml` file.

```toml
[dependencies]
decasify = "0.5"
```

Then use the crate functions and types in your project something like this:

```rust
use decasify::to_titlecase;
use decasify::{InputLocale, StyleGuide};

fn demo() {
    let input = "ILIK SU VE İTEN RÜZGARLAR";
    let output = to_titlecase(input, InputLocale::TR, None);
    eprintln! {"{output}"};
    let input = "title with a twist: a colon";
    let output = to_titlecase(input, InputLocale::EN, Some(StyleGuide::DaringFireball));
    eprintln! {"{output}"};
}
```

## Use as Lua Rock

Depend on the LuaRock in your project or install with `luarocks install decasify`:

```lua
dependencies = {
   "decasify"
}
```

Then import and use the provided function:

```lua
local decasify = require("decasify")
local input = "ILIK SU VE İTEN RÜZGARLAR"
local output = decasify.titlecase(input, "tr")
print(output)
input = "title with a twist: a colon"
output  = decasify.titlecase(input, "en", "gruber")
print(output)
```

## Use as Python Module

Depend on the Python module in your project or install with `pip install decasify`:

```toml
[project]
dependencies = [
  "decasify"
]
```

Then import and use the provided function and classes:

```python
import decasify import *

input = "ILIK SU VE İTEN RÜZGARLAR"
output = titlecase(input, InputLocale.TR)
print(output)
input = "title with a twist: a colon"
output  = titlecase(input, InputLocale.EN, StyleGuide.DaringFireball)
print(output)
```
