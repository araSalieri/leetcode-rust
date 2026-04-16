// https://leetcode.com/problems/ransom-note/description/
// Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.
// Each letter in magazine can only be used once in ransomNote.

// Example 1:
// Input: ransomNote = "a", magazine = "b"
// Output: false

// Example 2:
// Input: ransomNote = "aa", magazine = "ab"
// Output: false

// Example 3:
// Input: ransomNote = "aa", magazine = "aab"
// Output: true

// Constraints:
// 1 <= ransomNote.length, magazine.length <= 10^5
// ransomNote and magazine consist of lowercase English letters.

use std::collections::HashMap;

pub fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut ransom_map = HashMap::new();
    for c in ransom_note.chars() {
        *ransom_map.entry(c).or_insert(0) += 1;
    }

    let ransom_len = ransom_note.len();
    let mut count = 0;

    for c in magazine.chars() {
        let ransom_count = ransom_map.get(&c).copied().unwrap_or(0);
        if ransom_count > 0 {
            count += 1;
            *ransom_map.entry(c).or_insert(0) -= 1;
        }

        if ransom_len == count {
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
        assert_eq!(can_construct(String::from("a"), String::from("b")), false);
    }

    #[test]
    fn example_2() {
        assert_eq!(can_construct(String::from("aa"), String::from("ab")), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(can_construct(String::from("aa"), String::from("aab")), true);
    }

    #[test]
    fn example_4() {
        assert_eq!(
            can_construct(
                String::from("fihjjjjei"),
                String::from("hjibagacbhadfaefdjaeaebgi")
            ),
            false
        );
    }
}
