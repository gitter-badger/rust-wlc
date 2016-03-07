local define_pattern = "^#define XKB_KEY_([_%a%w]+)%s+(0x%x+)"
local bad_define_pattern = "^#define _"
local comment_pattern = "/%* ([_%w%+%s]+) %*/"
local ifs_line_pattern = "^#[e|i]"
local output_name = "keysyms.rs"

local input_name = "xkbcommon-keysyms.h"

local key_start_index = 54

local output_file = io.open(output_name, "w")
assert(output_file)

output_file:write("#![allow(dead_code)]\n#![allow(missing_docs)]\n#![allow(non_upper_case_globals)]\n\n")

output_file:write("//! Keysyms defined in xkbcommon-keysyms.h\n")
output_file:write("//!\n//! Autogenerated by convert.lua.\n\n")

output_file:write("use super::Keysym;\n")

for line in io.lines(input_name) do
    if not line:match(ifs_line_pattern) then
        local name, value = line:match(define_pattern)
        if name == nil or value == nil then
            if not line:match(bad_define_pattern) then
                output_file:write(line..'\n')
            end -- Don't add the #define _xkb_thing
        else -- Match
            local comment = line:match(comment_pattern)
            if comment ~= nil then
                output_file:write("/// " .. comment .. "\n")
            end
            local first = "pub const KEY_" .. name .. ": Keysym = "
            local space_count = math.abs(key_start_index - first:len())
            output_file:write(first .. string.rep(" ", space_count))
            output_file:write("Keysym(" .. value .. "u32);\n")
        end
    end
end
output_file:close() -- Flush output
print("File converted.")
