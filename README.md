# raca_std

This is the rust std for racaOS.

!!!Because the API of racaOS is unstable, so you must choose the right version of raca_std to use!!!

### Example

Add raca_std to dependencies

```rust {"id":"01J9AKJN5H2CMKD8P1ZMYF3M0S"}
// main.rs

#![no_std]
#![no_main]
#![feature(naked_functions)]

#[no_mangle]
pub fn main() -> usize {
    0
}

```

Then build it with `cargo build`, copy the executable file to the racaOS root directory.   
So you can run it in racaOS.
