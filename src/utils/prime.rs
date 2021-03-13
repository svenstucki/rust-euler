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

pub fn is_prime(nbr: u64) -> bool {
  if nbr <= 1 {
    return false;
  }
  if nbr >= 3 && nbr % 2 == 0 {
    return false;
  }

  for i in 2..nbr {
    if nbr % i == 0 {
      return false;
    }
  }
  return true;
}

#[cfg(test)]
mod test_prime_sieve {
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_is_prime() {
    assert_eq!(is_prime(1), false);
    assert_eq!(is_prime(2), true);
    assert_eq!(is_prime(3), true);
    assert_eq!(is_prime(4), false);
    assert_eq!(is_prime(89), true);
    assert_eq!(is_prime(90), false);
    assert_eq!(is_prime(97), true);
    assert_eq!(is_prime(99), false);
    assert_eq!(is_prime(227), true);
  }
}
