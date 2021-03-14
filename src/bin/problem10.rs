use euler::utils::prime;
use std::env;

fn main() {
    let nbr_str = match env::args().nth(1) {
        Some(x) => x,
        None => "2000000".to_string(),
    };
    let nbr = nbr_str.parse::<u32>().unwrap();
    println!("Solving problem for number {:?}", nbr);

    let result = solve(nbr);
    println!("Result = {:?}", result);
}

fn solve(nbr: u32) -> u64 {
    /*
      Find the sum of all primes below nbr.
    */
    let primes = prime::prime_sieve(nbr);
    return primes.iter().map(|p| *p as u64).sum();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(solve(10), 17);
    }
}
