// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use decasify::cli::Cli;
use decasify::{to_lowercase, to_sentencecase, to_titlecase, to_uppercase};
use decasify::{InputLocale, Result, StyleGuide, TargetCase};

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale = matches.get_one::<InputLocale>("locale").unwrap();
    let case = matches.get_one::<TargetCase>("case").unwrap().to_owned();
    let style = matches.get_one::<StyleGuide>("style").map(|s| s.to_owned());
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
    locale: InputLocale,
    case: TargetCase,
    style: Option<StyleGuide>,
) {
    for string in strings {
        let output = match case {
            TargetCase::Title => to_titlecase(string, locale, style),
            TargetCase::Lower => to_lowercase(string, locale),
            TargetCase::Upper => to_uppercase(string, locale),
            TargetCase::Sentence => to_sentencecase(string, locale),
        };
        println!("{output}")
    }
}
