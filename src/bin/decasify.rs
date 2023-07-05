use clap::CommandFactory;

use decasify::cli::Cli;

fn main() -> decasify::Result<()> {
    let version = option_env!("VERGEN_GIT_SEMVER").unwrap_or_else(|| env!("VERGEN_BUILD_SEMVER"));
    let app = Cli::command().version(version);
    let matches = app.get_matches();
    let _positionals = matches.get_many::<String>("input");
    Ok(())
}
