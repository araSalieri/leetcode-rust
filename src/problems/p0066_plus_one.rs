/* https://leetcode.com/problems/plus-one/description/

You are given a large integer represented as an integer array digits, where each digits[i] is the ith digit of the integer. The digits are ordered from most significant to least significant in left-to-right order. The large integer does not contain any leading 0's.
Increment the large integer by one and return the resulting array of digits.

Example 1:
Input: digits = [1,2,3]
Output: [1,2,4]
Explanation: The array represents the integer 123.
Incrementing by one gives 123 + 1 = 124.
Thus, the result should be [1,2,4].

Example 2:
Input: digits = [4,3,2,1]
Output: [4,3,2,2]
Explanation: The array represents the integer 4321.
Incrementing by one gives 4321 + 1 = 4322.
Thus, the result should be [4,3,2,2].

Example 3:
Input: digits = [9]
Output: [1,0]
Explanation: The array represents the integer 9.
Incrementing by one gives 9 + 1 = 10.
Thus, the result should be [1,0].

Constraints:
1 <= digits.length <= 100
0 <= digits[i] <= 9
digits does not contain any leading 0's.
*/

pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let len = digits.len();
    let mut output = vec![0; len + 1];

    output[len] = 1;
    for i in (0..len).rev() {
        let j = i + 1;
        output[j] = output[j] + digits[i];
        if output[j] > 9 {
            output[j] = 0;
            output[i] = 1;
        }
    }

    if output[0] == 0 {
        return output[1..].to_vec();
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(plus_one(vec![1, 2, 3]), vec![1, 2, 4]);
    }

    #[test]
    fn example_2() {
        assert_eq!(plus_one(vec![4, 3, 2, 1]), vec![4, 3, 2, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(plus_one(vec![9]), vec![1, 0]);
    }
}
