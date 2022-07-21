# a test repo - nothing to see here

This repo tries to reproduce the unique allocs for the rust
issue - https://github.com/rust-lang/rust/issues/57349

## testing

run `main` in `crate1`, which imports/uses `FOO` and `X` from `crate1`
in an attempt to see if the pointers are different. from where I 
looked, they are.


```
$ cargo run -- crate1
   Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/crate2 crate1`
FOO pointer is: 0x562ecd64905c
BAR pointer is: 0x562ecd64905c
X pointer is: 0x562ecd649060
Y pointer is: 0x562ecd649060

$ cargo run --release -- crate1
   Compiling crate1 v0.1.0 (/home/aj/.cache/cargo-temp/tmp-U0lHj9/crate1)
   Compiling crate2 v0.1.0 (/home/aj/.cache/cargo-temp/tmp-U0lHj9/crate2)
    Finished release [optimized] target(s) in 0.17s
     Running `target/release/crate2 crate1`
FOO pointer is: 0x55d875a8b000
BAR pointer is: 0x55d875a8b000
X pointer is: 0x55d875a8b004
Y pointer is: 0x55d875a8b004
```