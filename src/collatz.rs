use num_bigint::{BigUint, ToBigUint};

pub fn collatz_length(mut n: BigUint) -> i32 {
    let mut ans: i32 = 0;
    while !n.eq(&1.to_biguint().unwrap()) {
        ans += 1;
        if !n.bit(0) {
            n >>= 1;
        } else {
            n = 3.to_biguint().unwrap() * n + 1.to_biguint().unwrap();
        }
    }
    ans + 1
}

#[cfg(test)]
mod tests {
    use num_bigint::ToBigUint;

    use crate::collatz::collatz_length;

    #[test]
    fn it_works() {
        assert_eq!(collatz_length(5.to_biguint().unwrap()), 6);
    }
}
