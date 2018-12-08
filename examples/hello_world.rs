// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate irssi;

#[no_mangle]
pub unsafe fn hello_world_abicheck(version: *mut libc::c_int) {
    *version = irssi::abi::VERSION;
}

#[no_mangle]
pub fn hello_world_init() {
    irssi::module::register("hello_world", "core");
    irssi::print("Hello world from Rust!");
}

#[no_mangle]
pub fn hello_world_deinit() {
    irssi::print("Goodbye from Rust!");
}
