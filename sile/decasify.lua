-- SPDX-FileCopyrightText: © 2024 Caleb Maclennan <caleb@alerque.com>
-- SPDX-License-Identifier: LGPL-3.0-only

local base = require("packages.base")

local package = pl.class(base)
package._name = "decasify"

local decasify = require("decasify")

function package.decasify (node, _, options)
   if type(node) == "table" then
      return node
   end
   local locale = options.locale or SILE.settings:get("document.language") or nil
   return decasify.case(node, options.case, locale, options.style)
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
