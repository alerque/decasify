// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use regex::Regex;
use std::{borrow::Cow, fmt, fmt::Display, str::FromStr};
use unicode_titlecase::StrTitleCase;

use snafu::prelude::*;

#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Chunk {
    pub segments: Vec<Segment>,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Segment {
    Separator(String),
    Word(Word),
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Word {
    pub word: String,
}

#[derive(Snafu)]
pub enum Error {
    #[snafu(display("Unable to cast str to Chunk"))]
    StrToChunk {},
}

// Clap CLI errors are reported using the Debug trait, but Snafu sets up the Display trait.
// So we delegate. c.f. https://github.com/shepmaster/snafu/issues/110
impl std::fmt::Debug for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(self, fmt)
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

fn split_chunk(s: &str) -> Chunk {
    let mut segments: Vec<Segment> = Vec::new();
    let captures = Regex::new(r"(?<separator>\p{Whitespace}+)|(?<word>\P{Whitespace}+)").unwrap();
    for capture in captures.captures_iter(s) {
        if let Some(m) = capture.name("separator") {
            segments.push(Segment::Separator(m.as_str().to_string()));
        } else if let Some(m) = capture.name("word") {
            segments.push(Segment::Word(Word {
                word: m.as_str().to_owned(),
            }));
        }
    }
    Chunk { segments }
}

impl From<String> for Chunk {
    fn from(s: String) -> Self {
        split_chunk(s.as_ref())
    }
}

impl From<&String> for Chunk {
    fn from(s: &String) -> Self {
        split_chunk(s.as_ref())
    }
}

impl From<&str> for Chunk {
    fn from(s: &str) -> Self {
        split_chunk(s)
    }
}

impl From<&Cow<'_, str>> for Chunk {
    fn from(s: &Cow<'_, str>) -> Self {
        split_chunk(s)
    }
}

impl FromStr for Chunk {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        Ok(split_chunk(s))
    }
}

impl From<Chunk> for String {
    fn from(c: Chunk) -> Self {
        let mut s = String::new();
        for segment in c.segments {
            s.push_str(segment.to_string().as_ref());
        }
        s
    }
}

impl Display for Chunk {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for segment in &self.segments {
            fmt.write_str(segment.to_string().as_ref())?;
        }
        Ok(())
    }
}

impl Display for Segment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Segment::Separator(string) => fmt.write_str(string)?,
            Segment::Word(word) => fmt.write_str(word.to_string().as_ref())?,
        };
        Ok(())
    }
}

impl Word {
    pub fn to_lowercase(&self) -> String {
        self.word.to_lowercase()
    }
    pub fn to_uppercase(&self) -> String {
        self.word.to_uppercase()
    }
}

impl From<String> for Word {
    fn from(word: String) -> Self {
        Self { word }
    }
}

impl StrTitleCase for Word {
    fn to_titlecase(&self) -> String {
        self.word.to_titlecase()
    }
    fn to_titlecase_lower_rest(&self) -> String {
        self.word.to_titlecase_lower_rest()
    }
    fn to_titlecase_tr_or_az(&self) -> String {
        self.word.to_titlecase_tr_or_az()
    }
    fn to_titlecase_tr_or_az_lower_rest(&self) -> String {
        self.word.to_titlecase_tr_or_az_lower_rest()
    }
    fn starts_titlecase(&self) -> bool {
        self.word.starts_titlecase()
    }
    fn starts_titlecase_rest_lower(&self) -> bool {
        self.word.starts_titlecase_rest_lower()
    }
}

impl Display for Word {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.word.as_ref())?;
        Ok(())
    }
}
