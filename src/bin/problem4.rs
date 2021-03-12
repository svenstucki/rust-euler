#![feature(destructuring_assignment)]

use std::env;
use euler::utils;

fn main() {
  let nbr_str = match env::args().nth(1) {
    Some(x) => x,
    None => "3".to_string(),
  };
  let nbr = nbr_str.parse::<u32>().unwrap();
  println!("Solving problem for {:?} digits", nbr);

  let result = solve(nbr);
  println!("Result = {:?}", result);
}

fn solve(digits: u32) -> u32 {
  /*
    Find the largest palindrome made from the product of two digits-digit number.
    Solved with a naive approach, since the problem space has less than 1mln entries.
    It's surprisingly hard to determine unique products, see here for some background:
    https://math.dartmouth.edu/~carlp/sumproductboston.pdf
  */
  let min = 10u32.pow(digits-1);
  let max = 10u32.pow(digits); // numbers are in range min .. max-1

  let mut largest_palindrome = 0;
  for i in min..max {
    for j in min..max {
      let product = i*j;
      if utils::is_palindrome(product.to_string()) {
        if product > largest_palindrome {
          largest_palindrome = product;
        }
      }
    }
  }
  largest_palindrome
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(2), 9009);
  }
}
