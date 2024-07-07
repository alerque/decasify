#![doc = include_str!("../README.md")]

use regex::Regex;
use titlecase::titlecase as gruber_titlecase;
use unicode_titlecase::tr_az::StrTrAzCasing;
use unicode_titlecase::StrTitleCase;

pub mod types;

pub use types::{InputLocale, Result, StyleGuide, TargetCase};

#[cfg(feature = "cli")]
pub mod cli;

#[cfg(feature = "luamodule")]
pub mod lua;

#[cfg(feature = "pythonmodule")]
pub mod python;

/// Convert a string to title case following typestting conventions for a target locale
pub fn to_titlecase(string: &str, locale: InputLocale, style: Option<StyleGuide>) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale {
        InputLocale::EN => to_titlecase_en(words, style),
        InputLocale::TR => to_titlecase_tr(words, style),
    }
}

/// Convert a string to lower case following typestting conventions for a target locale
pub fn to_lowercase(string: &str, locale: InputLocale) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale {
        InputLocale::EN => to_lowercase_en(words),
        InputLocale::TR => to_lowercase_tr(words),
    }
}

/// Convert a string to upper case following typestting conventions for a target locale
pub fn to_uppercase(string: &str, locale: InputLocale) -> String {
    let words: Vec<&str> = string.split_whitespace().collect();
    match locale {
        InputLocale::EN => to_uppercase_en(words),
        InputLocale::TR => to_uppercase_tr(words),
    }
}

fn to_titlecase_en(words: Vec<&str>, style: Option<StyleGuide>) -> String {
    match style {
        Some(StyleGuide::AssociatedPress) => to_titlecase_ap(words),
        Some(StyleGuide::ChicagoManualOfStyle) => to_titlecase_cmos(words),
        Some(StyleGuide::DaringFireball) => to_titlecase_gruber(words),
        None => to_titlecase_gruber(words),
    }
}

fn to_titlecase_ap(words: Vec<&str>) -> String {
    eprintln!("AP style guide not implemented, string returned as-is!");
    words.join(" ")
}

fn to_titlecase_cmos(words: Vec<&str>) -> String {
    let mut words = words.iter().peekable();
    let mut output: Vec<String> = Vec::new();
    let first = words.next().unwrap();
    output.push(first.to_titlecase_lower_rest());
    while let Some(word) = words.next() {
        if words.peek().is_none() {
            output.push(word.to_titlecase_lower_rest());
        } else {
            match is_reserved_en(word.to_string()) {
                true => output.push(word.to_string().to_lowercase()),
                false => {
                    output.push(word.to_titlecase_lower_rest());
                }
            }
        }
    }
    output.join(" ")
}

fn to_titlecase_gruber(words: Vec<&str>) -> String {
    let text = words.join(" ");
    gruber_titlecase(&text)
}

fn to_titlecase_tr(words: Vec<&str>, style: Option<StyleGuide>) -> String {
    match style {
        Some(_) => panic!("Turkish implementation doesn't support different style guides."),
        None => {
            let mut words = words.iter();
            let mut output: Vec<String> = Vec::new();
            let first = words.next().unwrap();
            output.push(first.to_titlecase_tr_or_az_lower_rest());
            for word in words {
                match is_reserved_tr(word.to_string()) {
                    true => output.push(word.to_string().to_lowercase()),
                    false => {
                        output.push(word.to_titlecase_tr_or_az_lower_rest());
                    }
                }
            }
            output.join(" ")
        }
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
    let soruek = Regex::new(r"^([Mm][İiIıUuÜü])").unwrap();
    let word = word.as_str();
    baglac.is_match(word) || soruek.is_match(word)
}

fn to_lowercase_en(words: Vec<&str>) -> String {
    let mut output: Vec<String> = Vec::new();
    for word in words {
        output.push(word.to_lowercase());
    }
    output.join(" ")
}

fn to_lowercase_tr(words: Vec<&str>) -> String {
    let mut output: Vec<String> = Vec::new();
    for word in words {
        output.push(word.to_lowercase_tr_az());
    }
    output.join(" ")
}

fn to_uppercase_en(words: Vec<&str>) -> String {
    let mut output: Vec<String> = Vec::new();
    for word in words {
        output.push(word.to_uppercase());
    }
    output.join(" ")
}

fn to_uppercase_tr(words: Vec<&str>) -> String {
    let mut output: Vec<String> = Vec::new();
    for word in words {
        output.push(word.to_uppercase_tr_az());
    }
    output.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! titlecase {
        ($name:ident, $locale:expr, $style:expr, $input:expr, $expected:expr) => {
            #[test]
            fn $name() {
                let actual = to_titlecase($input, $locale, $style);
                assert_eq!(actual, $expected);
            }
        };
    }

    titlecase!(abc_none, InputLocale::EN, None, "a b c", "A B C");

    titlecase!(
        abc_cmos,
        InputLocale::EN,
        Some(StyleGuide::ChicagoManualOfStyle),
        "a b c",
        "A B C"
    );

    titlecase!(
        abc_gruber,
        InputLocale::EN,
        Some(StyleGuide::DaringFireball),
        "a b c",
        "A B C"
    );

    titlecase!(
        simple_cmos,
        InputLocale::EN,
        Some(StyleGuide::ChicagoManualOfStyle),
        "Once UPON A time",
        "Once upon a Time"
    );

    titlecase!(
        simple_gruber,
        InputLocale::EN,
        Some(StyleGuide::DaringFireball),
        "Once UPON A time",
        "Once UPON a Time"
    );

    titlecase!(
        colon_cmos,
        InputLocale::EN,
        Some(StyleGuide::ChicagoManualOfStyle),
        "foo: a baz",
        "Foo: a Baz"
    );

    titlecase!(
        colon_gruber,
        InputLocale::EN,
        Some(StyleGuide::DaringFireball),
        "foo: a baz",
        "Foo: A Baz"
    );

    // titlecase!(
    //     qna_cmos,
    //     InputLocale::EN,
    //     Some(StyleGuide::ChicagoManualOfStyle),
    //     "Q&A with Steve Jobs: 'That's what happens in technology'",
    //     "Q&a with Steve Jobs: 'that's What Happens in Technology'"
    // );

    titlecase!(
        qna_gruber,
        InputLocale::EN,
        Some(StyleGuide::DaringFireball),
        "Q&A with Steve Jobs: 'That's what happens in technology'",
        "Q&A With Steve Jobs: 'That's What Happens in Technology'"
    );

    titlecase!(
        turkish_chars,
        InputLocale::TR,
        None,
        "İLKİ ILIK ÖĞLEN",
        "İlki Ilık Öğlen"
    );

    titlecase!(
        turkish_blockwords,
        InputLocale::TR,
        None,
        "Sen VE ben ile o",
        "Sen ve Ben ile O"
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

    lowercase!(
        lower_en,
        InputLocale::EN,
        "foo BAR BaZ BIKE",
        "foo bar baz bike"
    );

    lowercase!(
        lower_tr,
        InputLocale::TR,
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

    uppercase!(
        upper_en,
        InputLocale::EN,
        "foo BAR BaZ bike",
        "FOO BAR BAZ BIKE"
    );

    uppercase!(
        upper_tr,
        InputLocale::TR,
        "foo BAR BaZ ILIK İLE",
        "FOO BAR BAZ ILIK İLE"
    );
}
