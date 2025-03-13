// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::get_override;
use crate::types::{StyleGuide, StyleGuideOptions, Word};

use regex::Regex;
use titlecase::titlecase as gruber_titlecase;
use unicode_titlecase::StrTitleCase;

pub fn titlecase(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault(opts) => titlecase_gruber(chunk, opts.unwrap_or_default()),
        StyleGuide::AssociatedPress(opts) => titlecase_ap(chunk, opts.unwrap_or_default()),
        StyleGuide::ChicagoManualOfStyle(opts) => titlecase_cmos(chunk, opts.unwrap_or_default()),
        StyleGuide::DaringFireball(opts) => titlecase_gruber(chunk, opts.unwrap_or_default()),
        _ => todo!("English implementation doesn't support this style guide."),
    }
}

fn titlecase_ap(chunk: Chunk, _opts: StyleGuideOptions) -> String {
    eprintln!("AP style guide not implemented, string returned as-is!");
    chunk.into()
}

fn titlecase_cmos(chunk: Chunk, _opts: StyleGuideOptions) -> String {
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

fn titlecase_gruber(chunk: Chunk, opts: StyleGuideOptions) -> String {
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
    let mut titilized = gruber_titlecase(chunk.to_string().as_ref());
    if opts.overrides.is_some() {
        let mut chunk: Chunk = titilized.into();
        chunk.segments.iter_mut().for_each(|segment| {
            if let Segment::Word(word) = segment {
                word.word =
                    if let Some(word) = get_override(word, &opts.overrides, |w| w.to_lowercase()) {
                        word.to_string()
                    } else {
                        word.to_string()
                    }
            }
        });
        titilized = chunk.to_string();
    }
    format!("{}{}{}", leading_trivia, titilized, trailing_trivia)
}

fn is_reserved(word: &Word) -> bool {
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
