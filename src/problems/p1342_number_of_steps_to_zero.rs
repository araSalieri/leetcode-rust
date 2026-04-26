/* https://leetcode.com/problems/number-of-steps-to-reduce-a-number-to-zero/description/

Given an integer num, return the number of steps to reduce it to zero.
In one step, if the current number is even, you have to divide it by 2, otherwise, you have to subtract 1 from it.

Example 1:
Input: num = 14
Output: 6
Explanation:
Step 1) 14 is even; divide by 2 and obtain 7.
Step 2) 7 is odd; subtract 1 and obtain 6.
Step 3) 6 is even; divide by 2 and obtain 3.
Step 4) 3 is odd; subtract 1 and obtain 2.
Step 5) 2 is even; divide by 2 and obtain 1.
Step 6) 1 is odd; subtract 1 and obtain 0.

Example 2:
Input: num = 8
Output: 4
Explanation:
Step 1) 8 is even; divide by 2 and obtain 4.
Step 2) 4 is even; divide by 2 and obtain 2.
Step 3) 2 is even; divide by 2 and obtain 1.
Step 4) 1 is odd; subtract 1 and obtain 0.

Example 3:
Input: num = 123
Output: 12

Constraints:
0 <= num <= 10^6
*/

pub fn number_of_steps(num: i32) -> i32 {
    let mut state = num;
    let mut steps = 0;
    while state > 0 {
        if state % 2 == 0 {
            state /= 2;
        } else {
            state -= 1;
        }
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(number_of_steps(14), 6);
    }

    #[test]
    fn example_2() {
        assert_eq!(number_of_steps(8), 4);
    }

    #[test]
    fn example_3() {
        assert_eq!(number_of_steps(123), 12);
    }
}
