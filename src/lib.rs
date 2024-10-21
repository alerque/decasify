// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

#![doc = include_str!("../README.md")]

use regex::Regex;
use titlecase::titlecase as gruber_titlecase;
use unicode_titlecase::tr_az::StrTrAzCasing;
use unicode_titlecase::StrTitleCase;

pub mod content;
pub mod types;

pub use content::{Chunk, Segment};
pub use types::{Case, Locale, Result, StyleGuide};

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "luamodule")]
pub mod lua;

#[cfg(feature = "pythonmodule")]
pub mod python;

#[cfg(feature = "wasm")]
pub mod wasm;

/// Convert a string to a specific case following typesetting conventions for a target locale
pub fn to_case(
    chunk: impl Into<Chunk>,
    case: impl Into<Case>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
) -> String {
    let chunk: Chunk = chunk.into();
    let case: Case = case.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    match case {
        Case::Lower => to_lowercase(chunk, locale),
        Case::Upper => to_uppercase(chunk, locale),
        Case::Sentence => to_sentencecase(chunk, locale),
        Case::Title => to_titlecase(chunk, locale, style),
    }
}

/// Convert a string to title case following typesetting conventions for a target locale
pub fn to_titlecase(
    chunk: impl Into<Chunk>,
    locale: impl Into<Locale>,
    style: impl Into<StyleGuide>,
) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    let style: StyleGuide = style.into();
    match locale {
        Locale::EN => to_titlecase_en(chunk, style),
        Locale::TR => to_titlecase_tr(chunk, style),
    }
}

/// Convert a string to lower case following typesetting conventions for a target locale
pub fn to_lowercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => to_lowercase_en(chunk),
        Locale::TR => to_lowercase_tr(chunk),
    }
}

/// Convert a string to upper case following typesetting conventions for a target locale
pub fn to_uppercase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => to_uppercase_en(chunk),
        Locale::TR => to_uppercase_tr(chunk),
    }
}

/// Convert a string to sentence case following typesetting conventions for a target locale
pub fn to_sentencecase(chunk: impl Into<Chunk>, locale: impl Into<Locale>) -> String {
    let chunk: Chunk = chunk.into();
    let locale: Locale = locale.into();
    match locale {
        Locale::EN => to_sentencecase_en(chunk),
        Locale::TR => to_sentencecase_tr(chunk),
    }
}

fn to_titlecase_en(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::AssociatedPress => to_titlecase_ap(chunk),
        StyleGuide::ChicagoManualOfStyle => to_titlecase_cmos(chunk),
        StyleGuide::DaringFireball => to_titlecase_gruber(chunk),
        StyleGuide::LanguageDefault => to_titlecase_gruber(chunk),
    }
}

fn to_titlecase_ap(chunk: Chunk) -> String {
    eprintln!("AP style guide not implemented, string returned as-is!");
    chunk.to_string()
}

fn to_titlecase_cmos(chunk: Chunk) -> String {
    let mut done_first = false;
    let mut chunk = chunk.clone();
    let mut segments = chunk.segments.iter_mut().peekable();
    while let Some(segment) = segments.next() {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_string().to_titlecase_lower_rest()
            } else if segments.peek().is_none() {
                // TODO: I think a bug is hiding here since peek() might give use a separator
                // that happens to be a trailing trivia. We need a custom iterator or peeker
                // that knows how to answer about first/last *word* segments.
                s.to_string().to_titlecase_lower_rest()
            } else {
                match is_reserved_en(s.to_string()) {
                    true => s.to_string().to_lowercase(),
                    false => s.to_string().to_titlecase_lower_rest(),
                }
            }
        }
    }
    chunk.to_string()
}

