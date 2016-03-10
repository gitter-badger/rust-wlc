#!/usr/bin/env python3

#This is a script to verify that Rust code properly represents the
#C functions that it represents

import sys
import re

class Enumeration(object):
    def __init__(self, names):
        for number, name in enumerate(names.split()):
            setattr(self, name, number)

RED_TEXT = '\033[91m'
GREEN_TEXT = '\033[92m'
YELLOW_TEXT = '\033[93m'
END_TEXT = '\033[0m'

RUST_TYPES = ("char bool "
        "i8 i16 i32 i16 i64 u8 u16 u32 u16 u64 isize usize f32 f64 "
        "Array Slice Str Tuple Function")

# Types defined in Rust's libc crate, might want to get this dynamically
C_TYPES = (" __fsword_t blkcnt64_t blkcnt_t blksize_t c_char c_double c_float "
           "c_int c_long c_longlong c_schar c_short c_uchar c_uint c_ulong "
           "c_ulonglong c_ushort cc_t clock_t dev_t fsblkcnt_t fsfilcnt_t "
           "gid_t in_addr_t in_port_t ino64_t ino_t int16_t int32_t int64_t "
           "int8_t intmax_t intptr_t key_t loff_t mode_t mqd_t nfds_t nlink_t "
           "off64_t off_t pid_t pthread_key_t pthread_t ptrdiff_t rlim64_t "
           "rlim_t sa_family_t shmatt_t sighandler_t size_t socklen_t speed_t "
           "ssize_t suseconds_t tcflag_t time_t uid_t uint16_t uint32_t "
           "uint64_t uint8_t uintmax_t uintptr_t useconds_t wchar_t")

RustTypes = Enumeration(RUST_TYPES)
CTypes = Enumeration(C_TYPES)

def log(string, log_type):
    log_type = log_type.lower().strip()
    if log_type == "success":
        color = GREEN_TEXT + "Success: "
    elif log_type == "error":
        color = RED_TEXT + "Error: "
    elif log_type == "warning":
        color = YELLOW_TEXT + "Warning: "
    else:
        raise ValueError("Bad log type {}".format(log_type))
    sys.stderr.write(color + string + END_TEXT +  "\n")


class RustFunction:
    def __init__(self, name: str, args: list, output: Enumeration):
        self.name = name
        self.args = args
        self.output = output


def delete_comments(lines: list) -> list:
    """Deletes all the stuff that appears in comments.
    If the comment is multiline, the lines are just not included"""
    result = []
    in_comment = False
    for line in lines:
        if in_comment:
            end = line.find("*/")
            if end == -1:
                continue
            result.append(line[end + len("*/"):])
            in_comment = False
            continue
        comment_start = line.find("//")
        if comment_start == -1:
            result.append(line)
        else:
            result.append(line[comment_start + len("//"):])
    return result

def get_rust_functions(rust_file: str) -> list:
    """Returns a list of Rust functions found in the rust file.
    If not found FileNotFoundError is thrown"""
    with open(rust_file, "r") as f:
        lines = f.readlines()
    lines = delete_comments(lines)
    functions = []
    for index in range(len(lines)):
        if " fn " not in lines[index] or not lines[index].strip():
            continue
        start_index = end_index = index
        # Find the semi colon that is where the function declaration ends
        semicolon = lines[index].rfind(";")
        is_def = False
        # Might span multiple lines, keep searching until you find it
        while semicolon == -1:
            if lines[index].rfind("{") != -1 or not lines[index].strip():
                is_def = True
                break
            index += 1
            semicolon = lines[index].rfind(";")
            end_index = index
        if is_def: continue
        func = extract_function("".join(lines[start_index: end_index + 1]))
        functions.append(func)
    return functions

def extract_function(line: str) -> RustFunction:
    """Gets the Rust function from the line containing
    the function declaration. Returns it as a RustFunction type."""
    # Get the input arguments first
    fn_start = re.search("fn.*\(", line)
    if fn_start is None:
        raise BadFunctionException("Function does not start with 'fn'")
    name = line[fn_start.start() + len("fn"): fn_start.end() - 1].strip()
    start = fn_start.end()
    end = re.search("\).*;", line).start()
    args = line[start:end]
    # Get types, that is all we care about
    arg_types = []
    for arg in args.split(","):
        if not arg.strip():
            continue
        # This also avoids the problem of the lib being in front
        # e.g: libc::uint32_t
        start = arg.rfind(":")
        if start == -1:
            msg = "Input paramaters malformed: {}".format(arg)
            raise BadFunctionException(msg)
        var_type = arg[start + 1:].replace("*mut", "").replace("*const", "").strip()
        arg_types.append(get_any_type(var_type))
    # Get the output type, if any
    arrow = re.search("\)\s*->", line)
    if not arrow:
        return_type = None
    else:
        start = arrow.end()
        end = re.search("\).*;", line).end() - 1
        return_string = (line[start:end]
                .replace("libc::", "")
                .replace("*const", "")
                .replace("*mut", "")
                .strip())
        return_type = get_any_type(return_string)
    log("Extracted function {}".format(line.strip()), "success")
    return RustFunction(name, arg_types, return_type)

def get_any_type(var_type: str) -> str:
    """Returns either the type the str represents. C types are checked
    first, since they are more likely to be correct. If Rust type is
    found then a warning is logged, but the type is returned regardless"""
    # Check if a C Type
    try:
        CTypes.__getattribute__(var_type)
    except AttributeError:
        # Try again with Rust Types
        try:
            if var_type == "()":
                log("Using explicit -> (), can be removed", "warning")
                return None
            RustTypes.__getattribute__(var_type)
            log("Using Rust type `{}` instead of a libc type".format(var_type),
                    "warning")
        except AttributeError:
            msg = "{} is not a proper Rust or libc type".format(var_type)
            raise BadFunctionException(msg)
    return var_type


class BadFunctionException(Exception):
    pass



if __name__ == "__main__":
    RUST_TYPES = RUST_TYPES + " Size Geometry ViewType ViewState"
    RustTypes = Enumeration(RUST_TYPES)
    line = "fn output_set_sleep(output: uintptr_t, sleep: bool);"
    function = extract_function(line)

    functions = get_rust_functions("handle.rs")

