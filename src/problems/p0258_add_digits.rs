/* https://leetcode.com/problems/add-digits/description/

Given an integer num, repeatedly add all its digits until the result has only one digit, and return it.

Example 1:
Input: num = 38
Output: 2
Explanation: The process is
38 --> 3 + 8 --> 11
11 --> 1 + 1 --> 2
Since 2 has only one digit, return it.

Example 2:
Input: num = 0
Output: 0

Constraints:
0 <= num <= 2^31 - 1
*/

pub fn add_digits(num: i32) -> i32 {
    let mut num = num;
    loop {
        let str_num = num.to_string();
        if str_num.len() == 1 {
            return num;
        }

        num = str_num
            .chars()
            .map(|c| c.to_digit(10).unwrap() as i32)
            .sum();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(add_digits(38), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(add_digits(0), 0);
    }
}
