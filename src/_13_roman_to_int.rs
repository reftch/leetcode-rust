// 13. Roman to Integer
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Hint
// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.

// Symbol       Value
// I             1
// V             5
// X             10
// L             50
// C             100
// D             500
// M             1000
// For example, 2 is written as II in Roman numeral, just two ones added together. 12 is written as XII, which is simply X + II. The number 27 is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given a roman numeral, convert it to an integer.

// Example 1:

// Input: s = "III"
// Output: 3
// Explanation: III = 3.
// Example 2:

// Input: s = "LVIII"
// Output: 58
// Explanation: L = 50, V= 5, III = 3.
// Example 3:

// Input: s = "MCMXCIV"
// Output: 1994
// Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.

struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let value = |b: u8| -> i32 {
            match b {
                b'I' => 1,
                b'V' => 5,
                b'X' => 10,
                b'L' => 50,
                b'C' => 100,
                b'D' => 500,
                b'M' => 1000,
                _ => 0,
            }
        };

        let (total, _) = s.bytes().fold((0, 0), |(total, prev), b| {
            let curr = value(b);
            if curr > prev {
                // Subtract prev twice: once to undo the previous addition, once for subtraction rule
                (total + curr - 2 * prev, curr)
            } else {
                (total + curr, curr)
            }
        });

        total
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_basic_cases() {
        assert_eq!(Solution::roman_to_int(String::from("III")), 3);
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("LVIII")), 58);
        assert_eq!(Solution::roman_to_int(String::from("MCMXCIV")), 1994);
    }

    #[test]
    fn test_single_chars() {
        assert_eq!(Solution::roman_to_int(String::from("I")), 1);
        assert_eq!(Solution::roman_to_int(String::from("V")), 5);
        assert_eq!(Solution::roman_to_int(String::from("X")), 10);
        assert_eq!(Solution::roman_to_int(String::from("L")), 50);
        assert_eq!(Solution::roman_to_int(String::from("C")), 100);
        assert_eq!(Solution::roman_to_int(String::from("D")), 500);
        assert_eq!(Solution::roman_to_int(String::from("M")), 1000);
    }

    #[test]
    fn test_subtraction_cases() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("IX")), 9);
        assert_eq!(Solution::roman_to_int(String::from("XL")), 40);
        assert_eq!(Solution::roman_to_int(String::from("XC")), 90);
        assert_eq!(Solution::roman_to_int(String::from("CD")), 400);
        assert_eq!(Solution::roman_to_int(String::from("CM")), 900);
    }

    #[test]
    fn test_complex_numbers() {
        assert_eq!(Solution::roman_to_int(String::from("MMMCMXCIX")), 3999);
        assert_eq!(Solution::roman_to_int(String::from("MMXXIII")), 2023);
        assert_eq!(Solution::roman_to_int(String::from("CDXLIV")), 444);
        assert_eq!(Solution::roman_to_int(String::from("DCXLIV")), 644);
    }

    #[test]
    fn test_large_values() {
        assert_eq!(
            Solution::roman_to_int(String::from("MMMDCCCLXXXVIII")),
            3888
        );
        assert_eq!(Solution::roman_to_int(String::from("MMDCCCLXXXVIII")), 2888);
    }

    #[test]
    fn test_all_same_chars() {
        assert_eq!(Solution::roman_to_int(String::from("IIII")), 4);
        assert_eq!(Solution::roman_to_int(String::from("VVVV")), 20);
        assert_eq!(Solution::roman_to_int(String::from("MMMM")), 4000);
    }

    #[test]
    fn test_increasing_values() {
        assert_eq!(Solution::roman_to_int(String::from("VI")), 6);
        assert_eq!(Solution::roman_to_int(String::from("XI")), 11);
        assert_eq!(Solution::roman_to_int(String::from("CL")), 150);
    }

    #[test]
    fn test_decreasing_values() {
        assert_eq!(Solution::roman_to_int(String::from("IV")), 4);
        assert_eq!(Solution::roman_to_int(String::from("XL")), 40);
    }
}
