#![feature(destructuring_assignment)]

use std::env;
use euler;

fn main() {
  let nbr_str = match env::args().nth(1) {
    Some(x) => x,
    None => "600851475143".to_string(),
  };
  let nbr = nbr_str.parse::<i64>().unwrap();
  println!("Solving problem for number {:?}", nbr);

  let result = solve(nbr);
  println!("Result = {:?}", result);
}

fn solve(nbr: i64) -> i64 {
  for cur in (1..=nbr).rev() {
    if nbr % cur != 0 {
      continue;
    }
    println!("checking factor {:?}", cur);
    if euler::utils::prime::is_prime(cur) {
      return cur;
    }
  }
  panic!("something went wrong, there must be at least one prime factor");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(13195), 29);
  }
}
