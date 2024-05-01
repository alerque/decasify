use decasify::cli::Cli;
use decasify::{to_lowercase, to_titlecase, to_uppercase};
use decasify::{InputLocale, Result, StyleGuide, TargetCase};

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

fn main() -> Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale = matches.get_one::<InputLocale>("locale").unwrap().to_owned();
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
            process(input.iter().map(|ln| ln.to_string()), locale, case, style);
        }
        false => process(
            io::stdin().lock().lines().map(|ln| ln.unwrap()),
            locale,
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
        match case {
            TargetCase::Title => {
                let output = to_titlecase(&string, locale.clone(), style.clone());
                println!("{output}")
            }
            TargetCase::Lower => {
                let output = to_lowercase(&string, locale.clone());
                println!("{output}")
            }
            TargetCase::Upper => {
                let output = to_uppercase(&string, locale.clone());
                println!("{output}")
            }
            _ => eprintln!("Target case {case:?} not implemented!"),
        }
    }
}
