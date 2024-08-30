# raca_std

This is the rust std for racaOS.

!!!Because the API of racaOS is unstable, so you must choose the right version of raca_std to use!!!

### Example
Add raca_std to dependencies

``` rust
#![no_std]
#![no_main]
#![feature(naked_functions)]

#[no_mangle]
pub fn main() -> usize {
    0
}

```
