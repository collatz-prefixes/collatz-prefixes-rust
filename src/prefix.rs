use num_bigint::BigUint;
use num_traits::{One, Zero};

use crate::CollatzIteration;

/// Returns the prefix of two numbers.
///
/// The prefix can be thought of as common prefix of the ECFs.
/// As an example, `ECF(3) = [0, 1, 5]` and `ECF(7) = [0, 1, 2, 4, 7, 11]`.
/// The common prefix here is `[0, 1]`, thus `find(3,7) = find(7,3) = [0, 1]`.
pub fn find(mut n: BigUint, mut m: BigUint) -> Vec<u32> {
    let mut ans = vec![];
    let mut twos = 0;

    loop {
        if !n.bit(0) && !m.bit(0) {
            // both are even
            twos += 1;
            n >>= 1;
            m >>= 1;
        } else if n.bit(0) && m.bit(0) {
            // both are odd
            ans.push(twos);
            n.three_x_plus_one();
            m.three_x_plus_one();
        } else {
            break;
        }
    }

    ans
}

/// Iterates a number through a prefix.
///
/// If the prefix is equal to ECF of the number, the result is expected to be 1.
pub fn iterate(mut n: BigUint, pf: &Vec<u32>) -> BigUint {
    if pf.is_empty() {
        n
    } else {
        // R_0 function
        n /= BigUint::one() << pf[0];

        // R function for i = 1..len(pf)
        for i in 1..pf.len() {
            n.three_x_plus_one();
            n /= BigUint::one() << (pf[i] - pf[i - 1]);
        }

        n
    }
}

/// Bijective mapping from a list of ascending numbers to an integer.
pub fn to_num(pf: Vec<u32>) -> BigUint {
    pf.into_iter()
        .fold(BigUint::zero(), |acc, p| acc + (BigUint::one() << p))
}

/// Bijective mapping from a number to a list of ascending numbers.
pub fn from_num(mut k: BigUint) -> Vec<u32> {
    let mut ans = vec![];

    let mut bit_pos = 0;
    while k > BigUint::zero() {
        if k.bit(0) {
            ans.push(bit_pos);
        }
        k >>= 1;
        bit_pos += 1;
    }

    ans
}

/// Add two prefixes. Does not mutate the input, returns a new vector.
///
/// This is done by "attaching" prefixes together.
///
///```md
///f1: [a, b, c]
///pf2:       [x,   y,   z]
///+-------------------------
///sum: [a, b, x+c, y+c, z+c]
///```
pub fn add(pf1: &Vec<u32>, pf2: &Vec<u32>) -> Vec<u32> {
    // edge cases
    if pf1.is_empty() {
        return pf2.to_vec();
    }
    if pf2.is_empty() {
        return pf1.to_vec();
    }

    let last = pf1[pf1.len() - 1];

    let mut ans = pf1.to_owned();
    ans[pf1.len() - 1] += pf2[0];

    for pf2_i in pf2.iter().skip(1) {
        ans.push(pf2_i + last);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collatz;
    use num_bigint::ToBigUint;
    use std::cmp::min;

    /// finds the prefix by simply comparing ECFs.
    fn prefix_brute(a: &Vec<u32>, b: &Vec<u32>) -> Vec<u32> {
        let minlen = min(a.len(), b.len());

        let mut ans = vec![];
        for i in 0..minlen {
            if a[i] == b[i] {
                ans.push(a[i]);
            } else {
                break;
            }
        }

        return ans;
    }

    #[test]
    fn test_prefix() {
        struct Case {
            n: BigUint,
            m: BigUint,
        }
        let cases = vec![
            Case {
                n: BigUint::one(),
                m: 2.to_biguint().unwrap(),
            },
            Case {
                n: 3.to_biguint().unwrap(),
                m: 12.to_biguint().unwrap(),
            },
            Case {
                n: 8.to_biguint().unwrap(),
                m: 16.to_biguint().unwrap(),
            },
            Case {
                n: 27.to_biguint().unwrap(),
                m: 37.to_biguint().unwrap(),
            },
        ];
        for case in cases {
            let ecf_n = collatz::ecf(case.n.clone());
            let ecf_m = collatz::ecf(case.m.clone());
            let pf = prefix_brute(&ecf_n, &ecf_m);

            assert_eq!(find(case.n.clone(), case.m.clone()), pf, "Prefix is wrong.");
            assert_eq!(
                find(case.m.clone(), case.n.clone()),
                pf,
                "Prefix should be commutative."
            );

            // test both numbers for ECF iteration
            assert_eq!(
                iterate(case.n, &ecf_n),
                BigUint::one(),
                "Iterating over ECF should result in 1."
            );
            assert_eq!(
                iterate(case.m, &ecf_m),
                BigUint::one(),
                "Iterating over ECF should result in 1."
            );
        }
    }

    #[test]
    fn test_prefix_add() {
        struct Case {
            pf1: Vec<u32>,
            pf2: Vec<u32>,
            ans: Vec<u32>,
        }
        let cases = vec![
            // edge
            Case {
                pf1: vec![],
                pf2: vec![1],
                ans: vec![1],
            },
            Case {
                pf1: vec![1],
                pf2: vec![],
                ans: vec![1],
            },
            // normal
            Case {
                pf1: vec![2, 4],
                pf2: vec![4],
                ans: vec![2, 8],
            },
            Case {
                pf1: vec![4],
                pf2: vec![2, 4],
                ans: vec![6, 8],
            },
            Case {
                pf1: vec![0, 1, 5],
                pf2: vec![0, 1, 3],
                ans: vec![0, 1, 5, 6, 8],
            },
        ];
        for case in cases {
            assert_eq!(add(&case.pf1, &case.pf2), case.ans, "Wrong result.");
        }
    }

    #[test]
    fn test_prefix_map() {
        struct Case {
            k: BigUint,
            pf: Vec<u32>,
        }
        let cases = vec![
            // edge
            Case {
                k: BigUint::one(),
                pf: vec![0],
            },
            // power of 2
            Case {
                k: 16.to_biguint().unwrap(),
                pf: vec![4],
            },
            // oods
            Case {
                k: 27.to_biguint().unwrap(),
                pf: vec![0, 1, 3, 4],
            },
            Case {
                k: 35.to_biguint().unwrap(),
                pf: vec![0, 1, 5],
            },
            // evens
            Case {
                k: 12.to_biguint().unwrap(),
                pf: vec![2, 3],
            },
            Case {
                k: 190.to_biguint().unwrap(),
                pf: vec![1, 2, 3, 4, 5, 7],
            },
        ];
        for case in cases {
            assert_eq!(to_num(case.pf.clone()), case.k, "Wrong number from prefix.");
            assert_eq!(from_num(case.k), case.pf, "Wrong prefix from number.");
        }
    }
}
