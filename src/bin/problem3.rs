use std::env;
use euler::utils::prime;

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

fn solve(mut nbr: i64) -> i64 {
  let mut cur = 3;
  if nbr % 2 == 0 {
    nbr = nbr / 2;
  }
  loop {
    if nbr % cur == 0 && prime::is_prime(cur) {
      nbr = nbr / cur;
      println!("discovered factor {:?}, new number is {:?}", cur, nbr);
      if nbr == 1 {
        return cur;
      }
    }
    cur += 2;
    if cur > nbr {
      panic!("a prime factor was missed!");
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(13195), 29);
  }
}
