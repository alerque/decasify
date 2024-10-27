// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::content::{Chunk, Segment};
use crate::types::StyleGuide;

pub use crate::en::lowercase;
pub use crate::en::sentencecase;
pub use crate::en::uppercase;

pub fn titlecase(_chunk: Chunk, _style: StyleGuide) -> String {
    todo!();
}
