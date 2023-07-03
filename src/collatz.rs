use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

/// fs,dndf
pub fn length(mut n: BigUint) -> u32 {
    let mut ans: u32 = 0;
    while !n.eq(&BigUint::one()) {
        ans += 1;
        if n.bit(0) {
            n = 3.to_biguint().unwrap() * n + BigUint::one();
        } else {
            n >>= 1;
        }
    }
    ans + 1
}

#[cfg(test)]
mod tests {
    use crate::collatz;
    use num_bigint::ToBigUint;

    #[test]
    fn collatz_sequences() {
        assert_eq!(collatz::length(5.to_biguint().unwrap()), 6);
    }
}