fn to_titlecase_gruber(chunk: Chunk) -> String {
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

fn to_titlecase_tr(chunk: Chunk, style: StyleGuide) -> String {
    match style {
        StyleGuide::LanguageDefault => {
            let mut chunk = chunk.clone();
            let mut done_first = false;
            chunk.segments.iter_mut().for_each(|segment| {
                if let Segment::Word(s) = segment {
                    *s = if !done_first {
                        done_first = true;
                        s.to_string().to_titlecase_tr_or_az_lower_rest()
                    } else {
                        match is_reserved_tr(s.to_string()) {
                            true => s.to_string().to_lowercase_tr_az(),
                            false => s.to_titlecase_tr_or_az_lower_rest(),
                        }
                    }
                }
            });
            chunk.to_string()
        }
        _ => todo!("Turkish implementation doesn't support different style guides."),
    }
}

fn is_reserved_en(word: String) -> bool {
    let word = word.to_lowercase();
    let word = word.as_str();
    let article = Regex::new(r"^(a|an|the)$").unwrap();
    let congunction = Regex::new(r"^(for|and|nor|but|or|yet|so|both|either|neither|not only|whether|after|although|as|as if|as long as|as much as|as soon as|as though|because|before|by the time|even if|even though|if|in order that|in case|in the event that|lest|now that|once|only|only if|provided that|since|so|supposing|that|than|though|till|unless|until|when|whenever|where|whereas|wherever|whether or not|while)$").unwrap();
    let preposition = Regex::new(r"^(about|above|across|after|against|along|among|around|at|before|behind|between|beyond|but|by|concerning|despite|down|during|except|following|for|from|in|including|into|like|near|of|off|on|onto|out|over|past|plus|since|throughout|to|towards|under|until|up|upon|up|to|with|within|without)$").unwrap();
    article.is_match(word) || congunction.is_match(word) || preposition.is_match(word)
}

fn is_reserved_tr(word: String) -> bool {
    let baglac = Regex::new(
        r"^([Vv][Ee]|[İi][Ll][Ee]|[Yy][Aa]|[Vv][Ee]|[Yy][Aa][Hh][Uu][Tt]|[Kk][İi]|[Dd][AaEe])$",
    )
    .unwrap();
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])([Dd][İiIıUuÜü][Rr]([Ll][AaEe][Rr])?|[Ss][İiIıUuÜü][Nn]|[Yy][İiIıUuÜü][Zz]|[Ss][İiIıUuÜü][Nn][İiIıUuÜü][Zz]|[Ll][AaEe][Rr])?$").unwrap();
    let word = word.as_str();
    baglac.is_match(word) || soruek.is_match(word)
}

fn to_lowercase_en(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_lowercase()
        }
    });
    chunk.to_string()
}

fn to_lowercase_tr(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_lowercase_tr_az()
        }
    });
    chunk.to_string()
}

fn to_uppercase_en(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_uppercase()
        }
    });
    chunk.to_string()
}

fn to_uppercase_tr(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = s.to_string().to_uppercase_tr_az()
        }
    });
    chunk.to_string()
}

fn to_sentencecase_en(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_string().to_titlecase_lower_rest()
            } else {
                s.to_string().to_lowercase()
            }
        }
    });
    chunk.to_string()
}

