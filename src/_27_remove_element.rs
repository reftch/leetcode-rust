// 27. Remove Element
//
// Given an integer array nums and an integer val, remove all occurrences of val in nums in-place.
// The order of the elements may be changed.
// Then return the number of elements in nums which are not equal to val.

// Consider the number of elements in nums which are not equal to val be k,
// to get accepted, you need to do the following things:

// Change the array nums such that the first k elements of nums contain the elements
// which are not equal to val. The remaining elements of nums are not important as well as the size of nums.
// Return k.

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut i = 0;
        for j in 0..nums.len() {
            if nums[j] != val {
                nums[i] = nums[j]; // Move not val element to front
                i += 1;
            }
        }

        i as i32
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_remove_element() {
        let mut nums = vec![3, 2, 2, 3];
        assert_eq!(Solution::remove_element(&mut nums, 3), 2);
        let mut nums = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element(&mut nums, 2), 5);
    }
}
