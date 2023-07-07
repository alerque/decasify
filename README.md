# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions?workflow=Rust+Test)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions?workflow=Rust+Lint)

A CLI tool to convert strings to title-case or other less aggressive tones that supports Turkish input.

Also provided are library versions as a Rust crate and a Lua Rock.

It was born out of frustration that no tooling seemed to properly support title-casing strings, particularly coming from all-caps Turkish.
Many can handle single words, and many more can handle English, but nothing seemed to be out there for full strings.

Input may be either shell arguments or STDIN.
Currently defaults to title-casing, others to come later.
Currently defaults to English rules, but the Turkish ones are actually more complete because that's my main use case.

``` console
$ decasify -l tr ILIK SU VE İTEN RÜZGARLAR
Ilık Su ve İten Rüzgarlar
$ echo ILIK SU VE İTEN RÜZGARLAR | decasify -l tr
Ilık Su ve İten Rüzgarlar
```

## Use as Rust crate

In your `Cargo.toml` file, be sure to disable default features since they include building the CLI binary:

```toml
[dependencies]
decasify = { version = "0.1", default-features = false }
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
    "decasify",
}

Then import ande use the provided function:

```lua
local decasify = require("decasify")
local input = "ILIK SU VE İTEN RÜZGARLAR"
local output = decasify.titlecase(input, "tr")
print(output)
```
