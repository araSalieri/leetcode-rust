/* https://leetcode.com/problems/repeated-substring-pattern/description/

Given a string s, check if it can be constructed by taking a substring of it and appending multiple copies of the substring together.

Example 1:
Input: s = "abab"
Output: true
Explanation: It is the substring "ab" twice.

Example 2:
Input: s = "aba"
Output: false

Example 3:
Input: s = "abcabcabcabc"
Output: true
Explanation: It is the substring "abc" four times or the substring "abcabc" twice.


Constraints:
1 <= s.length <= 10^4
s consists of lowercase English letters.
*/

pub fn repeated_substring_pattern(s: String) -> bool {
    for step in 1..=s.len() / 2 {
        if s.len() % step == 0 {
            let pattern = &s[0..step];
            let mut i = step;

            while i < s.len() {
                if &s[i..i + step] != pattern {
                    break;
                }
                i += step;
            }

            if i == s.len() {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(repeated_substring_pattern(String::from("abab")), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(repeated_substring_pattern(String::from("aba")), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(
            repeated_substring_pattern(String::from("abcabcabcabc")),
            true
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(repeated_substring_pattern(String::from("a")), false);
    }

    #[test]
    fn example_5() {
        assert_eq!(repeated_substring_pattern(String::from("abaababaab")), true);
    }
}
