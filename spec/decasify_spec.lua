local decasify = require("decasify")

describe("decasify", function()

   local titlecase = decasify.titlecase

   it("should provide the titlecase function", function()
      assert.is_function(titlecase)
   end)

   describe("titlecase", function()

      it("should cooperate with English style guides", function()
         local text = "foo: a baz"
         local cmos = "Foo: a Baz"
         local grub = "Foo: A Baz"
         assert.equals(cmos, titlecase(text, "en", "cmos"))
         assert.equals(grub, titlecase(text, "en", "gruber"))
      end)

      it("should be at peace with Turkish characters", function()
         local result = titlecase("İLKİ ILIK ÖĞLEN", "tr")
         assert.equals("İlki Ilık Öğlen", result)
      end)

      it("should be nice about Turish words", function()
         local result = titlecase("Sen VE ben ile o", "tr")
         assert.equals("Sen ve Ben ile O", result)
      end)

   end)

end)
