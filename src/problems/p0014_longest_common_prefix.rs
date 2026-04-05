// https://leetcode.com/problems/longest-common-prefix/
// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:

// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

// Constraints:

// 1 <= strs.length <= 200
// 0 <= strs[i].length <= 200
// strs[i] consists of only lowercase English letters if it is non-empty.

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let base = strs[0].as_bytes();

    for i in 0..base.len() {
        for word in &strs {
            if i >= word.len() || word.as_bytes()[i] != base[i] {
                return strs[0][..i].to_string();
            }
        }
    }

    strs[0].clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight"),
            ]),
            "fl"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car"),
            ]),
            ""
        );
    }
}
