/* https://leetcode.com/problems/remove-linked-list-elements/description/

Given the head of a linked list and an integer val, remove all the nodes of the linked list that has Node.val == val, and return the new head.

Example 1:
Input: head = [1,2,6,3,4,5,6], val = 6
Output: [1,2,3,4,5]

Example 2:
Input: head = [], val = 1
Output: []

Example 3:
Input: head = [7,7,7,7], val = 7
Output: []

Constraints:
The number of nodes in the list is in the range [0, 10^4].
1 <= Node.val <= 50
0 <= val <= 50
*/

use crate::list_node::ListNode;

pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
    fn remove(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        match head {
            None => head,
            Some(mut node) => {
                if node.val == val {
                    return remove(node.next, val);
                }
                node.next = remove(node.next, val);
                Some(node)
            }
        }
    }

    remove(head, val)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list_node::to_list;

    #[test]
    fn example_1() {
        assert_eq!(
            remove_elements(to_list(vec![1, 2, 6, 3, 4, 5, 6]), 6),
            to_list(vec![1, 2, 3, 4, 5])
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(remove_elements(to_list(vec![]), 1), to_list(vec![]));
    }
    #[test]
    fn example_3() {
        assert_eq!(
            remove_elements(to_list(vec![7, 7, 7, 7, 7, 7, 7]), 7),
            to_list(vec![])
        );
    }
}
