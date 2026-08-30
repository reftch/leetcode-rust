struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let first = match strs.first() {
            Some(s) => s.as_bytes(),
            None => return String::new(),
        };

        for (i, &byte) in first.iter().enumerate() {
            for str in &strs[1..] {
                let bytes = str.as_bytes();
                if i >= bytes.len() || bytes[i] != byte {
                    return strs[0][..i].to_string();
                }
            }
        }

        strs[0].clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        let strs = vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight"),
        ];

        assert_eq!(Solution::longest_common_prefix(strs), "fl");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(Solution::longest_common_prefix(vec![]), "");
    }
}
