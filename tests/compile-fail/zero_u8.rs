#[macro_use] extern crate nonzero_ext;

use std::num::NonZeroU8;

fn main() {
    let _a: NonZeroU8 = nonzero!(0u8); //~ ERROR could not evaluate repeat length
    //                  ^ should complain about the zero-ness of the argument    
}

