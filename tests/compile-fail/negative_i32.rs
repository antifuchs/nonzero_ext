#[macro_use]
extern crate nonzero_ext;

use std::num::NonZeroU32;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let _a: NonZeroU32 = nonzero!(-2i32); //~ ERROR no method named
    //                   ^ should not be able to convert
}
