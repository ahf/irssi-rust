// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use std::ffi::CString;

extern "C" {
    fn module_register_full(
        name: *const libc::c_char,
        submodule: *const libc::c_char,
        defined_module_name: *const libc::c_char,
    );
}

pub fn register(name: &str, submodule: &str) {
    let name_c = CString::new(name).unwrap();
    let submodule_c = CString::new(submodule).unwrap();
    let defined_module_name_c = CString::new(format!("{}/{}", name, submodule)).unwrap();

    unsafe {
        module_register_full(
            name_c.as_ptr(),
            submodule_c.as_ptr(),
            defined_module_name_c.as_ptr(),
        );
    }
}
