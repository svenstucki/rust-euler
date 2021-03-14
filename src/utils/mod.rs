pub mod prime;

pub fn is_palindrome(input: String) -> bool {
    let bytes = input.as_bytes();
    let max = bytes.len() / 2;

    let mut forward = bytes.iter();
    let mut backward = bytes.iter().rev();
    for _ in 0..max {
        if forward.next() != backward.next() {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert_eq!(is_palindrome(String::from("1001")), true);
        assert_eq!(is_palindrome(String::from("1002")), false);
        assert_eq!(is_palindrome(String::from("10001")), true);
        assert_eq!(is_palindrome(String::from("10201")), true);
        assert_eq!(is_palindrome(String::from("12201")), false);
    }
}
