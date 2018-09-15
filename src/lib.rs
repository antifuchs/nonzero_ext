use core::num::NonZeroUsize;
use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8};

macro_rules! impl_nonzeroness {
    ($trait_name:ident, $nonzero_type:ty, $wrapped:ty) => {
        impl $trait_name for $nonzero_type {
            type Wrapped = $wrapped;

            #[inline]
            fn new(n: $wrapped) -> Option<Self> {
                Self::new(n)
            }
        }
    };
}

/// A trait identifying a non-zero integral type. It is useful mostly
/// in order to give to genericized helper functions as `impl NonZero`
/// arguments.
pub trait NonZero {
    type Wrapped;

    /// Creates a new non-zero object from an integer that might be
    /// zero.
    fn new(n: Self::Wrapped) -> Option<Self>
    where
        Self: Sized;
}

impl_nonzeroness!(NonZero, NonZeroU8, u8);
impl_nonzeroness!(NonZero, NonZeroU16, u16);
impl_nonzeroness!(NonZero, NonZeroU32, u32);
impl_nonzeroness!(NonZero, NonZeroU64, u64);
impl_nonzeroness!(NonZero, NonZeroU128, u128);
impl_nonzeroness!(NonZero, NonZeroUsize, usize);

/// A trait identifying integral types that have a non-zeroable
/// equivalent.
pub trait NonZeroAble {
    type NonZero: crate::NonZero;

    /// Converts the integer to its non-zero equivalent.
    fn as_nonzero(self) -> Option<Self::NonZero>;
}

macro_rules! impl_nonzeroable {
    ($trait_name:ident, $nonzero_type: ty, $nonzeroable_type:ty) => {
        impl $trait_name for $nonzeroable_type {
            type NonZero = $nonzero_type;

            fn as_nonzero(self) -> Option<$nonzero_type> {
                Self::NonZero::new(self)
            }
        }
    };
}

impl_nonzeroable!(NonZeroAble, NonZeroU8, u8);
impl_nonzeroable!(NonZeroAble, NonZeroU16, u16);
impl_nonzeroable!(NonZeroAble, NonZeroU32, u32);
impl_nonzeroable!(NonZeroAble, NonZeroU64, u64);
impl_nonzeroable!(NonZeroAble, NonZeroU128, u128);
impl_nonzeroable!(NonZeroAble, NonZeroUsize, usize);

#[cfg(test)]
mod tests {
    use crate::{NonZero, NonZeroAble};

    #[test]
    fn as_nonzero() {
        let one: u8 = 1;
        assert_eq!(one.as_nonzero().expect("This ought to convert").get(), 1);
    }
}
