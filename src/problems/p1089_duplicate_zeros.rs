/* https://leetcode.com/problems/duplicate-zeros/description/

Given a fixed-length integer array arr, duplicate each occurrence of zero, shifting the remaining elements to the right.

Note that elements beyond the length of the original array are not written. Do the above modifications to the input array in place and do not return anything.

Example 1:
Input: arr = [1,0,2,3,0,4,5,0]
Output: [1,0,0,2,3,0,0,4]
Explanation: After calling your function, the input array is modified to: [1,0,0,2,3,0,0,4]

Example 2:
Input: arr = [1,2,3]
Output: [1,2,3]
Explanation: After calling your function, the input array is modified to: [1,2,3]

Constraints:
1 <= arr.length <= 10^4
0 <= arr[i] <= 9
*/

pub fn duplicate_zeros(arr: &mut Vec<i32>) {
    let len = arr.len();
    let mut i = 0;
    while i < len {
        if arr[i] == 0 {
            arr.insert(i + 1, 0);
            i += 2;
        } else {
            i += 1;
        }
    }

    arr.truncate(len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut arr = vec![1, 0, 2, 3, 0, 4, 5, 0];
        duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 0, 0, 2, 3, 0, 0, 4]);
    }

    #[test]
    fn example_2() {
        let mut arr = vec![1, 2, 3];
        duplicate_zeros(&mut arr);
        assert_eq!(arr, vec![1, 2, 3]);
    }
}
