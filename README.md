# nonzero_ext

## Traits to represent generic nonzero integer types

Rust ships with non-zero integer types now, which let programmers
promise (memory-savingly!) that a number can never be zero. That's
great, but sadly the standard library has no traits you can use to
represent all the non-zero integer types.

## Examples

Where this lack of traits in the standard library becomes
problematic is if you want to write a function that takes a vector
of integers, and that returns a vector of the corresponding
non-zero integer types, minus any elements that were zero in the
original. You can write that with the standard library quite
easily for concrete types:

``` rust
## use core::num::NonZeroU8;
fn only_nonzeros(v: Vec<u8>) -> Vec<NonZeroU8> {
  let out: Vec<NonZeroU8> = v
       .into_iter()
       .filter_map(|n| NonZeroU8::new(n))
       .collect();
  out
}
let expected: Vec<NonZeroU8> = vec![NonZeroU8::new(20).unwrap(), NonZeroU8::new(5).unwrap()];
assert_eq!(expected, only_nonzeros(vec![0, 20, 5]));
```rust

But what if you want to allow this function to work with any
integer type that has a corresponding non-zero type? This crate
can help:

``` rust
fn only_nonzeros<I>(v: Vec<I>) -> Vec<I::NonZero>
where
  I: Sized + NonZeroAble
{
  let out: Vec<I::NonZero> = v
       .into_iter()
       .filter_map(|n| n.as_nonzero())
       .collect();
  out
}

// It works for `u8`:
let input_u8: Vec<u8> = vec![0, 20, 5];
let expected_u8: Vec<NonZeroU8> = vec![NonZeroU8::new(20).unwrap(), NonZeroU8::new(5).unwrap()];
assert_eq!(expected_u8, only_nonzeros(input_u8));

// And it works for `u32`:
let input_u32: Vec<u32> = vec![0, 20, 5];
let expected_u32: Vec<NonZeroU32> = vec![NonZeroU32::new(20).unwrap(), NonZeroU32::new(5).unwrap()];
assert_eq!(expected_u32, only_nonzeros(input_u32));
```


License: Apache 2.0
