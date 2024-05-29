-- The busted tests here are a bit sparse, but we're really just testing that the Lua/Rust interface is working. The
-- unit tests on the Rust side for various language and casing combinations are much more complete. The important thing
-- to test here is anything that might need special care on the Lua side.
local decasify = require("decasify")

describe("decasify", function ()
   local titlecase = decasify.titlecase
   local lowercase = decasify.lowercase
   local uppercase = decasify.uppercase

   it("should provide the titlecase function", function ()
      assert.is_function(titlecase)
   end)

   describe("titlecase", function ()
      it("should not balk at nil values for optional args", function ()
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
         assert.equals(cmos, titlecase(text, "en", "cmos"))
         assert.equals(grub, titlecase(text, "en", "gruber"))
      end)

      it("should be at peace with Turkish characters", function ()
         local result = titlecase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equals("İlki Ilık Öğlen", result)
      end)

      it("should be nice about Turish words", function ()
         local result = titlecase("Sen VE ben ile o", "tr")
         assert.equals("Sen ve Ben ile O", result)
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
         assert.equals("ibuprofin", result)
      end)

      it("should be at peace with Turkish characters", function ()
         local result = lowercase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equals("ilki ılık öğlen", result)
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
         local result = uppercase("IBUPROFIN")
         assert.equals("IBUPROFIN", result)
      end)

      it("should be at peace with Turkish characters", function ()
         local result = uppercase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equals("İLKİ ILIK ÖĞLEN", result)
      end)
   end)
end)
