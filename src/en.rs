// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::types::StyleGuide;

use regex::Regex;
use titlecase::titlecase as gruber_titlecase;
use unicode_titlecase::StrTitleCase;

pub fn titlecase(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault => titlecase_gruber(chunk),
        StyleGuide::AssociatedPress => titlecase_ap(chunk),
        StyleGuide::ChicagoManualOfStyle => titlecase_cmos(chunk),
        StyleGuide::DaringFireball => titlecase_gruber(chunk),
        _ => todo!("English implementation doesn't support this style guide."),
    }
}

fn titlecase_ap(chunk: Chunk) -> String {
    eprintln!("AP style guide not implemented, string returned as-is!");
    chunk.to_string()
}

fn titlecase_cmos(chunk: Chunk) -> String {
    let mut done_first = false;
    let mut chunk = chunk.clone();
    let mut segments = chunk.segments.iter_mut().peekable();
    while let Some(segment) = segments.next() {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_titlecase_lower_rest()
            } else if segments.peek().is_none() {
                // TODO: I think a bug is hiding here since peek() might give us a separator
                // that happens to be a trailing trivia. We need a custom iterator or peeker
                // that knows how to answer about first/last *word* segments.
                s.to_titlecase_lower_rest()
            } else {
                match is_reserved(s) {
                    true => s.to_lowercase(),
                    false => s.to_titlecase_lower_rest(),
                }
            }
        }
    }
    chunk.to_string()
}

fn titlecase_gruber(chunk: Chunk) -> String {
    // The titlecase crate we are going to delegate to here trims the input. We need to restore
    // leading and trailing whitespace ourselves.
    let leading_trivia = if let Some(Segment::Separator(s)) = chunk.segments.first() {
        s.to_string()
    } else {
        String::from("")
    };
    let trailing_trivia = if let Some(Segment::Separator(s)) = chunk.segments.last() {
        s.to_string()
    } else {
        String::from("")
    };
    let titilized = gruber_titlecase(&chunk.to_string());
    format!("{}{}{}", leading_trivia, titilized, trailing_trivia)
}

fn is_reserved(word: &str) -> bool {
    let word = word.to_lowercase();
    let word = word.as_str();
    let article = Regex::new(r"^(a|an|the)$").unwrap();
    let congunction = Regex::new(r"^(for|and|nor|but|or|yet|so|both|either|neither|not only|whether|after|although|as|as if|as long as|as much as|as soon as|as though|because|before|by the time|even if|even though|if|in order that|in case|in the event that|lest|now that|once|only|only if|provided that|since|so|supposing|that|than|though|till|unless|until|when|whenever|where|whereas|wherever|whether or not|while)$").unwrap();
    let preposition = Regex::new(r"^(about|above|across|after|against|along|among|around|at|before|behind|between|beyond|but|by|concerning|despite|down|during|except|following|for|from|in|including|into|like|near|of|off|on|onto|out|over|past|plus|since|throughout|to|towards|under|until|up|upon|up|to|with|within|without)$").unwrap();
    article.is_match(word) || congunction.is_match(word) || preposition.is_match(word)
}

pub fn lowercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_lowercase()
        }
    });
    chunk.to_string()
}

pub fn uppercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_uppercase()
        }
    });
    chunk.to_string()
}

pub fn sentencecase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_titlecase_lower_rest()
            } else {
                s.to_lowercase()
            }
        }
    });
    chunk.to_string()
}
