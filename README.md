# a test repo - nothing to see here

This repo tries to reproduce the unique allocs for the rust
issue - https://github.com/rust-lang/rust/issues/57349

## testing

run `main` in `crate1`, which imports/uses `FOO` and `X` from `crate1`
in an attempt to see if the pointers are different. from where I 
looked, they are.


FOO pointer is: 0x559d00b0b05c
BAR pointer is: 0x559d00b0b05c
X pointer is: 0x559d00b0b060
Y pointer is: 0x559d00b0b060