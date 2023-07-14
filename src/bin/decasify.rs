use decasify::cli::Cli;
use decasify::to_titlecase;
use decasify::types::{InputLocale, Result, StyleGuide};

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale = matches.get_one::<InputLocale>("locale").unwrap().to_owned();
    let style = matches.get_one::<StyleGuide>("style").map(|s| s.to_owned());
    match matches.contains_id("input") {
        true => {
            let input: Vec<String> = matches
                .get_many::<String>("input")
                .unwrap()
                .cloned()
                .collect();
            let input: Vec<String> = vec![input.join(" ")];
            process(input.iter().map(|ln| ln.to_string()), locale, style);
        }
        false => process(
            io::stdin().lock().lines().map(|ln| ln.unwrap()),
            locale,
            style,
        ),
    }
    Ok(())
}

fn process<I: IntoIterator<Item = String>>(
    strings: I,
    locale: InputLocale,
    style: Option<StyleGuide>,
) {
    for string in strings {
        let output = to_titlecase(&string, locale.clone(), style.clone());
        println!("{output}");
    }
}
