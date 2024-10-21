## [0.7.1] - 2024-10-21

### Features

- *(crate)* Add `impl From<&str>` for case variants
- *(crate)* Provide all-purpose casing function with target case as argument
- *(lua)* Provide all-purpose casing function with target case as argument
- *(python)* Provide all-purpose casing function with target case as argument
- *(wasm)* Provide all-purpose casing function with target case as argument
- *(nvim)* Allow overriding locale & style per Decasify command invocation

### Bug Fixes

- Package Lua plugin in source dist
- *(nvim)* Correctly handle buffer-local default overrides

## [0.7.0] - 2024-10-19

### Features

- *(crate)* Be flexible on types by accepting `impl Into<T>`
- Allow Lua/Python/JS to pass similar to 'titlecase' instead of 'title'

### Bug Fixes

- *(nvim)* Correct rockspec_format, NeoVIM plugin only compatible with LuaRocks 3+

### Refator

- [**breaking**] Simplify type names
- [**breaking**] Change Option<StyleGuide> argument to StyleGuide

## [0.6.1] - 2024-10-18

### Features

- Preserve whitespace from inputs
- *(crate)* Be flexible on accepted types and Implement From trait on common possibilities
- *(nvim)* Add a rockspec for installation with rocks.nvim
- *(nvim)* Add a new command that runs decasify titlecasing on input range
- *(nvim)* Add a subcommand to access all casing options
- *(nvim)* Add global default case
- *(nvim)* Add global settings for locale and style guide
- *(nvim)* Allow and prioritize buffer local settings
- *(nvim)* Accept visual input range for partial line transformations

## [0.6.0] - 2024-08-13

### Chore

- [**breaking**] Re-license under GNU Lesser General Public License v3

## [0.5.8] - 2024-07-22

### Bug Fixes

- Make sure TR/AZ reserved words are lower-cased with locale
- Allow freestanding TR question suffixes as reserved without catching unrelated words

## [0.5.7] - 2024-07-15

### Bug Fixes

- *(wasm)* Export missing sentencecase function to JavaScript

## [0.5.6] - 2024-07-15

### Features

- *(wasm)* Implement API as WASM module
- Implement (fairly naive) sentence casing

## [0.5.5] - 2024-07-08

### Bug Fixes

- *(tooling)* Stop CI from pretending to be a source tarball

## [0.5.4] - 2024-07-08

### Bug Fixes

- *(tooling)* Checkout repo history so CI deploy correctly packages source files

## [0.5.3] - 2024-07-08

### Features

- *(tooling)* Use git-cliff to generate a changelog

### Bug Fixes

- *(build)* Package Python manifest in source distribution
- *(lua)* Hold back mlua build version to not break old LuaRocks

## [0.5.2] - 2024-07-07

### Features

- *(python)* Implement API as Python module

### Bug Fixes

- *(build)* Do not distribute main man page, makes dist require Rust tooling
- *(build)* Avoid the perceived need for an extra automake cycle in dist tarball
- *(build)* Make sure rockspec generation still works in source tarballs

## [0.5.1] - 2024-05-24

### Features

- *(lua)* Export version so Lua module can inspect itself
- *(lua)* Bring lowercase() and uppercase() functions to Lua API

## [0.5.0] - 2024-05-01

### Features

- Implement upper and lower case casting for EN vs TR/AZ

### Bug Fixes

- *(build)* Fixup Rust boilerplate so distclean functions

## [0.4.8] - 2024-04-08

### Bug Fixes

- *(build)* Pre-process rust makefile fragments to avoid Automake default rule duplication
- *(build)* Distribute makefile fragments with source packaging

## [0.4.3] - 2023-11-03

### Bug Fixes

- *(build)* Move build-time dependency checks out of runtime dep check configure flag
- *(build)* Make sure build target doesn't exit with success if it actually fails

## [0.4.2] - 2023-09-22

### Bug Fixes

- *(build)* Move git to developer-only dependency

## [0.4.0] - 2023-07-15

### Features

- Map out how target case might look in the API

### Bug Fixes

- Work around build issue in Flake, naersk using build.rs

## [0.3.0] - 2023-07-14

### Features

- *(cli)* Add style option to CLI to pick a style guide
- Implement John Gruber's Daring Fireball style

## [0.2.4] - 2023-07-07

### Features

- Add some English grammar parts to excluded words

## [0.2.2] - 2023-07-07

### Bug Fixes

- *(build)* Bundle rockspecs in releases
- *(build)* Correct version variable breaking buildtime checks

## [0.2.0] - 2023-07-07

### Features

- Add Lua Rockspec tooling to wrap Rust library
- Setup Rust library for use as a Lua module
- Wire up Lua module to actual titlecase function

## [0.1.0] - 2023-07-06

### Features

- Accept input as STDIN if no trailing args

