/* https://leetcode.com/problems/length-of-last-word/description/

Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.

Example 1:
Input: s = "Hello World"
Output: 5
Explanation: The last word is "World" with length 5.

Example 2:
Input: s = "   fly me   to   the moon  "
Output: 4
Explanation: The last word is "moon" with length 4.

Example 3:
Input: s = "luffy is still joyboy"
Output: 6
Explanation: The last word is "joyboy" with length 6.

Constraints:
1 <= s.length <= 10^4
s consists of only English letters and spaces ' '.
There will be at least one word in s.
*/

pub fn length_of_last_word(s: String) -> i32 {
    let mut max_len = 0;
    let mut temp_len = 0;

    for i in s.chars() {
        if i != ' ' {
            temp_len += 1;
            continue;
        }

        if temp_len > 0 {
            max_len = temp_len
        }

        temp_len = 0
    }

    if temp_len > 0 {
        max_len = temp_len
    }

    max_len
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(length_of_last_word(String::from("Hello World")), 5);
    }

    #[test]
    fn example_2() {
        assert_eq!(
            length_of_last_word(String::from("luffy is still joyboy")),
            6
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(length_of_last_word(String::from("Today is a nice day")), 3);
    }
}
