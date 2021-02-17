use std::env;

fn main() {
  let upto_str = match env::args().nth(1) {
    Some(x) => x,
    None => "1000".to_string(),
  };
  let upto = upto_str.parse::<i32>().unwrap();
  println!("Solving problem for numbers upto {:?}", upto);

  let sum = solve(upto);
  println!("Result = {:?}", sum);
}

fn solve(upto: i32) -> i32 {
  let mut sum = 0;
  for i in 1..upto {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }
  sum
}
