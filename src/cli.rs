// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(build)]
use crate::{Case, Locale, StyleGuide};

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
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(Locale))]
    pub locale: Locale,

    /// Target case
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(Case))]
    pub case: Case,

    /// Style Guide
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(StyleGuide))]
    pub style: StyleGuide,

    /// Input string
    pub input: Vec<String>,
}
