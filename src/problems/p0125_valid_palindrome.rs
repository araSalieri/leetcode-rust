/* https://leetcode.com/problems/valid-palindrome/description/

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

Example 1:
Input: s = "A man, a plan, a canal: Panama"
Output: true
Explanation: "amanaplanacanalpanama" is a palindrome.

Example 2:
Input: s = "race a car"
Output: false
Explanation: "raceacar" is not a palindrome.

Example 3:
Input: s = " "
Output: true
Explanation: s is an empty string "" after removing non-alphanumeric characters.
Since an empty string reads the same forward and backward, it is a palindrome.

Constraints:
1 <= s.length <= 2 * 10^5
s consists only of printable ASCII characters.
*/

pub fn is_palindrome(s: String) -> bool {
    let mut s = s.to_lowercase();
    s.retain(|c| c.is_alphanumeric());
    let len_s = s.len();

    for i in 0..(len_s / 2) {
        if s.as_bytes()[i] != s.as_bytes()[len_s - 1 - i] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            is_palindrome(String::from("A man, a plan, a canal: Panama")),
            true
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(is_palindrome(String::from("race a car")), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(is_palindrome(String::from(" ")), true);
    }
}
