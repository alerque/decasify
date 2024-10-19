// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use decasify::cli::Cli;
use decasify::{to_lowercase, to_sentencecase, to_titlecase, to_uppercase};
use decasify::{Case, Locale, Result, StyleGuide};

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale = matches.get_one::<Locale>("locale").unwrap();
    let case = matches.get_one::<Case>("case").unwrap().to_owned();
    let style = matches.get_one::<StyleGuide>("style").unwrap().to_owned();
    match matches.contains_id("input") {
        true => {
            let input: Vec<String> = matches
                .get_many::<String>("input")
                .unwrap()
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
            Case::Title => to_titlecase(string, locale, style),
            Case::Lower => to_lowercase(string, locale),
            Case::Upper => to_uppercase(string, locale),
            Case::Sentence => to_sentencecase(string, locale),
            _ => unreachable!(),
        };
        println!("{output}")
    }
}
