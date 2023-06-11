/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 *
 * https://leetcode.com/problems/longest-common-prefix/description/
 *
 * algorithms
 * Easy (40.97%)
 * Likes:    14134
 * Dislikes: 3984
 * Total Accepted:    2.4M
 * Total Submissions: 5.9M
 * Testcase Example:  '["flower","flow","flight"]'
 *
 * Write a function to find the longest common prefix string amongst an array
 * of strings.
 *
 * If there is no common prefix, return an empty string "".
 *
 *
 * Example 1:
 *
 *
 * Input: strs = ["flower","flow","flight"]
 * Output: "fl"
 *
 *
 * Example 2:
 *
 *
 * Input: strs = ["dog","racecar","car"]
 * Output: ""
 * Explanation: There is no common prefix among the input strings.
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= strs.length <= 200
 * 0 <= strs[i].length <= 200
 * strs[i] consists of only lowercase English letters.
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        let mut ans = String::new();
        if strs.is_empty() {
            return ans;
        }
        
        for j in 0..strs.len() {
            let mut prefix = strs[j].clone();

            for i in 0..strs.len() {
                let string = &strs[i];

                while !string.starts_with(&prefix) {
                    prefix.pop();

                    if prefix.is_empty() {
                        return String::new();
                    }
                }
            }
            if prefix.len() > ans.len(){
                ans = prefix;
            }
        }
        ans
    }
}
// @lc code=end
