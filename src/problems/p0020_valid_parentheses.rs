/* https://leetcode.com/problems/valid-parentheses/description/

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

Open brackets must be closed by the same type of brackets.
Open brackets must be closed in the correct order.
Every close bracket has a corresponding open bracket of the same type.

Example 1:
Input: s = "()"
Output: true

Example 2:
Input: s = "()[]{}"
Output: true

Example 3:
Input: s = "(]"
Output: false

Example 4:
Input: s = "([])"
Output: true

Example 5:
Input: s = "([)]"
Output: false

Constraints:
1 <= s.length <= 10^4
s consists of parentheses only '()[]{}'.
*/

pub fn is_valid(s: String) -> bool {
    if s.len() == 1 {
        return false;
    }
    let mut stack: Vec<char> = Vec::new();
    let mut valid = false;
    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
            continue;
        }

        let close_bracket = stack.pop();
        match close_bracket {
            Some(v) => {
                if (c == ')' && v == '(') || (c == ']' && v == '[') || (c == '}' && v == '{') {
                    valid = true;
                    continue;
                } else {
                    return false;
                }
            }
            None => return false,
        }
    }

    if let Some(_) = stack.pop() {
        return false;
    }

    valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(is_valid(String::from("{}")), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(is_valid(String::from("()[]{}")), true);
    }

    #[test]
    fn example_3() {
        assert_eq!(is_valid(String::from("(]")), false);
    }

    #[test]
    fn example_4() {
        assert_eq!(is_valid(String::from("([])")), true);
    }

    #[test]
    fn example_5() {
        assert_eq!(is_valid(String::from("([)]")), false);
    }
}
