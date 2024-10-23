-- SPDX-FileCopyrightText: © 2023 Caleb Maclennan <caleb@alerque.com>
-- SPDX-License-Identifier: LGPL-3.0-only

-- The busted tests here are a bit sparse, but we're really just testing that the Lua/Rust interface is working. The
-- unit tests on the Rust side for various language and casing combinations are much more complete. The important thing
-- to test here is anything that might need special care on the Lua side.
local decasify = require("decasify")

describe("decasify", function ()
   local case = decasify.case
   local titlecase = decasify.titlecase
   local lowercase = decasify.lowercase
   local uppercase = decasify.uppercase
   local sentencecase = decasify.sentencecase

   it("should identify its version", function ()
      local version_file = assert(io.open(".version", "r"))
      local build_env_version = version_file:read("*all")
      version_file:close()
      assert.is.equal("v" .. build_env_version, decasify.version)
   end)

   it("should provide the casing functions", function ()
      assert.is_function(case)
      assert.is_function(titlecase)
      assert.is_function(lowercase)
      assert.is_function(uppercase)
      assert.is_function(sentencecase)
   end)

   describe("case", function ()
      it("should not balk at nil values for optional args", function ()
         assert.no.error(function ()
            case("foo", nil, "en", "cmos")
         end)
         assert.no.error(function ()
            case("foo", nil, "tr")
         end)
         assert.no.error(function ()
            case("foo")
         end)
      end)

      it("should not balk at passing all options through", function ()
         local text = "foo: a baz"
         assert.equal("Foo: A Baz", case(text, "title", "en", "grubber"))
         assert.equal("FOO: A BAZ", case(text, "upper", "en", "gruber"))
      end)
   end)

   describe("titlecase", function ()
      it("should not balk at nil values for optional args", function ()
         assert.no.error(function ()
            titlecase("foo", nil, "cmos")
         end)
         assert.no.error(function ()
            titlecase("foo", "en", "cmos")
         end)
         assert.no.error(function ()
            titlecase("foo", "tr")
         end)
         assert.no.error(function ()
            titlecase("foo")
         end)
      end)

      it("should cooperate with English style guides", function ()
         local text = "foo: a baz"
         local cmos = "Foo: a Baz"
         local grub = "Foo: A Baz"
         assert.equal(cmos, titlecase(text, "en", "cmos"))
         assert.equal(grub, titlecase(text, "en", "gruber"))
      end)

      it("should be at peace with Turkish characters", function ()
         local result = titlecase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equal("İlki Ilık Öğlen", result)
      end)

      it("should be nice about Turish words", function ()
         local result = titlecase("Sen VE ben ile o", "tr")
         assert.equal("Sen ve Ben ile O", result)
      end)
   end)

   describe("lowercase", function ()
      it("should not balk at nil values for optional args", function ()
         assert.no.error(function ()
            lowercase("foo", "en")
         end)
         assert.no.error(function ()
            lowercase("foo")
         end)
      end)

      it("should default to handling string as English", function ()
         local result = lowercase("IBUPROFIN")
         assert.equal("ibuprofin", result)
      end)

      it("should be at peace with Turkish characters", function ()
         local result = lowercase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equal("ilki ılık öğlen", result)
      end)
   end)

   describe("uppercase", function ()
      it("should not balk at nil values for optional args", function ()
         assert.no.error(function ()
            uppercase("foo", "en")
         end)
         assert.no.error(function ()
            uppercase("foo")
         end)
      end)

      it("should default to handling string as English", function ()
         local result = uppercase("ibuprofin")
         assert.equal("IBUPROFIN", result)
      end)

      it("should be at peace with Turkish characters", function ()
         local result = uppercase("ilki ılık öğlen", "tr")
         assert.equal("İLKİ ILIK ÖĞLEN", result)
      end)
   end)

   describe("sentencecase", function ()
      it("should not balk at nil values for optional args", function ()
         assert.no.error(function ()
            sentencecase("foo", "en")
         end)
         assert.no.error(function ()
            sentencecase("foo")
         end)
      end)

      it("should default to handling string as English", function ()
         local result = sentencecase("insert BIKE here")
         assert.equal("Insert bike here", result)
      end)

      it("should be at peace with Turkish characters", function ()
         local result = sentencecase("ilk DAVRANSIN", "tr")
         assert.equal("İlk davransın", result)
      end)
   end)
end)
