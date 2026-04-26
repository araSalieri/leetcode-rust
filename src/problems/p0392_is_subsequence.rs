/* https://leetcode.com/problems/is-subsequence/description/

Given two strings s and t, return true if s is a subsequence of t, or false otherwise.
A subsequence of a string is a new string that is formed from the original string by deleting some (can be none) of the characters without disturbing the relative positions of the remaining characters. (i.e., "ace" is a subsequence of "abcde" while "aec" is not).

Example 1:
Input: s = "abc", t = "ahbgdc"
Output: true

Example 2:
Input: s = "axc", t = "ahbgdc"
Output: false

Constraints:
0 <= s.length <= 100
0 <= t.length <= 10^4
s and t consist only of lowercase English letters.
Follow up: Suppose there are lots of incoming s, say s1, s2, ..., sk where k >= 10^9, and you want to check one by one to see if t has its subsequence. In this scenario, how would you change your code?
*/

pub fn is_subsequence(s: String, t: String) -> bool {
    let s_len = s.len();
    let t_len = t.len();
    if s_len == 0 {
        return true;
    }

    if t_len == 0 || s_len > t_len {
        return false;
    }

    let mut index: usize = 0;
    for i in 0..t_len {
        if t.as_bytes()[i] == s.as_bytes()[index] {
            index += 1;
        }

        if index == s_len {
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
        assert_eq!(
            is_subsequence(String::from("abc"), String::from("ahbgdc")),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            is_subsequence(String::from("axc"), String::from("ahbgdc")),
            false
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(is_subsequence(String::from("abc"), String::from("")), false);
    }

    #[test]
    fn example_4() {
        assert_eq!(is_subsequence(String::from(""), String::from("abc")), true);
    }
}
