# decasify

[![Rust Test Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_test.yml?branch=master&label=Rust+Test&logo=Rust)](https://github.com/alerque/decasify/actions?workflow=Rust+Test)
[![Rust Lint Status](https://img.shields.io/github/actions/workflow/status/alerque/decasify/rust_lint.yml?branch=master&label=Rust+Lint&logo=Rust)](https://github.com/alerque/decasify/actions?workflow=Rust+Lint)

A CLI tool to convert strings to title-case or other less aggressive tones that supports Turkish input.

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
