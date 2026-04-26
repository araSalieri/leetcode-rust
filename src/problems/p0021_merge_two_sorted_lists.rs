/* https://leetcode.com/problems/merge-two-sorted-lists/

You are given the heads of two sorted linked lists list1 and list2.
Merge the two lists into one sorted list. The list should be made by splicing together the nodes of the first two lists.
Return the head of the merged linked list.

Example 1:
Input: list1 = [1,2,4], list2 = [1,3,4]
Output: [1,1,2,3,4,4]

Example 2:
Input: list1 = [], list2 = []
Output: []

Example 3:
Input: list1 = [], list2 = [0]
Output: [0]
*/

use crate::list_node::ListNode;

pub fn merge_two_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    fn merge(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, list2) => list2,
            (list1, None) => list1,
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    n1.next = merge(n1.next, Some(n2));
                    Some(n1)
                } else {
                    n2.next = merge(Some(n1), n2.next);
                    Some(n2)
                }
            }
        }
    }
    return merge(list1, list2);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_node::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            merge_two_lists(to_list(vec![1, 2, 4]), to_list(vec![1, 3, 4])),
            to_list(vec![1, 1, 2, 3, 4, 4])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(merge_two_lists(None, None), None);
    }

    #[test]
    fn example_3() {
        assert_eq!(merge_two_lists(None, to_list(vec![0])), to_list(vec![0]));
    }
}
