// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment, Word};
use crate::types::StyleGuide;

use regex::Regex;
use unicode_titlecase::tr_az::StrTrAzCasing;
use unicode_titlecase::StrTitleCase;

pub fn titlecase(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault => titlecase_tdk(chunk),
        StyleGuide::TurkishLanguageInstitute => titlecase_tdk(chunk),
        _ => todo!("Turkish implementation doesn't support different style guides."),
    }
}

fn titlecase_tdk(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = if !done_first {
                done_first = true;
                word.to_titlecase_tr_or_az_lower_rest()
            } else {
                match is_reserved(word) {
                    true => word.word.to_lowercase_tr_az(),
                    false => word.word.to_titlecase_tr_or_az_lower_rest(),
                }
            }
        }
    });
    chunk.into()
}

fn is_reserved(word: &Word) -> bool {
    let word = word.to_string();
    let word = word.as_ref();
    let baglac =
        Regex::new(r"^([Vv][Ee]|[İi][Ll][Ee]|[Yy][Aa]|[Yy][Aa][Hh][Uu][Tt]|[Kk][İi]|[Dd][AaEe])$")
            .unwrap();
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])([Dd][İiIıUuÜü][Rr]([Ll][AaEe][Rr])?|[Ss][İiIıUuÜü][Nn]|[Yy][İiIıUuÜü][Zz]|[Ss][İiIıUuÜü][Nn][İiIıUuÜü][Zz]|[Ll][AaEe][Rr])?$").unwrap();
    baglac.is_match(word) || soruek.is_match(word)
}

pub fn lowercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = word.word.to_lowercase_tr_az()
        }
    });
    chunk.into()
}

pub fn uppercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = word.word.to_uppercase_tr_az()
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
                word.word.to_titlecase_tr_or_az_lower_rest()
            } else {
                word.word.to_lowercase_tr_az()
            }
        }
    });
    chunk.into()
}
