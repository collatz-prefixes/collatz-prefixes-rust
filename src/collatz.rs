#[path = "../src/common.rs"]
mod common;
use common::three_x_plus_one;

use num_bigint::{BigUint, ToBigUint};
use num_traits::One;

// trait Three {
//     fn three(&self) -> BigUint;
// }
// impl Three for BigUint {
//     fn three(&self) -> BigUint {
//         3.to_biguint().unwrap()
//     }
// }

/// Collatz length is the number of iterations it takes to reach n to 1.
pub fn length(mut n: BigUint) -> usize {
    let mut ans = 0;

    while n != BigUint::one() {
        ans += 1;
        if n.bit(0) {
            n = three_x_plus_one(n)
        } else {
            n >>= 1;
        }
    }

    return ans + 1;
}

/// Collatz Sequence is the array of numbers seen during iterations until 1 is reached.
pub fn sequence(mut n: BigUint) -> Vec<BigUint> {
    let mut ans = Vec::new();

    while n != BigUint::one() {
        ans.push(n.clone());
        if n.bit(0) {
            n = three_x_plus_one(n)
        } else {
            n >>= 1;
        }
    }
    ans.push(n.clone());

    return ans;
}

/// Reduced Collatz Sequence is the array of odd numbers seen during iterations until 1 is reached.
pub fn reduced_sequence(mut n: BigUint) -> Vec<BigUint> {
    let mut ans = Vec::new();

    // if even, must be added at the start
    if !n.bit(0) {
        ans.push(n.clone())
    }

    while n != BigUint::one() {
        if n.bit(0) {
            ans.push(n.clone());
            n = three_x_plus_one(n);
        } else {
            n >>= 1;
        }
    }
    ans.push(n.clone());

    return ans;
}

/// Find ECF (Exponential Canonical Form) of a number.
pub fn ecf(mut n: BigUint) -> Vec<u32> {
    let mut ans = Vec::new();
    let mut twos = 0;

    while n != BigUint::one() {
        if n.bit(0) {
            ans.push(twos);
            n = three_x_plus_one(n);
        } else {
            twos += 1;
            n >>= 1;
        }
    }
    ans.push(twos);

    return ans;
}

/// Compute a number from it's ECF.
pub fn ecf_to_n(ecf: Vec<u32>) -> BigUint {
    let mut ans = BigUint::one();

    for i in (1..ecf.len()).rev() {
        ans <<= ecf[i] - ecf[i - 1];
        ans = (ans - BigUint::one()) / 3.to_biguint().unwrap();
    }

    return ans << ecf[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Maps a given list of numbers to list of biguints.
    fn to_biguints(nums: Vec<u32>) -> Vec<BigUint> {
        nums.iter().map(|x| x.to_biguint().unwrap()).collect()
    }

    #[test]
    fn test_collatz_sequences() {
        struct Case {
            n: BigUint,
            seq: Vec<BigUint>,
            rseq: Vec<BigUint>,
        }
        let cases = vec![
            // edge
            Case {
                n: BigUint::one(),
                seq: to_biguints(vec![1]),
                rseq: to_biguints(vec![1]),
            },
            // power of two
            Case {
                n: 8.to_biguint().unwrap(),
                seq: to_biguints(vec![8, 4, 2, 1]),
                rseq: to_biguints(vec![8, 1]),
            },
            // odd number
            Case {
                n: 5.to_biguint().unwrap(),
                seq: to_biguints(vec![5, 16, 8, 4, 2, 1]),
                rseq: to_biguints(vec![5, 1]),
            },
            // even number
            Case {
                n: 28.to_biguint().unwrap(),
                seq: to_biguints(vec![
                    28, 14, 7, 22, 11, 34, 17, 52, 26, 13, 40, 20, 10, 5, 16, 8, 4, 2, 1,
                ]),
                rseq: to_biguints(vec![28, 7, 11, 17, 13, 5, 1]),
            },
            // large sequence https://oeis.org/A008884
            Case {
                n: 27.to_biguint().unwrap(),
                seq: to_biguints(vec![
                    27, 82, 41, 124, 62, 31, 94, 47, 142, 71, 214, 107, 322, 161, 484, 242, 121,
                    364, 182, 91, 274, 137, 412, 206, 103, 310, 155, 466, 233, 700, 350, 175, 526,
                    263, 790, 395, 1186, 593, 1780, 890, 445, 1336, 668, 334, 167, 502, 251, 754,
                    377, 1132, 566, 283, 850, 425, 1276, 638, 319, 958, 479, 1438, 719, 2158, 1079,
                    3238, 1619, 4858, 2429, 7288, 3644, 1822, 911, 2734, 1367, 4102, 2051, 6154,
                    3077, 9232, 4616, 2308, 1154, 577, 1732, 866, 433, 1300, 650, 325, 976, 488,
                    244, 122, 61, 184, 92, 46, 23, 70, 35, 106, 53, 160, 80, 40, 20, 10, 5, 16, 8,
                    4, 2, 1,
                ]),
                rseq: to_biguints(vec![
                    27, 41, 31, 47, 71, 107, 161, 121, 91, 137, 103, 155, 233, 175, 263, 395, 593,
                    445, 167, 251, 377, 283, 425, 319, 479, 719, 1079, 1619, 2429, 911, 1367, 2051,
                    3077, 577, 433, 325, 61, 23, 35, 53, 5, 1,
                ]),
            },
        ];
        for case in cases {
            assert_eq!(sequence(case.n.clone()), case.seq, "Wrong sequence.");
            assert_eq!(
                length(case.n.clone()),
                case.seq.len(),
                "Wrong sequence length."
            );
            assert_eq!(
                reduced_sequence(case.n.clone()),
                case.rseq,
                "Wrong reduced sequence."
            );
        }
    }

    #[test]
    fn test_collatz_ecf() {
        struct Case {
            n: BigUint,
            ecf: Vec<u32>,
        }
        let cases = vec![
            // edge
            Case {
                n: BigUint::one(),
                ecf: vec![0],
            },
            // power of two
            Case {
                n: 16.to_biguint().unwrap(),
                ecf: vec![4],
            },
            // odd number
            Case {
                n: 3.to_biguint().unwrap(),
                ecf: vec![0, 1, 5],
            },
            // even number
            Case {
                n: 12.to_biguint().unwrap(),
                ecf: vec![2, 3, 7],
            },
            // large sequence https://oeis.org/A008884
            Case {
                n: 27.to_biguint().unwrap(),
                ecf: vec![
                    0, 1, 3, 4, 5, 6, 7, 9, 11, 12, 14, 15, 16, 18, 19, 20, 21, 23, 26, 27, 28, 30,
                    31, 33, 34, 35, 36, 37, 38, 41, 42, 43, 44, 48, 50, 52, 56, 59, 60, 61, 66, 70,
                ],
            },
        ];
        for case in cases {
            assert_eq!(ecf(case.n.clone()), case.ecf, "Wrong ECF from number.");
            assert_eq!(ecf_to_n(case.ecf), case.n, "Wrong number from ECF.");
        }
    }
}
