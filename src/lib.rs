use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

trait CollatzIteration {
    fn three_x_plus_one(&mut self);
}

impl CollatzIteration for BigUint {
    /// Shorthand for `n = 3*n + 1` in `BigUint`s.
    #[inline]
    fn three_x_plus_one(&mut self) {
        *self = 3.to_biguint().unwrap() * self.clone() + BigUint::one();
    }
}

pub mod collatz;
pub mod iterative;
pub mod piptree;
pub mod prefix;
pub mod riptree;
pub mod utils;
