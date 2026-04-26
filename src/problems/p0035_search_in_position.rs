/* https://leetcode.com/problems/search-insert-position/description/

Given a sorted array of distinct integers and a target value, return the index if the target is found. If not, return the index where it would be if it were inserted in order.

You must write an algorithm with O(log n) runtime complexity.

Example 1:
Input: nums = [1,3,5,6], target = 5
Output: 2

Example 2:
Input: nums = [1,3,5,6], target = 2
Output: 1

Example 3:
Input: nums = [1,3,5,6], target = 7
Output: 4

Constraints:
1 <= nums.length <= 10^4
-10^4 <= nums[i] <= 10^4
nums contains distinct values sorted in ascending order.
-10^4 <= target <= 10^4
*/

pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    let len = nums.len();

    if target < nums[0] {
        return 0;
    }

    for i in 0..len {
        if nums[i] == target {
            return i as i32;
        }

        if i + 1 < len && target < nums[i + 1] {
            return (i + 1) as i32;
        }
    }
    return len as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
    }

    #[test]
    fn example_3() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
    }
}
