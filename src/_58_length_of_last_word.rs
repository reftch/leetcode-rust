struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let d: Vec<_> = s.trim().split(' ').collect();
        d[d.len() - 1].len() as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lenght_of_last_word() {
        assert_eq!(Solution::length_of_last_word("Hello World".to_string()), 5);
        assert_eq!(
            Solution::length_of_last_word("luffy is still joyboy".to_string()),
            6
        );
        assert_eq!(
            Solution::length_of_last_word("   fly me   to   the moon  ".to_string()),
            4
        );
    }
}
