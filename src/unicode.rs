// SPDX-FileCopyrightText: © 2026 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::get_override;
use crate::types::{Locale, StyleOptions};

use unicode_titlecase::tr_az::StrTrAzCasing;
use unicode_titlecase::StrTitleCase;

pub fn titlecase(chunk: Chunk, locale: Locale, opts: StyleOptions) -> String {
    match locale {
        Locale::TR => titlecase_naive_tr(chunk, opts),
        _ => titlecase_naive(chunk, opts),
    }
}

fn titlecase_naive_tr(chunk: Chunk, opts: StyleOptions) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = if let Some(word) =
                get_override(word, &opts.overrides, |w| w.to_lowercase_tr_az())
            {
                word.to_string()
            } else {
                word.word
                    .to_lowercase_tr_az()
                    .to_titlecase_tr_or_az_lower_rest()
            }
        }
    });
    chunk.into()
}

fn titlecase_naive(chunk: Chunk, opts: StyleOptions) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word =
                if let Some(word) = get_override(word, &opts.overrides, |w| w.to_lowercase()) {
                    word.to_string()
                } else {
                    word.word.to_lowercase().to_titlecase()
                }
        }
    });
    chunk.into()
}
