// https://leetcode.com/problems/sqrtx/description/
// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
// You must not use any built-in exponent function or operator.
// For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.

// Example 1:
// Input: x = 4
// Output: 2
// Explanation: The square root of 4 is 2, so we return 2.

// Example 2:
// Input: x = 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.

// Constraints:
// 0 <= x <= 2^31 - 1

pub fn my_sqrt(x: i32) -> i32 {
    if x < 2 {
        return x;
    }

    let mut low = 1i64;
    let mut high = (x / 2) as i64;

    while low <= high {
        let mid = (low + high) / 2;
        let square = mid * mid;

        if square == x as i64 {
            return mid as i32;
        }
        if square < x as i64 {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    return high as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(my_sqrt(4), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(my_sqrt(8), 2);
    }

    #[test]
    fn example_3() {
        assert_eq!(my_sqrt(1024), 32);
    }
}
