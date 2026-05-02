/* https://leetcode.com/problems/reverse-linked-list/description

Given the head of a singly linked list, reverse the list, and return the reversed list.

Example 1:
Input: head = [1,2,3,4,5]
Output: [5,4,3,2,1]

Example 2:
Input: head = [1,2]
Output: [2,1]

Example 3:
Input: head = []
Output: []

Constraints:
The number of nodes in the list is the range [0, 5000].
-5000 <= Node.val <= 5000
*/

use crate::list_node::ListNode;

pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev: Option<Box<ListNode>> = None;
    let mut curr = head;
    while let Some(mut node) = curr {
        curr = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_node::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            reverse_list(to_list(vec![1, 2, 3, 4, 5])),
            to_list(vec![5, 4, 3, 2, 1])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(reverse_list(to_list(vec![1, 2])), to_list(vec![2, 1]));
    }

    #[test]
    fn example_3() {
        assert_eq!(reverse_list(to_list(vec![])), to_list(vec![]));
    }
}
