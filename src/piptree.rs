use num_bigint::BigUint;
use num_traits::One;

use crate::{
    prefix,
    utils::{from_binary, from_path, is_pow2},
};

/// Finds the nature of a path.
///
/// - `true`: result is even, refers to GOOD nature
/// - `false`: result is odd, refers to BAD nature
#[inline]
pub fn find_nature(p: &Vec<bool>, pf: &Vec<u32>, rpf: u32) -> bool {
    // check if the result of prefix iteration is even or odd
    !(prefix::iterate(from_path(p), &[pf.to_vec(), vec![rpf + 1]].concat()).bit(0))
}

/// Finds the path from root to the node indexed by p in PIPTree, with the path length of the root node being equal to |p|.
///
/// It starts from the target, and in a loop either does `m/2` or `(m-1)/2` until it reaches 1.
/// This gives the path from that number to root, so we reverse that to obtain the road from path to target.
///
/// In the resulting array:
/// - `true`: right
/// - `false`: left
pub fn get_root_directions(p: &Vec<bool>) -> Vec<bool> {
    let mut ans = vec![];

    let mut i = from_binary(p);
    while i > BigUint::one() {
        if i.bit(0) {
            i -= BigUint::one();
            ans.push(true); // right
        } else {
            ans.push(false); // left
        }
        i >>= 1;
    }

    ans.reverse();
    ans
}

/// Finds the prefix of a number using PIPTree properties.
pub fn prefix_find(n: BigUint, p: &Vec<bool>) -> Vec<u32> {
    assert_eq!(from_path(p), n, "Number must be at this path.");

    if is_pow2(&n) {
        let mut nn = n; // TODO: can be removed?
        let mut ans = 0;
        while nn > BigUint::one() {
            nn >>= 1;
            ans += 1;
        }
        vec![ans]
    } else {
        let dirs = get_root_directions(p);

        let root_pf = (p.len() - 1) as u32;
        let root_n = BigUint::one() << root_pf;
        let root_p = [vec![false; p.len() - 1], vec![true]].concat();

        // start from the root and work your way to the target
        let mut cur_pf = vec![root_pf];
        let mut cur_n = root_n.clone();
        let mut cur_p = root_p;

        for dir in dirs {
            // nature of current node
            let nat = find_nature(&cur_p, &cur_pf, root_pf);

            // decrement everything in the prefix
            for pf_i in &mut cur_pf {
                *pf_i -= 1;
            }

            // append root prefix if
            // BAD and RIGHT, or
            // GOOD and LEFT
            if (dir && !nat) || (!dir && nat) {
                cur_pf.push(root_pf);
            }

            // div by 2
            cur_n >>= 1;

            // if GOOD, add root too
            if !dir {
                cur_n += root_n.clone();
            }

            // go to the next child
            cur_p.rotate_left(1);
            let last = cur_p.last_mut().unwrap();
            *last = dir;
        }

        cur_pf
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::to_path;
    use num_bigint::ToBigUint;

    #[test]
    fn test_piptree() {
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
            let pf = prefix_find(case.n.clone(), &to_path(&case.n));

            assert_eq!(pf, case.pf, "Wrong prefix: {}", case.n);
            assert!(
                prefix::iterate(case.n, &pf).bit(0),
                "Result of prefix iteration should be odd..",
            );
        }
    }
}
