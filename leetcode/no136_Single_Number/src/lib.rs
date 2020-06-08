// https://leetcode.com/problems/single-number/
struct Solution1 {}
impl Solution1 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        *(nums
            .iter()
            .filter(|&n1| 1 == nums.iter().filter(|&n2| *n2 == *n1).count())
            .next()
            .unwrap())
    }
}
use std::collections::HashMap;
struct Solution2;
impl Solution2 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut hash = HashMap::new();

        for n in nums.iter() {
            if hash.contains_key(n) {
                hash.remove(&n);
            } else {
                hash.insert(n, n);
            }
        }
        **hash.keys().next().unwrap()
    }
}

use itertools::Itertools;
use std::collections::HashSet;
struct Solution3;
impl Solution3 {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let unique_nums: Vec<&i32> = nums.iter().clone().unique().collect();

        let sum: i32 = nums.iter().sum();
        let unique_nums_sum = unique_nums.iter().fold(0, |mut sum, &x| {
            sum += x;
            sum
        });

        return (2 * unique_nums_sum) - sum;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solution1_1() {
        let nums = vec![2, 2, 1];
        assert_eq!(1, Solution1::single_number(nums));
    }
    #[test]
    fn test_solution1_2() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(4, Solution1::single_number(nums));
    }
    #[test]
    fn test_solution2_1() {
        let nums = vec![2, 2, 1];
        assert_eq!(1, Solution2::single_number(nums));
    }
    #[test]
    fn test_solution2_2() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(4, Solution2::single_number(nums));
    }
    #[test]
    fn test_solution3_1() {
        let nums = vec![2, 2, 1];
        assert_eq!(1, Solution3::single_number(nums));
    }
    #[test]
    fn test_solution3_2() {
        let nums = vec![4, 1, 2, 1, 2];
        assert_eq!(4, Solution3::single_number(nums));
    }
}
