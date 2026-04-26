/* https://leetcode.com/problems/running-sum-of-1d-array/description

Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.

Example 1:
Input: nums = [1,2,3,4]
Output: [1,3,6,10]
Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].

Example 2:
Input: nums = [1,1,1,1,1]
Output: [1,2,3,4,5]
Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].

Example 3:
Input: nums = [3,1,2,10,1]
Output: [3,4,6,16,17]

Constraints:
1 <= nums.length <= 1000
-10^6 <= nums[i] <= 10^6
*/

pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::from(nums);
    for i in 1..result.len() {
        result[i] += result[i - 1];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(running_sum(vec![1, 1, 2]), vec![1, 2, 4]);
    }

    #[test]
    fn example_3() {
        assert_eq!(running_sum(vec![3, 1, 2, 10, 1]), vec![3, 4, 6, 16, 17]);
    }
}
