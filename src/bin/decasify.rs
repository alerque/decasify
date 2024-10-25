// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use decasify::cli::Cli;
use decasify::{lowercase, sentencecase, titlecase, uppercase};
use decasify::{Case, Locale, StyleGuide};

use snafu::prelude::*;

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

#[derive(Snafu)]
enum Error {
    #[snafu(display("Failed to identify input"))]
    Input {},

    #[snafu(display("Failed to resolve a known locale"))]
    Locale {},

    #[snafu(display("Failed to resolve a known case"))]
    Case {},

    #[snafu(display("Failed to resolve a known style guide"))]
    StyleGuide {},
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale = matches.get_one::<Locale>("locale").context(LocaleSnafu)?;
    let case = matches
        .get_one::<Case>("case")
        .context(CaseSnafu)?
        .to_owned();
    let style = matches
        .get_one::<StyleGuide>("style")
        .context(StyleGuideSnafu)?
        .to_owned();
    match matches.contains_id("input") {
        true => {
            let input: Vec<String> = matches
                .get_many::<String>("input")
                .context(InputSnafu)?
                .cloned()
                .collect();
            let input: Vec<String> = vec![input.join(" ")];
            process(input.iter().map(|ln| ln.to_string()), *locale, case, style);
        }
        false => process(
            io::stdin().lock().lines().map(|ln| ln.unwrap()),
            *locale,
            case,
            style,
        ),
    }
    Ok(())
}

fn process<I: IntoIterator<Item = String>>(
    strings: I,
    locale: Locale,
    case: Case,
    style: StyleGuide,
) {
    for string in strings {
        let output = match case {
            Case::Title => titlecase(string, locale, style),
            Case::Lower => lowercase(string, locale),
            Case::Upper => uppercase(string, locale),
            Case::Sentence => sentencecase(string, locale),
            _ => unreachable!(),
        };
        println!("{output}")
    }
}
