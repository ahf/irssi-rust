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

#[macro_export]
macro_rules! module_abicheck {
    ($name:ident) => {
        #[no_mangle]
        pub unsafe fn $name(version: *mut libc::c_int) {
            *version = irssi::abi::VERSION;
        }
    };
}

#[macro_export]
macro_rules! module_init {
    ($name:ident, $expr:block) => {
        #[no_mangle]
        pub fn $name() {
            $expr
        }
    };
}

#[macro_export]
macro_rules! module_deinit {
    ($name:ident, $expr:block) => {
        #[no_mangle]
        pub fn $name() {
            $expr
        }
    };
}
