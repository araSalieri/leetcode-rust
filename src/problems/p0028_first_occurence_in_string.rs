// https://leetcode.com/problems/find-the-index-of-the-first-occurrence-in-a-string/description/
// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.

// Example 1:

// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
// Example 2:

// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.

// Constraints:

// 1 <= haystack.length, needle.length <= 104
// haystack and needle consist of only lowercase English characters.

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    for i in 0..haystack_len {
        let window = i + needle_len;
        if window > haystack_len {
            return -1;
        }
        if haystack[i..window] == needle {
            return i as i32;
        }
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(str_str(String::from("sadbutsad"), String::from("sad")), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(str_str(String::from("abc"), String::from("c")), 2);
    }
}
