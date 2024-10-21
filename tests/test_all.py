# SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
# SPDX-License-Identifier: LGPL-3.0-only

from decasify import (
    case,
    titlecase,
    lowercase,
    uppercase,
    sentencecase,
    Case,
    Locale,
    StyleGuide,
)


def test_isfuction():
    assert callable(case)
    assert callable(titlecase)
    assert callable(lowercase)
    assert callable(uppercase)
    assert callable(sentencecase)


class TestCase:
    def test_optional_arguments(self):
        assert case("foo", Case.Title, Locale.EN) == "Foo"
        assert case("foo", Case.Title, Locale.EN, StyleGuide.DaringFireball) == "Foo"
        assert case("foo", Case.Upper, Locale.EN) == "FOO"

class TestTitlecase:
    def test_optional_arguments(self):
        assert titlecase("foo", Locale.EN) == "Foo"

    def test_english_style_guides(self):
        text = "foo: a baz"
        cmos = "Foo: a Baz"
        grub = "Foo: A Baz"
        assert titlecase(text, Locale.EN, StyleGuide.ChicagoManualOfStyle) == cmos
        assert titlecase(text, Locale.EN, StyleGuide.DaringFireball) == grub

    def test_turkish_characters(self):
        text = "İLKİ ILIK ÖĞLEN"
        outp = "İlki Ilık Öğlen"
        assert titlecase(text, Locale.TR) == outp

    def test_turkish_words(self):
        text = "Sen VE ben ile o"
        outp = "Sen ve Ben ile O"
        assert titlecase(text, Locale.TR) == outp


class TestLowercase:
    def test_english_defaults(self):
        text = "IBUPROFIN"
        outp = "ibuprofin"
        assert lowercase(text, Locale.EN) == outp

    def test_turkish_characters(self):
        text = "İLKİ ILIK ÖĞLEN"
        outp = "ilki ılık öğlen"
        assert lowercase(text, Locale.TR) == outp


class TestUppercase:
    def test_english_defaults(self):
        text = "ibuprofin"
        outp = "IBUPROFIN"
        assert uppercase(text, Locale.EN) == outp

    def test_turkish_characters(self):
        text = "ilki ılık öğlen"
        outp = "İLKİ ILIK ÖĞLEN"
        assert uppercase(text, Locale.TR) == outp


class TestSentencecase:
    def test_english_defaults(self):
        text = "insert BIKE here"
        outp = "Insert bike here"
        assert sentencecase(text, Locale.EN) == outp

    def test_turkish_characters(self):
        text = "ilk DAVRANSIN"
        outp = "İlk davransın"
        assert sentencecase(text, Locale.TR) == outp
