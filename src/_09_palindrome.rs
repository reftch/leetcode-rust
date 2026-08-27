// 9. Palindrome Number
// Easy
// Topics
// premium lock icon
// Companies
// Hint
// Given an integer x, return true if x is a palindrome, and false otherwise.

// Example 1:

// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
// Example 2:

// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
// Example 3:

// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.

// optimized solution
pub fn is_palindrome(x: i32) -> bool {
    let mut reverted: i32 = 0;
    let mut original = x;
    while original > 0 {
        reverted = reverted * 10 + original % 10;
        original /= 10;
    }
    reverted == x
}

// idiomatic Rust
pub fn is_palindrome2(x: i32) -> bool {
    let s = x.to_string();
    s.chars().eq(s.chars().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome(121));
        assert!(!is_palindrome(-121));
        assert!(!is_palindrome(10));
        assert!(is_palindrome(1456541));
    }

    #[test]
    fn test_is_palindrome2() {
        assert!(is_palindrome2(121));
        assert!(!is_palindrome2(-121));
        assert!(!is_palindrome2(10));
        assert!(is_palindrome2(1456541));
    }
}
