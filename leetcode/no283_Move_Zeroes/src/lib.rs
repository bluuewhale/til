// 283. Move Zeroes
// https://leetcode.com/problems/move-zeroes/

use std::vec::Vec;

struct Solution {}
impl Solution {
    pub fn move_zeroes_1(nums: &mut Vec<i32>) {
        let mut j = 0;

        for i in 0..nums.len() {
            if nums[i] != 0 {
                let tmp: i32 = nums[j];
                nums[j] = nums[i];
                nums[i] = tmp;

                j += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input() -> Vec<i32> {
        vec![0, 1, 0, 3, 12]
    }
    fn output() -> Vec<i32> {
        vec![1, 3, 12, 0, 0]
    }

    #[test]
    fn test_1_1() {
        let mut input = input();
        let output = output();
        Solution::move_zeroes_1(&mut input);
        let match_cnt = input.iter().zip(&output).filter(|&(a, b)| a == b).count();
        assert_eq!(match_cnt, output.len());
    }
}
