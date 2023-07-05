use clap::CommandFactory;

use decasify::cli::Cli;
use decasify::to_titlecase;

fn main() -> decasify::Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let locale: String = if matches.contains_id("locale") {
        matches.get_one::<String>("locale").unwrap().to_string()
    } else {
        String::from("en")
    };
    if matches.contains_id("input") {
        let input: Vec<String> = matches.get_many::<String>("input").unwrap().cloned().collect();
        let input = input.join(" ");
        let output = to_titlecase(&input, &locale);
        eprintln!("{input} → {output}");
    }
    Ok(())
}
