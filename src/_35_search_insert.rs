// 35. Search Insert Position
//
// Given a sorted array of distinct integers and a target value, return the index if the target is found.
// If not, return the index where it would be if it were inserted in order.

// You must write an algorithm with O(log n) runtime complexity.

struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;
            if nums[mid] < target {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as i32
    }
}

#[cfg(test)]
mod test {
    use crate::_35_search_insert::Solution;

    #[test]
    fn test_search_insert() {
        let nums = vec![-1, 3, 5, 6];
        assert_eq!(Solution::search_insert(nums.clone(), 0), 1);
        assert_eq!(Solution::search_insert(nums.clone(), 5), 2);
        assert_eq!(Solution::search_insert(nums.clone(), 2), 1);
        assert_eq!(Solution::search_insert(nums.clone(), 7), 4);
    }
}
