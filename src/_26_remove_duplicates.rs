struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0; // Position for the next unique element

        for j in 1..nums.len() {
            if nums[j] != nums[i] {
                // Compare with last unique element
                i += 1;
                nums[i] = nums[j]; // Move unique element to front
            }
        }

        (i + 1) as i32 // Number of unique elements
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_duplicates() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(Solution::remove_duplicates(&mut nums), 5);
        let mut nums = vec![1, 1, 2];
        assert_eq!(Solution::remove_duplicates(&mut nums), 2);
    }
}
