#![feature(destructuring_assignment)]
use std::env;

fn main() {
  let upto_str = match env::args().nth(1) {
    Some(x) => x,
    None => "4000000".to_string(),
  };
  let upto = upto_str.parse::<i32>().unwrap();
  println!("Solving problem for numbers upto {:?}", upto);

  let sum = solve(upto);
  println!("Result = {:?}", sum);
}

fn solve(upto: i32) -> i32 {
  let mut sum = 0;

  let mut last = 1;
  let mut cur = 1;
  loop {
    if cur % 2 == 0 {
      sum += cur;
    }

    // calculate next fibonacci number
    (last, cur) = (cur, last + cur);

    if cur >= upto {
      break;
    }
  }

  sum
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(90), 44)
  }
}
