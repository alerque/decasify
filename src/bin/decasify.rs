use decasify::cli::Cli;
use decasify::to_titlecase;

use clap::CommandFactory;
use std::io;
use std::io::BufRead;

fn main() -> decasify::Result<()> {
    let version = option_env!("VERGEN_GIT_DESCRIBE").unwrap_or_else(|| env!("CARGO_PKG_VERSION"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale: String = if matches.contains_id("locale") {
        matches.get_one::<String>("locale").unwrap().to_string()
    } else {
        String::from("en")
    };
    match matches.contains_id("input") {
        true => {
            let input: Vec<String> = matches
                .get_many::<String>("input")
                .unwrap()
                .cloned()
                .collect();
            let input: Vec<String> = vec![input.join(" ")];
            process(input.iter().map(|ln| ln.to_string()), &locale);
        }
        false => process(io::stdin().lock().lines().map(|ln| ln.unwrap()), &locale),
    }
    Ok(())
}

fn process<I: IntoIterator<Item = String>>(strings: I, locale: &str) {
    for string in strings {
        let output = to_titlecase(&string, locale);
        println!("{output}");
    }
}
