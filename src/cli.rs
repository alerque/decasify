use clap::Parser;

/// A CLI tool to convert all-caps strings to title-case or other less aggressive tones that
/// supports Turkish input.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "decasify")]
pub struct Cli {
    /// Locale
    #[clap(short, long)]
    pub locale: Option<String>,

    /// Input string
    pub input: Vec<String>,
}
