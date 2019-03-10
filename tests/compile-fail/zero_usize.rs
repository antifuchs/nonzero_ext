#[macro_use]
extern crate nonzero_ext;

use std::num::NonZeroUsize;

#[cfg_attr(rustfmt, rustfmt_skip)]
fn main() {
    let _a: NonZeroUsize = nonzero!(0usize); //~ ERROR evaluation of constant value failed [E0080]
    //                     ^ should complain about the zero-ness of the argument
}