fn to_sentencecase_tr(chunk: Chunk) -> String {
    let mut chunk = chunk.clone();
    let mut done_first = false;
    chunk.segments.iter_mut().for_each(|segment| {
        if let Segment::Word(s) = segment {
            *s = if !done_first {
                done_first = true;
                s.to_string().to_titlecase_tr_or_az_lower_rest()
            } else {
                s.to_string().to_lowercase_tr_az()
            }
        }
    });
    chunk.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cast_from_str() {
        let res = to_titlecase("FIST", "en", "gruber");
        assert_eq!(res, "Fist");
        let res = to_titlecase("FIST", "tr", "");
        assert_eq!(res, "Fıst");
        let res = to_titlecase("FIST", "tr", "default");
        assert_eq!(res, "Fıst");
    }

    #[test]
    fn cast_from_legacy_option() {
        let res = to_titlecase("FIST", "en", Some(StyleGuide::DaringFireball));
        assert_eq!(res, "Fist");
        let res = to_titlecase("FIST", "en", None);
        assert_eq!(res, "Fist");
    }

    macro_rules! case {
        ($name:ident, $case:expr, $locale:expr, $style:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_case($input, $case, $locale, $style);
                assert_eq!(actual, $expected);
            }
        };
    }

    case!(
        abc_title_me,
        Case::Title,
        Locale::EN,
        StyleGuide::LanguageDefault,
        "a b c",
        "A B C"
    );

    case!(
        abc_lower_me,
        Case::Lower,
        Locale::EN,
        StyleGuide::LanguageDefault,
        "A B C",
        "a b c"
    );

    macro_rules! titlecase {
        ($name:ident, $locale:expr, $style:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_titlecase($input, $locale, $style);
                assert_eq!(actual, $expected);
            }
        };
    }

    titlecase!(
        abc_none,
        Locale::EN,
        StyleGuide::LanguageDefault,
        "a b c",
        "A B C"
    );

    titlecase!(
        abc_cmos,
        Locale::EN,
        StyleGuide::ChicagoManualOfStyle,
        "a b c",
        "A B C"
    );

    titlecase!(
        abc_gruber,
        Locale::EN,
        StyleGuide::DaringFireball,
        "a b c",
        "A B C"
    );

    titlecase!(
        simple_cmos,
        Locale::EN,
        StyleGuide::ChicagoManualOfStyle,
        "Once UPON A time",
        "Once upon a Time"
    );

    titlecase!(
        simple_gruber,
        Locale::EN,
        StyleGuide::DaringFireball,
        "Once UPON A time",
        "Once UPON a Time"
    );

    titlecase!(
        colon_cmos,
        Locale::EN,
        StyleGuide::ChicagoManualOfStyle,
        "foo: a baz",
        "Foo: a Baz"
    );

    titlecase!(
        colon_gruber,
        Locale::EN,
        StyleGuide::DaringFireball,
        "foo: a baz",
        "Foo: A Baz"
    );

    // titlecase!(
    //     qna_cmos,
    //     Locale::EN,
    //     StyleGuide::ChicagoManualOfStyle,
    //     "Q&A with Steve Jobs: 'That's what happens in technology'",
    //     "Q&a with Steve Jobs: 'that's What Happens in Technology'"
    // );

    titlecase!(
        qna_gruber,
        Locale::EN,
        StyleGuide::DaringFireball,
        "Q&A with Steve Jobs: 'That's what happens in technology'",
        "Q&A With Steve Jobs: 'That's What Happens in Technology'"
    );

    titlecase!(
        ws_gruber,
        Locale::EN,
        StyleGuide::DaringFireball,
        "  free  trolling\n  space  ",
        "  Free  Trolling\n  Space  "
    );

    titlecase!(
        turkish_question,
        Locale::TR,
        StyleGuide::LanguageDefault,
        "aç mısın",
        "Aç mısın"
    );

    titlecase!(
        turkish_question_false,
        Locale::TR,
        StyleGuide::LanguageDefault,
        "dualarımızda minnettarlık",
        "Dualarımızda Minnettarlık"
    );

    titlecase!(
        turkish_chars,
        Locale::TR,
        StyleGuide::LanguageDefault,
        "İLKİ ILIK ÖĞLEN",
        "İlki Ilık Öğlen"
    );

    titlecase!(
        turkish_blockwords,
        Locale::TR,
        StyleGuide::LanguageDefault,
        "Sen VE ben ile o",
        "Sen ve Ben ile O"
    );

    titlecase!(
        turkish_ws,
        Locale::TR,
        StyleGuide::LanguageDefault,
        "  serbest  serseri\n  boşluk  ",
        "  Serbest  Serseri\n  Boşluk  "
    );

    macro_rules! lowercase {
        ($name:ident, $locale:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_lowercase($input, $locale);
                assert_eq!(actual, $expected);
            }
        };
    }

    lowercase!(lower_en, Locale::EN, "foo BAR BaZ BIKE", "foo bar baz bike");

    lowercase!(
        lower_tr,
        Locale::TR,
        "foo BAR BaZ ILIK İLE",
        "foo bar baz ılık ile"
    );

    macro_rules! uppercase {
        ($name:ident, $locale:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_uppercase($input, $locale);
                assert_eq!(actual, $expected);
            }
        };
    }

    uppercase!(upper_en, Locale::EN, "foo BAR BaZ bike", "FOO BAR BAZ BIKE");

    uppercase!(
        upper_tr,
        Locale::TR,
        "foo BAR BaZ ILIK İLE",
        "FOO BAR BAZ ILIK İLE"
    );

    macro_rules! sentencecase {
        ($name:ident, $locale:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_sentencecase($input, $locale);
                assert_eq!(actual, $expected);
            }
        };
    }

    sentencecase!(
        sentence_en,
        Locale::EN,
        "insert BIKE here",
        "Insert bike here"
    );

    sentencecase!(sentence_tr, Locale::TR, "ilk DAVRANSIN", "İlk davransın");
}
