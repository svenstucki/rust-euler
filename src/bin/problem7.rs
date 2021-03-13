use std::env;
use euler::utils::prime;

fn main() {
  let nbr_str = match env::args().nth(1) {
    Some(x) => x,
    None => "10001".to_string(),
  };
  let nbr = nbr_str.parse::<u32>().unwrap();
  println!("Solving problem for number {:?}", nbr);

  let result = solve(nbr);
  println!("Result = {:?}", result);
}

fn solve(nbr: u32) -> u64 {
  /*
    Find the nbr'st prime number
  */
  let mut cur = 3;
  let mut cnt = 1;
  loop {
    if prime::is_prime(cur) {
      cnt += 1;
      if cnt >= nbr {
        return cur;
      }
    }
    cur += 2;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(6), 13);
  }
}
