// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::types::Word;

use std::collections::HashSet;
use unicode_titlecase::StrTitleCase;

#[derive(Clone, Debug)]
pub struct ReservedWords {
    words: HashSet<String>,
}

impl ReservedWords {
    pub fn from_slice(words: &[&str]) -> Self {
        Self {
            words: words.iter().map(|&s| s.to_lowercase()).collect(),
        }
    }

    pub fn add_slice(&mut self, words: &[&str]) {
        for &word in words {
            self.words.insert(word.to_lowercase());
        }
    }

    pub fn contains(&self, word: impl AsRef<str>) -> bool {
        self.words.contains(&word.as_ref().to_lowercase())
    }
}

pub trait IsReserved {
    fn is_reserved(&self, reserved_words: &ReservedWords) -> bool;
}

impl IsReserved for Word {
    fn is_reserved(&self, reserved_words: &ReservedWords) -> bool {
        reserved_words.contains(&self.word)
    }
}

pub fn lowercase(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(word) = segment {
            word.word = word.word.to_lowercase()
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
                word.word.to_titlecase_lower_rest()
            } else {
                word.word.to_lowercase()
            }
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
