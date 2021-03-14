use std::env;

fn main() {
  let nbr_str = match env::args().nth(1) {
    Some(x) => x,
    None => "1000".to_string(),
  };
  let nbr = nbr_str.parse::<u32>().unwrap();
  println!("Solving problem for number {:?}", nbr);

  let result = solve(nbr);
  println!("Result = {:?}", result);
}

fn solve(sum: u32) -> u32 {
  /*
    Find the product a*b*c of the Pythagorean triple a^2+b^2=c^2 (a<b<c) whose sum a+b+c equals sum.
  */
  for a in 1..=sum-2 {
    for b in a..=sum-a-1 {
      for c in b..=sum-a-b {
        if a*a + b*b != c*c {
          continue;
        }
        // found a triplet, check sum
        if a + b + c == sum {
          return a*b*c;
        }
      }
    }
  }
  panic!("no suitable triplet found!");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn example() {
    assert_eq!(solve(12), 60); // a, b, c = 3, 4, 5
  }
}
