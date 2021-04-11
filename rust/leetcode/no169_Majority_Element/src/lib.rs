// 169.Majority Element
// https://leetcode.com/problems/majority-element/

use std::collections::HashMap;
struct Solution {}
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut result: i32 = 0;
        let majority: i32 = ((nums.len() as f64) / 2.0).ceil() as i32;
        //
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (i, v) in nums.iter().enumerate() {
            let cnt = map.entry(*v).or_insert(0);
            *cnt += 1;

            if *cnt >= majority {
                result = *v;
                break;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input1() -> Vec<i32> {
        vec![3, 2, 3]
    }

    fn output1() -> i32 {
        3
    }

    fn input2() -> Vec<i32> {
        vec![2, 2, 1, 1, 1, 2, 2]
    }
    fn output2() -> i32 {
        2
    }
    #[test]
    fn test_1() {
        assert_eq!(Solution::majority_element(input1()), output1());
        assert_eq!(Solution::majority_element(input2()), output2());
    }
}
