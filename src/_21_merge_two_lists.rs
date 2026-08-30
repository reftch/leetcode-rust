// 21. Merge Two Sorted Lists
// Easy
// Topics
// premium lock icon
// Companies
// You are given the heads of two sorted linked lists list1 and list2.

// Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.

// Return the head of the merged linked list.
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
// Example 2:

// Input: list1 = [], list2 = []
// Output: []
// Example 3:

// Input: list1 = [], list2 = [0]
// Output: [0]

struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        let mut list1 = list1;
        let mut list2 = list2;

        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            if node1.val <= node2.val {
                // Take ownership of list1's node and advance list1
                let mut next = list1.take().unwrap();
                list1 = next.next.take();
                tail.next = Some(next);
            } else {
                // Take ownership of list2's node and advance list2
                let mut next = list2.take().unwrap();
                list2 = next.next.take();
                tail.next = Some(next);
            }
            tail = tail.next.as_mut().unwrap();
        }

        // Attach remaining elements directly without iterating
        tail.next = list1.or(list2);

        dummy.next
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // Helper: build a list from a slice
    fn to_list(vals: &[i32]) -> Option<Box<ListNode>> {
        match vals {
            [] => None,
            [first, rest @ ..] => {
                let mut node = Box::new(ListNode::new(*first));
                node.next = to_list(rest);
                Some(node)
            }
        }
    }

    // Helper: collect a list into a Vec
    fn to_vec(list: Option<Box<ListNode>>) -> Vec<i32> {
        let mut result = Vec::new();
        let mut current = list;
        while let Some(mut node) = current {
            result.push(node.val);
            current = node.next.take();
        }
        result
    }

    #[test]
    fn merge_two_non_empty_lists() {
        let list1 = to_list(&[1, 2, 4]);
        let list2 = to_list(&[1, 3, 4]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn merge_empty_and_non_empty() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[0, 5, 6]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![0, 5, 6]);
    }

    #[test]
    fn merge_two_empty_lists() {
        let list1 = to_list(&[]);
        let list2 = to_list(&[]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), Vec::<i32>::new());
    }

    #[test]
    fn lists_with_duplicate_values() {
        let list1 = to_list(&[1, 1, 3]);
        let list2 = to_list(&[1, 2, 2]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![1, 1, 1, 2, 2, 3]);
    }

    #[test]
    fn single_element_lists() {
        let list1 = to_list(&[5]);
        let list2 = to_list(&[3]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![3, 5]);
    }

    #[test]
    fn one_list_smaller_than_other() {
        let list1 = to_list(&[1, 2, 3, 4, 5]);
        let list2 = to_list(&[2, 4]);
        let merged = Solution::merge_two_lists(list1, list2);
        assert_eq!(to_vec(merged), vec![1, 2, 2, 3, 4, 4, 5]);
    }
}
