use std::env;

fn main() {
    let nbr_str = match env::args().nth(1) {
        Some(x) => x,
        None => "100".to_string(),
    };
    let nbr = nbr_str.parse::<u32>().unwrap();
    println!("Solving problem for numbers upto {:?}", nbr);

    let result = solve(nbr);
    println!("Result = {:?}", result);
}

fn solve(upto: u32) -> u32 {
    /*
      Find the difference between the sum of the squares of the numbers 1..=upto and the square of the sum.
    */
    let sum = upto * (1 + upto) / 2;
    let mut sum_of_squares = 0;
    for i in 1..=upto {
        sum_of_squares += i * i;
    }
    return sum * sum - sum_of_squares;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(10), 2640);
    }
}
