#!/usr/bin/env python3

#This is a script to verify that Rust code properly represents the
#C functions that it represents

# Enumeration hack
class Enumeration(object):
    def __init__(self, names):  # or *names, with no .split
        for number, name in enumerate(names.split()):
            setattr(self, name, number)

RUST_TYPES = ("Char Boolean"
        "i8 i16 i32 i16 i64 u8 u16 u32 u16 u64 isize usize f32 f64"
        "Array Slice Str Tuple Function")

# Types defined in Rust's libc crate
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

class RustFunction:
    def __init__(self, args: list, output: Enumeration):
        self.args = args
        self.output = output

def get_rust_functions(rust_file: str):
    """Returns a list of Rust functions found in the rust file.
    If not found FileNotFoundError is thrown"""
    with open(rust_file, "r") as f:
        lines = f.readlines().splitlines()
    # Gonna have to pass multple line probably
    functions = []
    for index in range(len(lines)):
        if "fn" not in line:
            continue
        start_index = end_index = index
        # Find the semi colon that is where the function declaration ends
        semicolon = lines[index].rfind(";")
        # Might span multiple lines, keep searching until you find it
        while semicolon == -1:
            index += 1
            lines[index].rfind(";")
            end_index = index
        func = extract_function("".join(lines[start_index: end_index + 1]))
        functions.append(func)
    return functions

def extract_function(line: str) -> RustFunction:
    """Gets the rust function from the line contaning 

