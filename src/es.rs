// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::generics::{IsReserved, ReservedWords};
use crate::get_override;
use crate::types::{StyleGuide, StyleOptions};

use unicode_titlecase::StrTitleCase;

pub use crate::generics::{lowercase, sentencecase, uppercase};

pub fn titlecase(chunk: Chunk, style: StyleGuide, opts: StyleOptions) -> String {
    let rae_reserved = ReservedWords::from_slice(&[
        "a", "al", "ante", "bajo", "con", "contra", "de", "del", "desde", "durante", "e", "el",
        "en", "entre", "hacia", "hasta", "la", "las", "los", "mas", "mediante", "ni", "o", "para",
        "pero", "por", "que", "según", "si", "sin", "so", "sino", "sobre", "tras", "u", "un",
        "una", "unas", "unos", "y",
    ]);
    let mut fundeu_reserved = rae_reserved.clone();
    fundeu_reserved.add_slice(&[
        "mi", "mis", "nuestro", "nuestra", "nuestros", "nuestras", "tu", "tus", "vuestro",
        "vuestra", "vuestros", "vuestras", "su", "sus",
    ]);
    match style {
        StyleGuide::LanguageDefault => titlecase_spanish(chunk, opts, rae_reserved),
        StyleGuide::RealAcademiaEspanola => titlecase_spanish(chunk, opts, rae_reserved),
        StyleGuide::FundeuRealAcademiaEspanola => titlecase_spanish(chunk, opts, fundeu_reserved),
        _ => todo!("Spanish implementation doesn't support this style guide."),
    }
}

fn titlecase_spanish(chunk: Chunk, opts: StyleOptions, reserved: ReservedWords) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word =
                if let Some(word) = get_override(word, &opts.overrides, |w| w.to_lowercase()) {
                    word.to_string()
                } else if !done_first {
                    done_first = true;
                    word.to_titlecase_lower_rest()
                } else {
                    match word.is_reserved(&reserved) {
                        true => word.word.to_lowercase(),
                        false => word.word.to_titlecase_lower_rest(),
                    }
                }
        }
    });
    chunk.into()
}
