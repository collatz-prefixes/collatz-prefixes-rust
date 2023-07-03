use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

/// Shorthand for 3x + 1 operation for BigUint.
///
/// TODO: make this a macro?
pub fn three_x_plus_one(n: BigUint) -> BigUint {
    3.to_biguint().unwrap() * n + BigUint::one()
}
