// Copyright (c) 2018 Alexander Færøy. All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

extern crate irssi;

irssi::module_abicheck!(hello_world_abicheck);

irssi::module_init!(hello_world_init, {
    irssi::module::register("hello_world", "core");
    irssi::print("Hello world from Rust!");
});

irssi::module_deinit!(hello_world_deinit, {
    irssi::print("Goodbye from Rust!");
});
