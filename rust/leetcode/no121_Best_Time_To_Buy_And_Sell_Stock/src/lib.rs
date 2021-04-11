// 121. Best Time to Buy and Sell Stock
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

struct Solution {}
impl Solution {
    pub fn calc_max(prices: &[i32]) -> Option<&i32> {
        return prices.iter().max();
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len < 2 {
            return 0;
        }

        let mut max = Self::calc_max(&prices[1..]).unwrap();
        let mut max_profit = 0;

        for (i, v) in prices.iter().enumerate() {
            let mut i = i + 1;

            if v == max {
                if i == len {
                    i -= 1;
                }
                max = Self::calc_max(&prices[i..]).unwrap();
            }

            let profit = max - v;
            if profit > max_profit {
                max_profit = profit
            }
        }
        max_profit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn input1() -> Vec<i32> {
        vec![7, 1, 5, 3, 6, 4]
    }
    fn output1() -> i32 {
        5
    }

    fn input2() -> Vec<i32> {
        vec![7, 6, 4, 3, 1]
    }
    fn output2() -> i32 {
        0
    }

    #[test]
    fn test1() {
        assert_eq!(output1(), Solution::max_profit(input1()));
    }
    #[test]
    fn test2() {
        assert_eq!(output2(), Solution::max_profit(input2()));
    }
}
