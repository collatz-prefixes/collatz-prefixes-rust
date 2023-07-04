use num_bigint::BigUint;
use num_traits::One;

use crate::{
    prefix::{add, iterate},
    utils::to_path,
    CollatzIteration,
};

/// Find the ECF by iteratively extending the path until prefix iteration results in 1.
pub fn path_extension(
    n: BigUint,
    prefix_finder: fn(n: BigUint, p: &Vec<bool>) -> Vec<u32>,
) -> Vec<u32> {
    let mut p = to_path(&n);
    while iterate(n.clone(), &prefix_finder(n.clone(), &p)) != BigUint::one() {
        p.push(true);
    }

    prefix_finder(n, &p)
}

/// Find the ECF by iteratively consuming the prefix until the iteration result is 1.
pub fn prefix(
    mut n: BigUint,
    prefix_finder: fn(n: BigUint, p: &Vec<bool>) -> Vec<u32>,
) -> Vec<u32> {
    let mut ans = vec![];
    loop {
        let pf = prefix_finder(n.clone(), &to_path(&n));
        ans = add(&ans, &pf);
        n = iterate(n, &pf);
        if n == BigUint::one() {
            return ans;
        } else {
            n.three_x_plus_one();
            if ans.is_empty() {
                ans.push(*ans.last().unwrap());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{collatz::ecf, piptree, riptree};
    use num_bigint::ToBigUint;

    #[test]
    fn test_iteratives() {
        struct Case {
            n: BigUint,
        }
        let cases = vec![
            Case { n: BigUint::one() },
            Case {
                n: 5.to_biguint().unwrap(),
            },
            Case {
                n: 8.to_biguint().unwrap(),
            },
            Case {
                n: 27.to_biguint().unwrap(),
            },
            Case {
                n: 38.to_biguint().unwrap(),
            },
            Case {
                n: (186438726873 as i64).to_biguint().unwrap(),
            },
        ];
        for case in cases {
            let case_ecf = ecf(case.n.clone());

            assert_eq!(
                prefix(case.n.clone(), riptree::prefix_find),
                case_ecf,
                "ECF mismatch using Prefix + RIPTree for {}",
                case.n
            );

            assert_eq!(
                prefix(case.n.clone(), piptree::prefix_find),
                case_ecf,
                "ECF mismatch using Prefix + PIPTree for {}",
                case.n
            );

            assert_eq!(
                path_extension(case.n.clone(), riptree::prefix_find),
                case_ecf,
                "ECF mismatch using Path + RIPTree for {}",
                case.n
            );

            assert_eq!(
                path_extension(case.n.clone(), piptree::prefix_find),
                case_ecf,
                "ECF mismatch using Path + PIPTree {}",
                case.n
            );
        }
    }
}
