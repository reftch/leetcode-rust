// 35. Search Insert Position
//
// Given a sorted array of distinct integers and a target value, return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;
        for _ in 0..nums.len() {
            if nums[i] == target
                || (i > 0 && nums[i] > target && nums[i - 1] < target)
                || nums[i] > target
            {
                return i as i32;
            }
            i += 1;
        }

        i as i32
    }
}

#[cfg(test)]
mod test {
    use crate::_35_search_insert::Solution;

    #[test]
    fn test_search_insert() {
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 0), 0);
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 5), 2);
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 2), 1);
        let nums = vec![1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums, 7), 4);
    }
}
