// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

use decasify::cli::Cli;
use decasify::types::Result;
use decasify::{lowercase, sentencecase, titlecase, uppercase};
use decasify::{Case, Locale, StyleGuide, StyleOptions, StyleOptionsBuilder};

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let case = matches
        .get_one::<Case>("case")
        .unwrap_or(&Case::default())
        .to_owned();
    eprintln! {"case: {case:?}"};
    let locale = matches
        .get_one::<Locale>("locale")
        .unwrap_or(&Locale::default())
        .to_owned();
    let style = matches
        .get_one::<StyleGuide>("style")
        .unwrap_or(&StyleGuide::default())
        .to_owned();
    let opts = if let Some(overrides) = matches.get_many::<String>("overrides") {
        StyleOptionsBuilder::new()
            .overrides(overrides.collect())
            .build()
    } else {
        StyleOptions::default()
    };
    match matches.contains_id("input") {
        true => {
            let input: Vec<String> = matches
                .get_many::<String>("input")
                .unwrap()
                .cloned()
                .collect();
            let input: Vec<String> = vec![input.join(" ")];
            process(
                input.iter().map(|ln| ln.to_string()),
                *locale,
                case,
                style,
                opts,
            );
        }
        false => process(
            io::stdin().lock().lines().map(|ln| ln.unwrap()),
            *locale,
            case,
            style,
            opts,
        ),
    }
    Ok(())
}

fn process<I: IntoIterator<Item = String>>(
    strings: I,
    locale: Locale,
    case: Case,
    style: StyleGuide,
    opts: StyleOptions,
) {
    for string in strings {
        let output = match case {
            Case::Title => titlecase(string, locale, style.clone(), opts.clone()),
            Case::Lower => lowercase(string, locale),
            Case::Upper => uppercase(string, locale),
            Case::Sentence => sentencecase(string, locale),
            _ => unreachable!(),
        };
        println!("{output}")
    }
}
