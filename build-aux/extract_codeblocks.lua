local output_pattern = assert(os.getenv("OUTPUT_PATTERN"))

local count = 0

local function extract (element)
   count = count + 1
   local output = output_pattern:format(count)
   local f = assert(io.open(output, "w"))
   f:write(element.text)
   f:close()
end

return {
   { CodeBlock = extract },
}
