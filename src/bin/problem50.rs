#![feature(test)]

use euler::utils::prime;
use std::env;
use std::time::Instant;

fn main() {
    let nbr_str = match env::args().nth(1) {
        Some(x) => x,
        None => "1000000".to_string(),
    };
    let nbr = nbr_str.parse::<u32>().unwrap();
    println!("Solving problem for number {:?}", nbr);

    let start = Instant::now();
    let result = solve(nbr);
    let duration = start.elapsed();
    println!("Result is {:?}, calculated in {:?}", result, duration);
}

fn solve(nbr: u32) -> u32 {
    /*
      Find the prime below nbr, made from the longest consecutive sum of primes.
    */
    let primes = prime::prime_sieve(nbr);

    let mut longest_chain = 0;
    let mut longest_chain_prime = 0;

    for start_idx in 0..primes.len() {
        let mut sum = primes[start_idx];
        println!("start_idx = {:?}, start_prime = {:?}", start_idx, sum);

        for idx in (start_idx + 1)..(primes.len() - longest_chain) {
            let prime = primes[idx];
            sum += prime;

            if sum >= nbr {
                break;
            }

            if idx - start_idx + 1 > longest_chain && primes.contains(&sum) {
                // sum is prime and has longer chain
                println!("** new longest chain for: {:?} with {:?} elements", sum, idx - start_idx + 1);
                longest_chain = idx - start_idx + 1;
                longest_chain_prime = sum;
            }

        }
    }

    longest_chain_prime
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_solve(b: &mut Bencher) {
        b.iter(|| solve(10_000));
    }

    #[test]
    fn example_small() {
        assert_eq!(solve(100), 41);
    }

    #[test]
    fn example_large() {
        assert_eq!(solve(1000), 953);
    }
}
