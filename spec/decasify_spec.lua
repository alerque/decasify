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

   end)

end)
