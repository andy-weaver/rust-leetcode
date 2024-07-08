struct Solution {
    nums1: Vec<isize>,
    m: isize,
    nums2: Vec<isize>,
    n: isize,
}

// You are given two integer arrays nums1 and nums2, sorted in non-decreasing order, and
// two integers m and n, representing the number of elements in nums1 and nums2 respectively.

// Merge nums1 and nums2 into a single array sorted in non-decreasing order.

// The final sorted array should not be returned by the function, but instead be stored
// inside the array nums1. To accommodate this, nums1 has a length of m + n, where the first
// m elements denote the elements that should be merged, and the last n elements are set to 0
// and should be ignored. nums2 has a length of n.

// Constraints:
fn validate_constraints(nums1: &[isize], m: isize, nums2: &[isize], n: isize) {
    assert!(validate_constraints_1(nums1, m, n));
    assert!(validate_constraints_2(nums2, n));
    assert!(validate_constraints_3(m, n));
    assert!(validate_constraints_4(m, n));
    assert!(validate_constraints_5(nums1, nums2));
}

//     nums1.length == m + n
fn validate_constraints_1(nums1: &[isize], m: isize, n: isize) -> bool {
    nums1.len() == (m + n) as usize
}

//     nums2.length == n
fn validate_constraints_2(nums2: &[isize], n: isize) -> bool {
    nums2.len() == n as usize
}

//     0 <= m, n <= 200
fn validate_constraints_3(m: isize, n: isize) -> bool {
    let lower_bound: isize = 0;
    let upper_bound: isize = 200;

    m >= lower_bound && m <= upper_bound && n >= lower_bound && upper_bound <= 200
}

//     1 <= m + n <= 200
fn validate_constraints_4(m: isize, n: isize) -> bool {
    let lower_bound: isize = 1;
    let upper_bound: isize = 200;
    (m + n) >= lower_bound && (m + n) <= upper_bound
}

//     -109 <= nums1[i], nums2[j] <= 109
fn validate_constraints_5(nums1: &[isize], nums2: &[isize]) -> bool {
    let bound: isize = 10_isize.pow(9);
    println!("bound: {:?}", bound);
    nums1.iter().all(|x: &isize| -x <= bound && x <= &bound)
        && nums2.iter().all(|y: &isize| -y <= bound && y <= &bound)
}

fn merge<'a>(
    nums1: &'a mut [isize],
    nums2: &'a [isize],
    m: isize,
    n: isize,
) -> Result<&'a mut [isize], &'a str> {
    validate_constraints(nums1, m, nums2, n);
    nums1
        .iter_mut()
        .skip(m as usize)
        .zip(nums2.iter())
        .for_each(|(x, y)| *x = *y);
    nums1.sort();
    Ok(nums1) // Return the sorted array
}

impl Solution {
    fn new(nums1: Vec<isize>, m: isize, nums2: Vec<isize>, n: isize) -> Self {
        Self { nums1, m, nums2, n }
    }

    fn merge(nums1: &[isize], m: isize, nums2: &[isize], n: isize) {
        validate_constraints(nums1, m, nums2, n);
        nums1
            .iter()
            .skip(m as usize)
            .zip(nums2.iter())
            .for_each(|(x, y)| *x = *y);
        nums1.sort();
    }
}

// Turn the examples into tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sorted_array_example_1() {
        // Example 1:
        // Input: nums1 = [1,2,3,0,0,0], m = 3, nums2 = [2,5,6], n = 3
        let nums1: &[isize] = &mut [1, 2, 3, 0, 0, 0];
        let nums2: &[isize] = &[2, 5, 6];
        let m: isize = 3;
        let n: isize = 3;

        // Output: [1,2,2,3,5,6]
        let expected: &[isize] = &[1, 2, 2, 3, 5, 6];

        // Explanation: The arrays we are merging are [1,2,3] and [2,5,6].
        // The result of the merge is [1,2,2,3,5,6] with the underlined elements coming from nums1.
        let mut solution: Solution = Solution::new(nums1.to_vec(), m, nums2.to_vec(), n);
        assert_eq!(solution.merge().unwrap(), expected);
    }

    #[test]
    fn test_merge_sorted_array_example_2() {
        // Example 2:

        // Input: nums1 = [1], m = 1, nums2 = [], n = 0
        let nums1: &[isize] = &[1];
        let nums2: &[isize] = &[];
        let m: isize = 1;
        let n: isize = 0;

        // Output: [1]
        let expected: &mut [isize] = &mut [1];

        let mut solution = Solution::new(nums1.to_vec(), m, nums2.to_vec(), n);

        assert_eq!(solution.merge().unwrap(), expected);
        // Explanation: The arrays we are merging are [1] and [].
        // The result of the merge is [1].
    }

    #[test]
    fn test_merge_sorted_array_example_3() {
        // Example 3:
        // Input: nums1 = [0], m = 0, nums2 = [1], n = 1
        let nums1: &[isize] = &[0];
        let nums2: &[isize] = &[1];
        let m: isize = 0;
        let n: isize = 1;

        // Output: [1]
        let expected: &[isize] = &[1];

        let mut solution = Solution::new(nums1.to_vec(), m, nums2.to_vec(), n);

        assert_eq!(solution.merge().unwrap(), expected);
        // Explanation: The arrays we are merging are [] and [1].
        // The result of the merge is [1].
        // Note that because m = 0, there are no elements in nums1. The 0 is only there to ensure the merge result can fit in nums1.
    }

    #[test]
    fn test_returns_non_decreasing() {
        let mut nums1: &mut [isize] = &mut [4, 3, 1, 0, 0, 0];
        let nums2: &[isize] = &[2, 5, 6];
        let m: isize = 3;
        let n: isize = 3;

        nums1 = merge(nums1, nums2, m, n).expect("Non-decreasing test failed!");
        let expected: &[isize] = &[1, 2, 3, 4, 5, 6];

        assert_eq!(nums1, expected);
    }
}
