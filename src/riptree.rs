use num_bigint::BigUint;
use num_traits::One;

use crate::{
    prefix,
    utils::{is_pow2, pton},
};

/// Finds the next number that resides at the path of `n`.
///
/// The path is also given, as `n` can be in different paths (see path extension).
#[inline]
pub fn next_in_path(n: BigUint, p: Vec<bool>) -> BigUint {
    n + (BigUint::one() << p.len())
}

// Finds the prefix of a number, or a number at the given path.
//
// If you only care about the number, simply pass NTOP(n) as the path.
pub fn prefix_find(mut n: BigUint, p: Vec<bool>) -> Vec<u32> {
    assert_eq!(pton(p.clone()), n, "Number must be at this path.");

    if is_pow2(n.clone()) {
        let mut ans = 0;
        while n > BigUint::one() {
            n >>= 1;
            ans += 1;
        }
        return vec![ans];
    } else {
        prefix::find(n.clone(), next_in_path(n, p))
    }
}

mod tests {
    use super::*;
    use crate::utils::ntop;
    use num_bigint::ToBigUint;

    #[test]
    fn test_riptree() {
        struct Case {
            n: BigUint,
            pf: Vec<u32>,
        }
        let cases = vec![
            // edge
            Case {
                n: BigUint::one(),
                pf: vec![0],
            },
            // power of two
            Case {
                n: 2.to_biguint().unwrap(),
                pf: vec![1],
            },
            Case {
                n: 8.to_biguint().unwrap(),
                pf: vec![3],
            },
            // others
            Case {
                n: 5.to_biguint().unwrap(),
                pf: vec![0],
            },
            Case {
                n: 3.to_biguint().unwrap(),
                pf: vec![0, 1],
            },
            Case {
                n: 7.to_biguint().unwrap(),
                pf: vec![0, 1, 2],
            },
            Case {
                n: 27.to_biguint().unwrap(),
                pf: vec![0, 1, 3, 4],
            },
            Case {
                n: 321.to_biguint().unwrap(),
                pf: vec![0, 2, 4],
            },
            Case {
                n: 322.to_biguint().unwrap(),
                pf: vec![1, 3, 5, 6, 8],
            },
        ];
        for case in cases {
            let pf = prefix_find(case.n.clone(), ntop(case.n.clone()));

            assert_eq!(pf, case.pf, "Wrong prefix.");
            assert!(
                prefix::iterate(case.n, pf).bit(0),
                "Result of prefix iteration should be odd..",
            );
        }
    }

    #[test]
    fn test_next_in_path() {
        struct Case {
            n: BigUint,
            k: BigUint,
        }
        let cases = vec![
            Case {
                n: BigUint::one(),
                k: 2.to_biguint().unwrap(),
            },
            Case {
                n: 2.to_biguint().unwrap(),
                k: 4.to_biguint().unwrap(),
            },
            Case {
                n: 3.to_biguint().unwrap(),
                k: 7.to_biguint().unwrap(),
            },
            Case {
                n: 7.to_biguint().unwrap(),
                k: 15.to_biguint().unwrap(),
            },
            Case {
                n: 5.to_biguint().unwrap(),
                k: 13.to_biguint().unwrap(),
            },
        ];
        for case in cases {
            assert_eq!(
                next_in_path(case.n.clone(), ntop(case.n)),
                case.k,
                "Wrong number at next in path."
            );
        }
    }
}
