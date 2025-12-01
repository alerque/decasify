// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#[cfg(build)]
use crate::{Case, Locale, StyleGuide};

use clap::builder::styling::{AnsiColor, Styles};
use clap::{builder, Parser};
use strum::VariantNames;

#[macro_export]
macro_rules! clap_enum_variants {
    ($e: ty) => {{
        use builder::TypedValueParser;
        builder::PossibleValuesParser::new(<$e>::VARIANTS).map(|s| s.parse::<$e>().unwrap())
    }};
}

/// Convert prose strings to other cases following locale specific rule sets.
///
/// Can convert input in any supported language from any case to any other case.
#[derive(Parser, Debug)]
#[clap(author, bin_name = "decasify")]
pub struct Cli {
    /// The locale of the input text
    ///
    /// Used to identify what language-specific handling needs to be done. This can affect how
    /// individual Unicode characters are coveted to other cases as well as change which style
    /// guides are considered.
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(Locale))]
    pub locale: Locale,

    /// The desired output case
    ///
    /// What case to convert to. Note the output *can* be affected by the input case, so in some
    /// cases (pun intended) you may need to to either process twice or avoid doing so depending on
    /// the expected result. For example title-casing in some English styles tries to preserve
    /// capitalized acronyms, but lower-casing does not. First converting to lower-case then to
    /// title-case would give a different result than directly to title-case.
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(Case))]
    pub case: Case,

    /// Proffered style guide
    ///
    /// For languages with more than one style guide, this picks which set of guidelines to follow.
    /// Each language will have a default (typically the one we have the most robust implementation
    /// for or implemented first).
    #[clap(short, long, default_value_t, ignore_case = true, value_parser = clap_enum_variants!(StyleGuide))]
    pub style: StyleGuide,

    /// Override words
    ///
    /// Override the output case of specific words regardless of what the casing function or style
    /// guide would normally do with them. The case used to specify these words is used in the
    /// output regardless of the target style or input case.
    #[clap(short = 'O', long, num_args(1..))]
    pub overrides: Option<Vec<String>>,

    /// The input string or strings (note STDIN also accepted)
    ///
    /// Note that all input arguments are processed together joined with a space, and STDIN streams
    /// are processed line by line. This can effect the output, for example if using sentence case
    /// the input should be on a single line, not broken across several.
    pub input: Vec<String>,
}

pub const STYLES: Styles = Styles::styled()
    .header(AnsiColor::Magenta.on_default().bold())
    .usage(AnsiColor::Yellow.on_default().bold())
    .literal(AnsiColor::BrightCyan.on_default().bold())
    .placeholder(AnsiColor::Cyan.on_default())
    .error(AnsiColor::BrightRed.on_default().bold())
    .valid(AnsiColor::BrightGreen.on_default().bold())
    .invalid(AnsiColor::BrightYellow.on_default().bold());
