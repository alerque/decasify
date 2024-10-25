// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use crate::types::{Case, Locale, StyleGuide};

pub trait Decasify {
    fn to_case(
        &self,
        case: impl Into<Case>,
        locale: impl Into<Locale>,
        style: impl Into<StyleGuide>,
    ) -> String;
    fn to_titlecase(&self, locale: impl Into<Locale>, style: impl Into<StyleGuide>) -> String;
    fn to_lowercase(&self, locale: impl Into<Locale>) -> String;
    fn to_uppercase(&self, locale: impl Into<Locale>) -> String;
    fn to_sentencecase(&self, locale: impl Into<Locale>) -> String;
}

impl Decasify for str {
    fn to_case(
        &self,
        case: impl Into<Case>,
        locale: impl Into<Locale>,
        style: impl Into<StyleGuide>,
    ) -> String {
        crate::case(self, case, locale, style)
    }
    fn to_titlecase(&self, locale: impl Into<Locale>, style: impl Into<StyleGuide>) -> String {
        crate::titlecase(self, locale, style)
    }
    fn to_lowercase(&self, locale: impl Into<Locale>) -> String {
        crate::lowercase(self, locale)
    }
    fn to_uppercase(&self, locale: impl Into<Locale>) -> String {
        crate::uppercase(self, locale)
    }
    fn to_sentencecase(&self, locale: impl Into<Locale>) -> String {
        crate::sentencecase(self, locale)
    }
}
