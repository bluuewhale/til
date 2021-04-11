struct Solution {}
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        let range: Vec<i32> = (1..(nums.len() + 1) as i32).collect();
        let range_set: HashSet<i32> = HashSet::from_iter(range);
        let nums_set: HashSet<i32> = HashSet::from_iter(nums);

        let mut diff: Vec<i32> = range_set.difference(&nums_set).map(|&x| x).collect();

        diff.sort();
        diff
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn input() -> Vec<i32> {
        vec![4, 3, 2, 7, 8, 2, 3, 1]
    }

    fn output() -> Vec<i32> {
        vec![5, 6]
    }
    #[test]
    fn it_works() {
        assert_eq!(Solution::find_disappeared_numbers(input()), output());
    }
}
