use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

/// Shorthand for 3x + 1 operation for BigUint.
///
/// TODO: make this a macro
pub fn three_x_plus_one(n: BigUint) -> BigUint {
    BigUint::from_bytes_le(&[3]) * n + BigUint::one()
}

/// Maps a given list of numbers to list of biguints.
pub fn to_biguints(nums: Vec<u32>) -> Vec<BigUint> {
    nums.iter().map(|x| x.to_biguint().unwrap()).collect()
}
