use std::env;

use collatz::{ecf, reduced_sequence, sequence};
use num_bigint::{BigUint, ToBigUint};
use num_traits::One;
use prefix::from_num;
use utils::to_path;

use crate::{collatz::length, prefix::to_num};

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

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Expected 2 arguments: function & number.");
        return;
    }

    let function = args[1].as_str();
    let n = args[2].parse::<BigUint>().unwrap();

    match function {
        "len" => println!("{}", length(n)),
        "seq" => println!("{:?}", sequence(n)),
        "rdseq" => println!("{:?}", reduced_sequence(n)),
        "ecf" => println!("{:?}", ecf(n)),
        "path" => println!("{:?}", to_path(&n)),
        "map" => println!("{:?}", from_num(n)),
        "pf-map" => println!(
            "{:?}",
            to_num(riptree::prefix_find(n.clone(), &to_path(&n)))
        ),
        "pf-rip" => println!("{:?}", riptree::prefix_find(n.clone(), &to_path(&n))),
        "pf-pip" => println!("{:?}", piptree::prefix_find(n.clone(), &to_path(&n))),
        "ecf-pf-rip" => println!("{:?}", iterative::prefix(&n, riptree::prefix_find)),
        "ecf-pf-pip" => println!("{:?}", iterative::prefix(&n, piptree::prefix_find)),
        "ecf-path-rip" => println!("{:?}", iterative::path_extension(&n, riptree::prefix_find)),
        "ecf-path-pip" => println!("{:?}", iterative::path_extension(&n, piptree::prefix_find)),
        _ => println!("Unknown function."),
    }
}
