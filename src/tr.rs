// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::types::StyleGuide;

use regex::Regex;
use unicode_titlecase::tr_az::StrTrAzCasing;
use unicode_titlecase::StrTitleCase;

pub fn to_titlecase(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault => {
            let mut chunk = chunk.clone();
            let mut done_first = false;
            chunk.segments.iter_mut().for_each(|segment| {
                if let Segment::Word(s) = segment {
                    *s = if !done_first {
                        done_first = true;
                        s.to_string().to_titlecase_tr_or_az_lower_rest()
                    } else {
                        match is_reserved(s.to_string()) {
                            true => s.to_string().to_lowercase_tr_az(),
                            false => s.to_titlecase_tr_or_az_lower_rest(),
                        }
                    }
                }
            });
            chunk.to_string()
        }
        _ => todo!("Turkish implementation doesn't support different style guides."),
    }
}

fn is_reserved(word: String) -> bool {
    let baglac = Regex::new(
        r"^([Vv][Ee]|[İi][Ll][Ee]|[Yy][Aa]|[Vv][Ee]|[Yy][Aa][Hh][Uu][Tt]|[Kk][İi]|[Dd][AaEe])$",
    )
    .unwrap();
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])([Dd][İiIıUuÜü][Rr]([Ll][AaEe][Rr])?|[Ss][İiIıUuÜü][Nn]|[Yy][İiIıUuÜü][Zz]|[Ss][İiIıUuÜü][Nn][İiIıUuÜü][Zz]|[Ll][AaEe][Rr])?$").unwrap();
    let word = word.as_str();
    baglac.is_match(word) || soruek.is_match(word)
}

pub fn to_lowercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_lowercase_tr_az()
        }
    });
    chunk.to_string()
}

pub fn to_uppercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_uppercase_tr_az()
        }
    });
    chunk.to_string()
}

pub fn to_sentencecase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_string().to_titlecase_tr_or_az_lower_rest()
            } else {
                s.to_string().to_lowercase_tr_az()
            }
        }
    });
    chunk.to_string()
}
