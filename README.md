# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_test.yml)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions/workflows/rust_list.yml)
[![Flake Run Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/nix.yml?branch=master&label=Flake&logo=NixOS)](https://github.com/alerque/decasify/actions/workflows/nix.yml)
[![Lua Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/luacheck.yml?branch=master&label=Luacheck&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/luacheck.yml)
[![Lua Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/busted.yml?branch=master&label=Busted&logo=Lua)](https://github.com/alerque/decasify/actions/workflows/busted.yml)  
[![GitHub tag (latest)](https://img.shields.io/github/v/tag/alerque/decasify?logo=github&color=blue)](https://github.com/alerque/decasify/releases)
[![Crates.io (latest)](https://img.shields.io/crates/v/decasify?logo=rust&color=blue)](https://crates.io/crates/decasify)
[![LuaRocks (latest)](https://img.shields.io/luarocks/v/alerque/decasify?logo=lua&color=blue)][rock]
[![PyPi (latest)](https://img.shields.io/pypi/v/decasify?logo=python&color=blue)](https://pypi.org/project/decasify)
[![NPM Version](https://img.shields.io/npm/v/decasify?logo=npm&color=blue)](https://www.npmjs.com/package/decasify)

A CLI utility, Rust crate, Lua rock, Python module, JavaScript module, Neovim plugin, and SILE package to cast strings to title-case (and other cases) according to locale specific style guides including Turkish support.

This project was born out of frustration with authors and editors leaving ALL CAPS TITLES in Markdown sources.
No tooling I could find properly supported casting these to title-cased strings (which are more versatile for typesetting purposes).
The problem was doubly hard because most of my work is adjacent to Turkish, and even less tooling was available and has special issues with case conversions.
Many tools can handle casing single words, some programmer specific tools handle re-casing tokens and identifiers, and yet a few others can handle *English* strings.
But *nothing* seemed to be out there for changing the case of Turkish prose.

The CLI defaults to titlecase and English, but lower, upper, and sentence case options are also available.
The Rust, Lua, Python, and JavaScript library APIs have functions specific to each operation.
Where possible the APIs currently default to English rules and (for English) the Gruber style guide, but others are available.

The Turkish style follows the Turkish Language Institute's [guidelines][tdk].

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

## Use as a Rust crate

In your `Cargo.toml` file.

```toml
[dependencies]
decasify = "0.7"
```

Then use the crate functions and types in your project something like this:

```rust
use decasify::to_titlecase;
use decasify::{Locale, StyleGuide};

fn demo() {
    let input = "ILIK SU VE İTEN RÜZGARLAR";
    let output = to_titlecase(input, Locale::TR, StyleGuide::LanguageDefault);
    eprintln! {"{output}"};
    let input = "title with a twist: a colon";
    let output = to_titlecase(input, Locale::EN, StyleGuide::DaringFireball);
    eprintln! {"{output}"};
}
```

## Use as a Lua Rock

Depend on [the LuaRock][rock] in your project or install with `luarocks install decasify`:

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

## Use as a Python Module

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
output = titlecase(input, Locale.TR)
print(output)
input = "title with a twist: a colon"
output  = titlecase(input, Locale.EN, StyleGuide.DaringFireball)
print(output)
```

## Use as a JavaScript (WASM) Module

Depend on the WASM based JavaScript module in your project with `npm add decasify`:

Then import and use the provided functions and classes:

```javascript
import { titlecase, uppercase, lowercase, Locale, StyleGuide } from 'decasify';

var input = "ILIK SU VE İTEN RÜZGARLAR"
var output = titlecase(input, Locale.TR)
console.log(output)

var input = "title with a twist: a colon"
var output = titlecase(input, Locale.EN, StyleGuide.DaringFireball)
console.log(output)
```

## Use as a Neovim plugin

* Using [rocks.nvim](https://github.com/nvim-neorocks/rocks.nvim), simply run `:Rocks install decasify.nvim`.

* Using [lazy.nvim](https://lazy.folke.io/), simply add `{ "alerque/decasify" }`

* Using other plugin managers that don't automatically detect dependencies, you will need to manually specify the dependency and/or make sure the Lua Rock for [decasify](https://luarocks.org/modules/alerque/decasify) is available, then use this repository as a plugin however your plugin manager handles that.

    ```lua
    -- for packer.nvim
    use {
       "alerque/decasify",
       rocks = { "decasify" },
    }
    ```

* Using no plugin manager, make sure the [decasify Rock][rock] is installed matching the version of Lua NeoVIM is built with, then copy `plugin/decasify.lua` to wherever your user's plugin directory is.

A new command `:Decasify` will become available (with optional subcommands for cases other than title case) that transforms the current line or any range of lines.
The default case, locale, and style guide can be changed (before or after loading) with global or buffer local variables:

```lua
-- Set the default target case globally
vim.g.decasify_case = "title"
-- Change the locale for the current buffer
vim.b.decasify_locale = "tr"
-- Change the default style guide globally
vim.g.decasify_style = "gruber"
```

## Use as a SILE package

[The SILE Typesetter](https://sile-typesetter.org/) leverages LuaRocks to manage 3rd party packages.
The [decasify.sile][rock.sile] rock can be installed with `luarocks install decasify.sile`.
Typically you'll want to adjust the Lua version to match your SILE installation, perhaps with `luarocks --lua $(sile -q <<< SILE.lua_version) install decasify.sile`.
Additionally you make want to use `--local` to install to your user account instead of the system root, or `--tree lua_modules` to install locally inside a single project.
Loading it in a SILE document uses the usual `\use[module=package.decasify]` (see notes in the SILE manual about setting package paths if you installed via `--local`).
Once loaded the package exposes a `\decasify{}` function that can take any combination of `case`, `locale`, and `style` settings and applies the appropriate transformation to the content.
By default it will track the language of the document content.

[rock]: http://luarocks.org/modules/alerque/decasify
[rock.sile]: http://luarocks.org/modules/alerque/decasify.sile
