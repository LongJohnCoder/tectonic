#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libc::strlen;

use crate::xetex_errors::overflow;
use crate::xetex_ini::{
    init_pool_ptr, init_str_ptr, max_strings, pool_ptr, pool_size, str_pool, str_ptr, str_start,
    BUFFER,
};

pub(crate) const EMPTY_STRING: i32 = 65536 + 1;

pub(crate) type UnicodeScalar = i32;
pub(crate) type pool_pointer = i32;
pub(crate) type str_number = i32;
pub(crate) type packed_UTF16_code = u16;
/* tectonic/xetex-stringpool.c: preloaded "string pool" constants
   Copyright 2017-2018 the Tectonic Project
   Licensed under the MIT License.
*/
static mut string_constants: [*const i8; 3] = [
    b"this marks the start of the stringpool\x00" as *const u8 as *const i8,
    b"\x00" as *const u8 as *const i8,
    std::ptr::null(),
];
pub(crate) unsafe fn load_pool_strings(mut spare_size: i32) -> i32 {
    let mut s: *const i8 = std::ptr::null();
    let mut i: i32 = 0i32;
    let mut g: str_number = 0i32;
    loop {
        let fresh0 = i;
        i = i + 1;
        s = string_constants[fresh0 as usize];
        if s.is_null() {
            break;
        }
        let mut len = strlen(s);
        let total_len = len;
        if total_len >= spare_size as usize {
            return 0i32;
        }
        loop {
            let fresh1 = len;
            len = len.wrapping_sub(1);
            if !(fresh1 > 0) {
                break;
            }
            let fresh2 = s;
            s = s.offset(1);
            let fresh3 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            str_pool[fresh3 as usize] = *fresh2 as packed_UTF16_code
        }
        g = make_string()
        /* Returns 0 on error. */
    }
    g
}
pub(crate) unsafe fn length(mut s: str_number) -> i32 {
    if s as i64 >= 65536 {
        str_start[((s + 1i32) as i64 - 65536) as usize] - str_start[(s as i64 - 65536) as usize]
    } else if s >= 32i32 && s < 127i32 {
        1
    } else if s <= 127i32 {
        3
    } else if s < 256i32 {
        4
    } else {
        8
    }
}
pub(crate) unsafe fn make_string() -> str_number {
    if str_ptr == max_strings as i32 {
        overflow("number of strings", max_strings - init_str_ptr as usize);
    }
    str_ptr += 1;
    str_start[(str_ptr - 65536) as usize] = pool_ptr;
    str_ptr - 1
}
pub(crate) unsafe fn append_str(mut s: str_number) {
    let mut i: i32 = 0;
    let mut j: pool_pointer = 0;
    i = length(s);
    if pool_ptr + i > pool_size {
        overflow("pool size", (pool_size - init_pool_ptr) as usize);
    }
    j = str_start[(s as i64 - 65536) as usize];
    while i > 0i32 {
        str_pool[pool_ptr as usize] = str_pool[j as usize];
        pool_ptr += 1;
        j += 1;
        i -= 1
    }
}
pub(crate) unsafe fn str_eq_buf(mut s: str_number, mut k: i32) -> bool {
    let mut j: pool_pointer = 0;
    j = str_start[(s as i64 - 65536) as usize];
    while j < str_start[((s + 1i32) as i64 - 65536) as usize] {
        if BUFFER[k as usize] as i64 >= 65536 {
            if str_pool[j as usize] as i64
                != 55296 + (BUFFER[k as usize] as i64 - 65536) / 1024 as i64
            {
                return false;
            } else {
                if str_pool[(j + 1i32) as usize] as i64
                    != 56320 + (BUFFER[k as usize] as i64 - 65536) % 1024 as i64
                {
                    return false;
                } else {
                    j += 1
                }
            }
        } else if str_pool[j as usize] as i32 != BUFFER[k as usize] {
            return false;
        }
        j += 1;
        k += 1
    }
    true
}
pub(crate) unsafe fn str_eq_str(mut s: str_number, mut t: str_number) -> bool {
    let mut j: pool_pointer = 0;
    let mut k: pool_pointer = 0;
    if length(s) != length(t) {
        return false;
    }
    if length(s) == 1i32 {
        if (s as i64) < 65536 {
            if (t as i64) < 65536 {
                if s != t {
                    return false;
                }
            } else if s != str_pool[str_start[(t as i64 - 65536) as usize] as usize] as i32 {
                return false;
            }
        } else if (t as i64) < 65536 {
            if str_pool[str_start[(s as i64 - 65536) as usize] as usize] as i32 != t {
                return false;
            }
        } else if str_pool[str_start[(s as i64 - 65536) as usize] as usize] as i32
            != str_pool[str_start[(t as i64 - 65536) as usize] as usize] as i32
        {
            return false;
        }
    } else {
        j = str_start[(s as i64 - 65536) as usize];
        k = str_start[(t as i64 - 65536) as usize];
        while j < str_start[((s + 1i32) as i64 - 65536) as usize] {
            if str_pool[j as usize] as i32 != str_pool[k as usize] as i32 {
                return false;
            }
            j += 1;
            k += 1
        }
    }
    true
}
pub(crate) unsafe fn search_string(mut search: str_number) -> str_number {
    let mut s: str_number = 0;
    let mut len: i32 = 0;
    len = length(search);
    if len == 0i32 {
        return (65536 + 1i32 as i64) as str_number;
    } else {
        s = search - 1i32;
        while s as i64 > 65535 {
            if length(s) == len {
                if str_eq_str(s, search) {
                    return s;
                }
            }
            s -= 1
        }
    }
    0i32
}
/* tectonic/xetex-stringpool.h: preloaded "string pool" constants
   Copyright 2017 the Tectonic Project
   Licensed under the MIT License.
*/
pub(crate) unsafe fn slow_make_string() -> str_number {
    let mut s: str_number = 0;
    let mut t: str_number = 0;
    t = make_string();
    s = search_string(t);
    if s > 0i32 {
        str_ptr -= 1;
        pool_ptr = str_start[(str_ptr - 65536) as usize];
        return s;
    }
    t
}
