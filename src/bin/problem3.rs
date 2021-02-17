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
  // get all primes up to and including number
  let primes = euler::utils::prime_sieve::prime_sieve(nbr);

  for p in primes.iter().rev() {
    if nbr % p == 0 {
      return *p;
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
