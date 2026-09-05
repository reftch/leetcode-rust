// 28. Find the Index of the First Occurrence in a String
//
// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
// Example 2:

// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.

struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack_len = haystack.len();
        let needle_len = needle.len();

        if needle_len == 0 {
            return 0;
        }
        if haystack_len < needle_len {
            return -1;
        }

        for i in 0..=haystack_len - needle_len {
            let mut matches = true;
            for j in 0..needle_len {
                if haystack.as_bytes()[i + j] != needle.as_bytes()[j] {
                    matches = false;
                    break;
                }
            }
            if matches {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_str() {
        assert_eq!(
            Solution::str_str("sadbutsad".to_string(), "sad".to_string()),
            0
        );
        assert_eq!(
            Solution::str_str("leetcode".to_string(), "leeto".to_string()),
            -1
        );
        assert_eq!(Solution::str_str("xabcy".to_string(), "abc".to_string()), 1);
        assert_eq!(Solution::str_str("aaaa".to_string(), "aaaa".to_string()), 0);
        assert_eq!(Solution::str_str("aaa".to_string(), "aaaa".to_string()), -1);
        assert_eq!(
            Solution::str_str("mississippi".to_string(), "issipi".to_string()),
            -1
        );
    }
}
