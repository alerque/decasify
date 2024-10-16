// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::types::Result;
use regex::Regex;
use std::{borrow::Cow, error, fmt, fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq)]
pub enum Segment {
    Separator(String),
    Word(String),
}

#[derive(Debug, Clone)]
pub struct Chunk {
    pub segments: Vec<Segment>,
}

fn split_chunk(src: &str) -> Chunk {
    let mut segments: Vec<Segment> = Vec::new();
    let captures = Regex::new(r"(?<separator>\p{Whitespace}+)|(?<word>\P{Whitespace}+)").unwrap();
    for capture in captures.captures_iter(src) {
        if let Some(m) = capture.name("separator") {
            segments.push(Segment::Separator(m.as_str().to_string()));
        } else if let Some(m) = capture.name("word") {
            segments.push(Segment::Word(m.as_str().to_string()));
        }
    }
    Chunk { segments }
}

impl From<String> for Chunk {
    fn from(src: String) -> Self {
        split_chunk(src.as_ref())
    }
}

impl From<&String> for Chunk {
    fn from(src: &String) -> Self {
        split_chunk(src.as_ref())
    }
}

impl From<&str> for Chunk {
    fn from(src: &str) -> Self {
        split_chunk(src)
    }
}

impl From<&Cow<'_, str>> for Chunk {
    fn from(src: &Cow<'_, str>) -> Self {
        split_chunk(src)
    }
}

impl FromStr for Chunk {
    type Err = Box<dyn error::Error>;
    fn from_str(src: &str) -> Result<Self> {
        Ok(split_chunk(src))
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
