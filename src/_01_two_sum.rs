// 1. Two Sum
// Solved
// Easy
// Topics
// premium lock icon
// Companies
// Hint
// You are given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]
// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = std::collections::HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, index as i32];
        }
        map.insert(num, index);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
}
