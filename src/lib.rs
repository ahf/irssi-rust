// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate libc;

use std::ffi::CString;

pub mod abi;
pub mod module;

extern "C" {
    fn printtext_string(
        server: *const libc::c_void,
        target: *const libc::c_void,
        level: libc::c_int,
        string: *const libc::c_char,
    );
}

pub fn print(input: &str) {
    let input_c = CString::new(input).unwrap();

    unsafe {
        printtext_string(
            std::ptr::null(),
            std::ptr::null(),
            0x0008_0000,
            input_c.as_ptr(),
        );
    }
}
