/* https://leetcode.com/problems/contains-duplicate/description/

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.

Example 1:
Input: nums = [1,2,3,1]
Output: true
Explanation:
The element 1 occurs at the indices 0 and 3.

Example 2:
Input: nums = [1,2,3,4]
Output: false
Explanation:
All elements are distinct.

Example 3:
Input: nums = [1,1,1,3,3,4,3,2,4,2]
Output: true

Constraints:
1 <= nums.length <= 10^5
-10^9 <= nums[i] <= 10^9
*/

use std::collections::HashSet;

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut unique_num = HashSet::new();
    for num in nums {
        if !unique_num.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
    }
}
