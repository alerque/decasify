// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment, Word};
use crate::types::StyleGuide;

use regex::Regex;
use unicode_titlecase::StrTitleCase;

pub use crate::en::lowercase;
pub use crate::en::sentencecase;
pub use crate::en::uppercase;

pub fn titlecase(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault => titlecase_fr(chunk),
        _ => todo!("French implementation doesn't support this style guide."),
    }
}

fn titlecase_fr(chunk: Chunk) -> String {
    let mut segments: Vec<Segment> = Vec::new();
    chunk.clone().segments.into_iter().for_each(|segment| {
        match segment {
            Segment::Separator(_) => segments.push(segment),
            Segment::Word(ref word) => {
                let mut segs = word.word.split("-").peekable();
                while let Some(s) = segs.next() {
                    segments.push(Segment::Word(Word { word: s.into() }));
                    if segs.peek().is_some() {
                        segments.push(Segment::Separator("-".into()));
                    }
                }
            }
        };
    });
    let mut chunk = chunk.clone();
    let mut words = chunk
        .segments
        .iter_mut()
        .filter_map(|segment| match segment {
            Segment::Word(word) => Some(word),
            _ => None,
        })
        .peekable();
    if let Some(word) = words.next() {
        word.word = word.to_titlecase_lower_rest();
    }
    while let Some(word) = words.next() {
        word.word = match words.peek().is_none() {
            true => word.to_titlecase_lower_rest(),
            false => match is_reserved(word) {
                true => word.to_lowercase(),
                false => word.to_titlecase_lower_rest(),
            },
        };
    }
    chunk.into()
}

fn is_reserved(word: &Word) -> bool {
    let word = word.to_lowercase();
    let word = word.as_str();
    // https://github.com/benoitvallon/titlecase-french/blob/83e092e91dccdd39871dfeac0d58dc06d997dabb/config.js#L22
    let lower_case_word_list = vec![
        "le", "la", "les", // definite articles
        "un", "une", "des", // indefinite articles
        "du", "de", "des", // partitive articles
        "au", "aux", "du", "des", // contracted articles
        "ce", "cet", "cette", "ces", // demonstrative adjectives
        "quel", "quels", "quelle", "quelles", // exclamative adjectives
        "mon", "ton", "son", "notre", "votre", "leur", "ma", "ta", "sa", "mes", "tes", "ses",
        "nos", "vos", "leurs", // possessive adjectives
        "mais", "ou", "et", "donc", "or", "ni", "car", "voire",
        // coordinating conjunctions
        "que", "qu", "quand", "comme", "si", "lorsque", "lorsqu", "puisque", "puisqu", "quoique",
        "quoiqu", // subordinating conjunctions
        "à", "chez", "dans", "entre", "jusque", "jusqu", "hors", "par", "pour", "sans", "vers",
        "sur", "pas", "parmi", "avec", "sous", "en", // prepositions
        "je", "tu", "il", "elle", "on", "nous", "vous", "ils", "elles", "me", "te", "se", "y",
        // personal pronouns
        "qui", "que", "quoi", "dont", "où", // relative pronouns
        "ne", // others
    ];
    let lower_case_words = lower_case_word_list.join("|");
    let lower_case_word = Regex::new(format!("^({lower_case_words})$").as_ref()).unwrap();
    lower_case_word.is_match(word)
}

//capitalizedSpecials: [
//  { input: 'À', output: 'A'},
//  { input: 'Â', output: 'A'},
//  { input: 'Ä', output: 'A'},
//  { input: 'É', output: 'E'},
//  { input: 'È', output: 'E'},
//  { input: 'Ê', output: 'E'},
//  { input: 'Ë', output: 'E'},
//  { input: 'Ç', output: 'C'},
//  { input: 'Î', output: 'I'},
//  { input: 'Ï', output: 'I'},
//  { input: 'Ô', output: 'O'},
//  { input: 'Ö', output: 'O'},
//  { input: 'Û', output: 'U'},
//  { input: 'Ü', output: 'U'},
//  { input: 'Ù', output: 'U'}
//],
