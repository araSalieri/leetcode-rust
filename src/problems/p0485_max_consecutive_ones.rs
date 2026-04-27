/* https://leetcode.com/problems/max-consecutive-ones/description/

Given a binary array nums, return the maximum number of consecutive 1's in the array.

Example 1:
Input: nums = [1,1,0,1,1,1]
Output: 3
Explanation: The first two digits or the last three digits are consecutive 1s. The maximum number of consecutive 1s is 3.

Example 2:
Input: nums = [1,0,1,1,0,1]
Output: 2

Constraints:
1 <= nums.length <= 10^5
nums[i] is either 0 or 1.
*/

pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
    let mut counter_max = 0;
    let mut counter = 0;
    for num in nums {
        if num == 1 {
            counter += 1;
            counter_max = counter_max.max(counter);
        } else {
            counter = 0
        }
    }

    counter_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1]), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(find_max_consecutive_ones(vec![1, 0, 1, 1, 0, 1]), 2);
    }
}
