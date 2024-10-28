// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use decasify::*;

#[test]
fn cast_from_str() {
    let res = titlecase("FIST", "en", "gruber");
    assert_eq!(res, "Fist");
    let res = titlecase("FIST", "tr", "");
    assert_eq!(res, "Fıst");
    let res = titlecase("FIST", "tr", "default");
    assert_eq!(res, "Fıst");
}

#[test]
fn cast_from_legacy_option() {
    let res = titlecase("FIST", "en", Some(StyleGuide::DaringFireball));
    assert_eq!(res, "Fist");
    let res = titlecase("FIST", "en", None);
    assert_eq!(res, "Fist");
}

#[cfg(feature = "unstable-trait")]
#[test]
fn trait_chery() {
    use decasify::Decasify;
    let s = "WHY THE LONG FACE?";
    assert_eq!(s.to_case("sentence", "en", None), "Why the long face?");
    assert_eq!(
        <str as Decasify>::to_lowercase(s, "en"),
        "why the long face?"
    );
    assert_eq!(
        s.to_owned().to_case("sentence", "en", None),
        "Why the long face?"
    );
}

macro_rules! case {
    ($name:ident, $case:expr, $locale:expr, $style:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = case($input, $case, $locale, $style);
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

case!(
    trivia_en,
    Case::Title,
    Locale::EN,
    StyleGuide::LanguageDefault,
    "  foo  bar  ",
    "  Foo  Bar  "
);

case!(
    trivia_tr,
    Case::Title,
    Locale::TR,
    StyleGuide::LanguageDefault,
    "  foo  bar  ",
    "  Foo  Bar  "
);

macro_rules! titlecase {
    ($name:ident, $locale:expr, $style:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = titlecase($input, $locale, $style);
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
            let actual = lowercase($input, $locale);
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
            let actual = uppercase($input, $locale);
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
            let actual = sentencecase($input, $locale);
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
