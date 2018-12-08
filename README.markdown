# Irssi Rust Interface  [![Build Status](https://travis-ci.org/ahf/irssi-rust.svg?branch=master)](https://travis-ci.org/ahf/irssi-rust)

**A Rust crate for writing modules for the Irssi IRC client.**

## Compiling the example module

To download and build the example `hello world` module:

    $ git clone https://github.com/ahf/irssi-rust.git
    $ cd irssi-rust
    $ cargo build --example hello_world

Copy the generated `libhello_world.so` to `~/.irssi/modules/`:

    $ cp ./target/debug/examples/libhello_world.so ~/.irssi/modules/

You should now be able to start your Irssi and run `/LOAD hello_world`.

## Authors

- Alexander Færøy (<ahf@irssi.org>).
