// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::types::{Case, Locale, StyleGuide};

pub trait Decasify {
    fn to_case(&self, case: Case, locale: Locale, style: StyleGuide) -> String;
    fn to_titlecase(&self, locale: Locale, style: StyleGuide) -> String;
    fn to_lowercase(&self, locale: Locale) -> String;
    fn to_uppercase(&self, locale: Locale) -> String;
    fn to_scentencecase(&self, locale: Locale) -> String;
}
