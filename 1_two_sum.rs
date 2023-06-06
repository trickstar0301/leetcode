/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

pub struct Solution {}

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashMap<i32, usize> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&j) = complements.get(&complement) {
                return vec![j as i32, i as i32];
            }
            complements.insert(num, i);
        }

        vec![]
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let expected = vec![0, 1];
        assert_eq!(two_sum(&nums, target), expected);
    }
}
