#[cfg(build)]
use crate::{InputLocale, StyleGuide, TargetCase};

use clap::{builder, Parser};
use strum::VariantNames;

#[macro_export]
macro_rules! clap_enum_variants {
    ($e: ty) => {{
        use builder::TypedValueParser;
        builder::PossibleValuesParser::new(<$e>::VARIANTS).map(|s| s.parse::<$e>().unwrap())
    }};
}

/// A CLI tool to convert all-caps strings to title-case or other less aggressive tones that
/// supports Turkish input.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "decasify")]
pub struct Cli {
    /// Locale
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(InputLocale))]
    pub locale: InputLocale,

    /// Target case
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(TargetCase))]
    pub case: TargetCase,

    /// Style Guide
    #[clap(short, long, ignore_case = true, value_parser = clap_enum_variants!(StyleGuide))]
    pub style: Option<StyleGuide>,

    /// Input string
    pub input: Vec<String>,
}
