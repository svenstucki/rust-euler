use std::ops::Range;

pub fn prime_sieve(upto: i64) -> Vec<i64> {
  let mut primes = Vec::with_capacity((upto/2) as usize);
  primes.push(1);

  let mut sieve = Vec::new();
  sieve.resize(upto as usize, false);
  for i in 2..=upto {
    let idx = (i-1) as usize;
    if sieve[idx] {
      // skip already marked entries
      continue;
    }
    // current number is prime, mark multiples
    primes.push(i);
    for j in (Range { start: i*i, end: upto+1 }).step_by(i as usize) {
      sieve[(j-1) as usize] = true;
    }
  }

  return primes;
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn small() {
    assert_eq!(prime_sieve(10), vec![1, 2, 3, 5, 7]);
  }

  #[test]
  fn upper_bound() {
    assert_eq!(prime_sieve(6), vec![1, 2, 3, 5]);
    assert_eq!(prime_sieve(7), vec![1, 2, 3, 5, 7]);
  }
}
