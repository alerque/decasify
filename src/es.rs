// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::get_override;
use crate::types::{StyleGuide, StyleOptions, Word};

use unicode_titlecase::StrTitleCase;

pub fn titlecase(chunk: Chunk, style: StyleGuide, opts: StyleOptions) -> String {
    let articles_prepositions_conjunctions = [
        "a", "al", "ante", "bajo", "con", "contra", "de", "del", "desde", "durante", "e", "el",
        "en", "entre", "hacia", "hasta", "la", "las", "los", "mas", "mediante", "ni", "o", "para",
        "pero", "por", "que", "según", "si", "sin", "so", "sino", "sobre", "tras", "u", "un",
        "una", "unas", "unos", "y",
    ];
    let determiners = [
        "mi", "mis", "nuestro", "nuestra", "nuestros", "nuestras", "tu", "tus", "vuestro",
        "vuestra", "vuestros", "vuestras", "su", "sus",
    ];
    match style {
        StyleGuide::LanguageDefault => {
            titlecase_spanish(chunk, opts, &articles_prepositions_conjunctions)
        }
        StyleGuide::RealAcademiaEspanola => {
            titlecase_spanish(chunk, opts, &articles_prepositions_conjunctions)
        }
        StyleGuide::FundeuRealAcademiaEspanola => {
            let mut combined = articles_prepositions_conjunctions.to_vec();
            combined.extend_from_slice(&determiners);
            titlecase_spanish(chunk, opts, &combined)
        }
        _ => todo!("Spanish implementation doesn't support this style guide."),
    }
}

fn titlecase_spanish(chunk: Chunk, opts: StyleOptions, reserved_words: &[&str]) -> String {
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
                    match is_reserved(word, reserved_words) {
                        true => word.word.to_lowercase(),
                        false => word.word.to_titlecase_lower_rest(),
                    }
                }
        }
    });
    chunk.into()
}

fn is_reserved(word: &Word, reserved_words: &[&str]) -> bool {
    reserved_words.contains(&word.word.to_lowercase().as_str())
}

pub fn lowercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = word.to_lowercase()
        }
    });
    chunk.into()
}

pub fn uppercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = word.to_uppercase()
        }
    });
    chunk.into()
}

pub fn sentencecase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = if !done_first {
                done_first = true;
                word.to_titlecase_lower_rest()
            } else {
                word.to_lowercase()
            }
        }
    });
    chunk.into()
}
