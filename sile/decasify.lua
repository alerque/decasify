local base = require("packages.base")

local package = pl.class(base)
package._name = "decasify"

local decasify = require("decasify")

function package:decasify (input, extraArgs)
   if type(self) ~= "table" or (self.type ~= "class" and self.type ~= "package") then
      input, extraArgs = self, input
   end
   if not extraArgs then
      extraArgs = {}
   end
   if not extraArgs.options then
      extraArgs.options = {}
   end
   local case = extraArgs.options.case or nil
   local locale = extraArgs.options.locale or SILE.settings:get("document.language") or nil
   local style = extraArgs.options.style or nil
   return decasify.case(input, case, locale, style)
end

function package:_init ()
   base._init(self)
   self:loadPackage("inputfilter")
end

function package:registerCommands ()
   self:registerCommand("decasify", function (options, content)
      SILE.process(self.class.packages.inputfilter:transformContent(content, self.decasify, options))
   end, "Typeset the enclosed text using case conversion from decasify")
end

package.documentation = [[
\begin{document}
\use[module=packages.decasify]
The \autodoc:package{decasify} package provides commands for language-aware case conversion of input text.
It is similar to the \autodoc:package{textcase} package, but handles locale aware style guides instead of just raw Unicode character conversions.
\end{document}
]]

return package
