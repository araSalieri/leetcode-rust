/*
https://leetcode.com/problems/remove-duplicates-from-sorted-list/description/
Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

Example 1:
Input: head = [1,1,2]
Output: [1,2]

Example 2:
Input: head = [1,1,2,3,3]
Output: [1,2,3]

Constraints:
The number of nodes in the list is in the range [0, 300].
-100 <= Node.val <= 100
The list is guaranteed to be sorted in ascending order.
*/

use crate::list_node::ListNode;

pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn delete(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head {
            None => head,
            Some(mut node) => {
                node.next = delete(node.next);
                if node.next.as_ref().map_or(false, |n| n.val == node.val) {
                    node.next = node.next.take().unwrap().next;
                }
                Some(node)
            }
        }
    }

    delete(head)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_node::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            delete_duplicates(to_list(vec![1, 1, 2])),
            to_list(vec![1, 2])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            delete_duplicates(to_list(vec![1, 1, 2, 3, 3])),
            to_list(vec![1, 2, 3])
        );
    }
}
