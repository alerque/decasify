// SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
// SPDX-License-Identifier: LGPL-3.0-only

use decasify::*;

#[test]
fn cast_from_str() {
    let res = titlecase("FIST", "en", "gruber", "default").unwrap();
    assert_eq!(res, "Fist");
    let res = titlecase("FIST", "tr", "", "default").unwrap();
    assert_eq!(res, "Fıst");
    let res = titlecase("FIST", "tr", "default", "default").unwrap();
    assert_eq!(res, "Fıst");
}

#[test]
fn cast_from_legacy_option() {
    let res = titlecase(
        "FIST",
        "en",
        Some(StyleGuide::DaringFireball),
        StyleOptions::default(),
    )
    .unwrap();
    assert_eq!(res, "Fist");
    let res = titlecase("FIST", "en", None, StyleOptions::default()).unwrap();
    assert_eq!(res, "Fist");
}

#[test]
fn custom_style_guide() {
    let options: StyleOptions = StyleOptionsBuilder::new().overrides(vec!["fOO"]).build();
    let res = titlecase("foo bar", "tr", StyleGuide::LanguageDefault, options).unwrap();
    assert_eq!(res, "fOO Bar");
}

#[cfg(feature = "unstable-trait")]
#[test]
fn trait_chery() {
    use Decasify::Decasify;
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
    ($name:ident, $case:expr, $locale:expr, $style:expr, $opts:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = case($input, $case, $locale, $style, $opts).unwrap();
            assert_eq!(actual, $expected);
        }
    };
}

case!(
    abc_title_me,
    Case::Title,
    Locale::EN,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "a b c",
    "A B C"
);

case!(
    abc_lower_me,
    Case::Lower,
    Locale::EN,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "A B C",
    "a b c"
);

case!(
    trivia_en,
    Case::Title,
    Locale::EN,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "  foo  bar  ",
    "  Foo  Bar  "
);

case!(
    trivia_es,
    Case::Title,
    Locale::ES,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "  foo  bar  ",
    "  Foo  Bar  "
);

case!(
    trivia_tr,
    Case::Title,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "  foo  bar  ",
    "  Foo  Bar  "
);

macro_rules! titlecase {
    ($name:ident, $locale:expr, $style:expr, $opts:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = titlecase($input, $locale, $style, $opts).unwrap();
            assert_eq!(actual, $expected);
        }
    };
}

titlecase!(
    abc_none,
    Locale::EN,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "a b c",
    "A B C"
);

titlecase!(
    abc_cmos,
    Locale::EN,
    StyleGuide::ChicagoManualOfStyle,
    StyleOptions::default(),
    "a b c",
    "A B C"
);

titlecase!(
    abc_gruber,
    Locale::EN,
    StyleGuide::DaringFireball,
    StyleOptions::default(),
    "a b c",
    "A B C"
);

titlecase!(
    simple_cmos,
    Locale::EN,
    StyleGuide::ChicagoManualOfStyle,
    StyleOptions::default(),
    "Once UPON A time",
    "Once upon a Time"
);

titlecase!(
    simple_gruber,
    Locale::EN,
    StyleGuide::DaringFireball,
    StyleOptions::default(),
    "Once UPON A time",
    "Once UPON a Time"
);

titlecase!(
    colon_cmos,
    Locale::EN,
    StyleGuide::ChicagoManualOfStyle,
    StyleOptions::default(),
    "foo: a baz",
    "Foo: a Baz"
);

titlecase!(
    colon_gruber,
    Locale::EN,
    StyleGuide::DaringFireball,
    StyleOptions::default(),
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
    StyleOptions::default(),
    "Q&A with Steve Jobs: 'That's what happens in technology'",
    "Q&A With Steve Jobs: 'That's What Happens in Technology'"
);

titlecase!(
    ws_gruber,
    Locale::EN,
    StyleGuide::DaringFireball,
    StyleOptions::default(),
    "  free  trolling\n  space  ",
    "  Free  Trolling\n  Space  "
);

titlecase!(
    rae_articles,
    Locale::ES,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "el libro del autor",
    "El Libro del Autor"
);

titlecase!(
    rae_holiday,
    Locale::ES,
    StyleGuide::RealAcademiaEspanola,
    StyleOptions::default(),
    "DÍA DE los muertos",
    "Día de los Muertos"
);

titlecase!(
    rae_magazine,
    Locale::ES,
    StyleGuide::RealAcademiaEspanola,
    StyleOptions::default(),
    "cien años DE soledad",
    "Cien Años de Soledad"
);

titlecase!(
    rae_prepositions,
    Locale::ES,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "en la casa de mi madre",
    "En la Casa de Mi Madre"
);

titlecase!(
    turkish_question,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "aç mısın",
    "Aç mısın"
);

titlecase!(
    turkish_question_false,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "dualarımızda minnettarlık",
    "Dualarımızda Minnettarlık"
);

titlecase!(
    turkish_chars,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "İLKİ ILIK ÖĞLEN",
    "İlki Ilık Öğlen"
);

titlecase!(
    turkish_blockwords,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "Sen VE ben ile o",
    "Sen ve Ben ile O"
);

titlecase!(
    turkish_ws,
    Locale::TR,
    StyleGuide::LanguageDefault,
    StyleOptions::default(),
    "  serbest  serseri\n  boşluk  ",
    "  Serbest  Serseri\n  Boşluk  "
);

macro_rules! lowercase {
    ($name:ident, $locale:expr, $input:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let actual = lowercase($input, $locale).unwrap();
            assert_eq!(actual, $expected);
        }
    };
}

lowercase!(lower_en, Locale::EN, "foo BAR BaZ BIKE", "foo bar baz bike");

lowercase!(lower_es, Locale::ES, "Hola MUNDO", "hola mundo");

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
            let actual = uppercase($input, $locale).unwrap();
            assert_eq!(actual, $expected);
        }
    };
}

uppercase!(upper_en, Locale::EN, "foo BAR BaZ bike", "FOO BAR BAZ BIKE");

uppercase!(upper_es, Locale::ES, "hola MUNDo", "HOLA MUNDO");

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
            let actual = sentencecase($input, $locale).unwrap();
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

sentencecase!(sentence_es, Locale::ES, "hola MUNDO", "Hola mundo");

sentencecase!(sentence_tr, Locale::TR, "ilk DAVRANSIN", "İlk davransın");
