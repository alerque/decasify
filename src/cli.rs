use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, PartialEq, Parser, ValueEnum)]
pub enum StyleGuide {
    AP,
    CMOS,
    Gruber,
}

#[derive(Debug, Clone, PartialEq, Parser, ValueEnum)]
pub enum InputLocale {
    EN,
    TR,
}

/// A CLI tool to convert all-caps strings to title-case or other less aggressive tones that
/// supports Turkish input.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "decasify")]
pub struct Cli {
    /// Locale
    #[clap(short, long, value_enum)]
    pub locale: InputLocale,

    /// Style Guide
    #[clap(short, long, value_enum)]
    pub style: Option<StyleGuide>,

    /// Input string
    pub input: Vec<String>,
}
