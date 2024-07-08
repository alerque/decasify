from decasify import titlecase, lowercase, uppercase, InputLocale, StyleGuide


def test_isfuction():
    assert callable(titlecase)
    assert callable(lowercase)
    assert callable(uppercase)


class TestTitlecase:
    def test_optional_arguments(self):
        assert titlecase("foo", InputLocale.EN) == "Foo"
        assert titlecase("foo", InputLocale.EN) == "Foo"

    def test_english_style_guides(self):
        text = "foo: a baz"
        cmos = "Foo: a Baz"
        grub = "Foo: A Baz"
        assert titlecase(text, InputLocale.EN, StyleGuide.ChicagoManualOfStyle) == cmos
        assert titlecase(text, InputLocale.EN, StyleGuide.DaringFireball) == grub

    def test_turkish_characters(self):
        text = "İLKİ ILIK ÖĞLEN"
        outp = "İlki Ilık Öğlen"
        assert titlecase(text, InputLocale.TR) == outp

    def test_turkish_words(self):
        text = "Sen VE ben ile o"
        outp = "Sen ve Ben ile O"
        assert titlecase(text, InputLocale.TR) == outp


class TestLowercase:
    def test_english_defaults(self):
        text = "IBUPROFIN"
        outp = "ibuprofin"
        assert lowercase(text, InputLocale.EN) == outp

    def test_turkish_characters(self):
        text = "İLKİ ILIK ÖĞLEN"
        outp = "ilki ılık öğlen"
        assert lowercase(text, InputLocale.TR) == outp


class TestUppercase:
    def test_english_defaults(self):
        text = "ibuprofin"
        outp = "IBUPROFIN"
        assert uppercase(text, InputLocale.EN) == outp

    def test_turkish_characters(self):
        text = "ilki ılık öğlen"
        outp = "İLKİ ILIK ÖĞLEN"
        assert uppercase(text, InputLocale.TR) == outp
