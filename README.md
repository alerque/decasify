# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_test.yml)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_list.yml)
[![Flake Run Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/nix.yml?branch=master&label=Flake&logo=NixOS)](https://github.com/alerque/decasify/actions/workflows/nix.yml)
[![Lua Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/luacheck.yml?branch=master&label=Luacheck&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/luacheck.yml)
[![Lua Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/busted.yml?branch=master&label=Busted&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/busted.yml)  
[![GitHub tag (latest)](https://img.shields.io/github/v/tag/alerque/decasify?logo=github&color=blue)](https://github.com/alerque/decasify/releases)
[![Crates.io (latest)](https://img.shields.io/crates/v/decasify?logo=rust&color=blue)](https://crates.io/crates/decasify)
[![LuaRocks (latest)](https://img.shields.io/luarocks/v/alerque/decasify?logo=lua&color=blue)](https://luarocks.org/modules/alerque/decasify)
[![PyPi (latest)](https://img.shields.io/pypi/v/decasify?logo=python&color=blue)](https://pypi.org/project/decasify)
[![NPM Version](https://img.shields.io/npm/v/decasify?logo=npm&color=blue)](https://www.npmjs.com/package/decasify)

A CLI utility, Rust crate, Lua Rock, Python module, and JavaScript module to cast strings to title-case according to locale specific style guides including Turkish support.

This project was born out of frustration with ALL CAPS TITLES in Markdown that no tooling seemed to properly support casting to title-cased strings, particularly coming from Turkish.
Many tools can handle casing single words, and some others can handle English strings, but nothing seemed to be out there for full Turkish strings.

The CLI defaults to titlecase and English, but lower, upper, and sentence case options are also available.
The Rust, Lua, Python, and JavaScript library APIs have functions specific to each operation.
Where possible the APIs currently default to English rules and (for English) the Gruber style rules, but others are available.
The Turkish rules follow Turkish Language Institute's [guidelines][tdk].

For English, three style guides are known: Associated Press (AP), Chicago Manual of Style (CMOS), and John Grubber's Daring Fireball (Gruber).
The Gruber style is by far the most complete, being implemented by the [titlecase crate][titlecase_crate].
The CMOS style handles a number of parts of speech but has punctuation related issues.
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

[tdk]: https://tdk.gov.tr/icerik/yazim-kurallari/buyuk-harflerin-kullanildigi-yerler/
[titlecase_crate]: https://crates.io/crates/titlecase

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

Then import and use the provided functions:

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

Then import and use the provided functions and type classes:

```python
from decasify import *

input = "ILIK SU VE İTEN RÜZGARLAR"
output = titlecase(input, InputLocale.TR)
print(output)
input = "title with a twist: a colon"
output  = titlecase(input, InputLocale.EN, StyleGuide.DaringFireball)
print(output)
```

## Use as JavaScript (WASM) Module

Depend on the WASM based JavaScript module in your project with `npm add decasify`:

Then import and use the provided functions and classes:

```javascript
import { titlecase, uppercase, lowercase, InputLocale, StyleGuide } from 'decasify';

var input = "ILIK SU VE İTEN RÜZGARLAR"
var output = titlecase(input, InputLocale.TR)
console.log(output)

var input = "title with a twist: a colon"
var output = titlecase(input, InputLocale.EN, StyleGuide.DaringFireball)
console.log(output)
```

