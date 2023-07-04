use num_bigint::BigUint;
use num_traits::{One, Zero};

/// Finds the path from a number.
///
/// 1. Decrement
/// 2. Convert to binary array
/// 3. Reverse array
/// 4. Flip bits
#[inline]
pub fn to_path(n: &BigUint) -> Vec<bool> {
    to_binary(&(n.clone() - BigUint::one()))
        .iter()
        .rev()
        .map(|b| !b)
        .collect()
}

/// Finds the number from a path.
///
/// 1. Flip bits
/// 2. Reverse array
/// 3. Convert to decimal
/// 4. Increment
#[inline]
pub fn from_path(p: &[bool]) -> BigUint {
    from_binary(&p.iter().map(|b| !b).rev().collect::<Vec<bool>>()[..]) + BigUint::one()
}

/// Given a binary representation in bools, compute the corresponding number.
pub fn from_binary(b: &[bool]) -> BigUint {
    BigUint::from_radix_be(
        b.iter()
            .map(|b| if *b { 1 } else { 0 })
            .collect::<Vec<u8>>()
            .as_slice(),
        2,
    )
    .unwrap()
}

/// Convert a number to binary format using bools.
/// The resulting array has the minimum number of bits needed
/// to represent the number. In doing so, `ntob(0)` results in
/// an empty array.
///
/// - `0` corresponds to `false`
/// - `1` corresponds to `true`
pub fn to_binary(n: &BigUint) -> Vec<bool> {
    if *n == BigUint::zero() {
        vec![]
    } else {
        n.to_radix_be(2).into_iter().map(|b| b == 1).collect()
    }
}

/// Returns `true` if the number is a power of two.
pub fn is_pow2(n: &BigUint) -> bool {
    if *n == BigUint::zero() {
        true
    } else {
        n & (n - BigUint::one()) == BigUint::zero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::ToBigUint;

    #[test]
    fn test_path() {
        struct Case {
            n: BigUint,
            p: Vec<bool>,
        }
        let cases = vec![
            // edge
            Case {
                n: BigUint::one(),
                p: vec![],
            },
            Case {
                n: 2.to_biguint().unwrap(),
                p: vec![false],
            },
        ];
        for case in cases {
            assert_eq!(to_path(&case.n), case.p, "Wrong path from number.");
            assert_eq!(from_path(&case.p), case.n, "Wrong number from path.");
        }
    }

    #[test]
    fn test_binary() {
        struct Case {
            n: BigUint,
            b: Vec<bool>,
        }
        let cases = vec![
            // edge
            Case {
                n: BigUint::one(),
                b: vec![true],
            },
            Case {
                n: 3.to_biguint().unwrap(),
                b: vec![true, true],
            },
            Case {
                n: 8.to_biguint().unwrap(),
                b: vec![true, false, false, false],
            },
            Case {
                n: 15.to_biguint().unwrap(),
                b: vec![true, true, true, true],
            },
        ];
        for case in cases {
            assert_eq!(to_binary(&case.n), case.b, "Wrong binary from number.");
            assert_eq!(from_binary(&case.b), case.n, "Wrong number from binary.");
        }
    }

    #[test]
    fn test_pow2() {
        struct Case {
            yes: BigUint,
            no: BigUint,
        }
        let cases = vec![
            // edge
            Case {
                yes: BigUint::one(),
                no: 3.to_biguint().unwrap(),
            },
            Case {
                yes: 2.to_biguint().unwrap(),
                no: 5.to_biguint().unwrap(),
            },
            Case {
                yes: 4.to_biguint().unwrap(),
                no: 7.to_biguint().unwrap(),
            },
            Case {
                yes: 16.to_biguint().unwrap(),
                no: 19.to_biguint().unwrap(),
            },
        ];
        for case in cases {
            assert!(is_pow2(&case.yes));
            assert!(!is_pow2(&case.no));
        }
    }
}
