#[macro_use] extern crate nonzero_ext;

use std::num::NonZeroUsize;

fn main() {
    let _a: NonZeroUsize = nonzero!(0usize); //~ ERROR could not evaluate repeat length
    //                     ^ should complain about the zero-ness of the argument
}

