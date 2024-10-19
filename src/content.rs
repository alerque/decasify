// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::types::Result;
use regex::Regex;
use std::{borrow::Cow, error, fmt, fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Segment {
    Separator(String),
    Word(String),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Chunk {
    pub segments: Vec<Segment>,
}

fn split_chunk(s: &str) -> Chunk {
    let mut segments: Vec<Segment> = Vec::new();
    let captures = Regex::new(r"(?<separator>\p{Whitespace}+)|(?<word>\P{Whitespace}+)").unwrap();
    for capture in captures.captures_iter(s) {
        if let Some(m) = capture.name("separator") {
            segments.push(Segment::Separator(m.as_str().to_string()));
        } else if let Some(m) = capture.name("word") {
            segments.push(Segment::Word(m.as_str().to_string()));
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
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self> {
        Ok(split_chunk(s))
    }
}

impl Display for Segment {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let _ = match self {
            Segment::Separator(string) => fmt.write_str(string),
            Segment::Word(string) => fmt.write_str(string),
        };
        Ok(())
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
