#![feature(destructuring_assignment)]

use std::env;

fn main() {
  let nbr_str = match env::args().nth(1) {
    Some(x) => x,
    None => "20".to_string(),
  };
  let nbr = nbr_str.parse::<u32>().unwrap();
  println!("Solving problem for numbers upto {:?}", nbr);

  let result = solve(nbr);
  println!("Result = {:?}", result);
}

fn solve(upto: u32) -> u32 {
  /*
    Find the smallest number that can be evenly divided by all numbers in the range 1..=upto
  */
  let mut cur = upto;
  loop {
    let mut i = 2;
    while i <= upto {
      if cur % i != 0 {
        break;
      }
      i += 1;
    }
    if i > upto {
      // cur is divisible by all i (2..=upto)
      return cur;
    }

    cur += 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(10), 2520);
  }
}
