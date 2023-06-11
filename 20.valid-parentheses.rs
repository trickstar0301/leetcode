/*
 * @lc app=leetcode id=20 lang=rust
 *
 * [20] Valid Parentheses
 *
 * https://leetcode.com/problems/valid-parentheses/description/
 *
 * algorithms
 * Easy (40.35%)
 * Likes:    20138
 * Dislikes: 1226
 * Total Accepted:    3.4M
 * Total Submissions: 8.4M
 * Testcase Example:  '"()"'
 *
 * Given a string s containing just the characters '(', ')', '{', '}', '[' and
 * ']', determine if the input string is valid.
 *
 * An input string is valid if:
 *
 *
 * Open brackets must be closed by the same type of brackets.
 * Open brackets must be closed in the correct order.
 * Every close bracket has a corresponding open bracket of the same type.
 *
 *
 *
 * Example 1:
 *
 *
 * Input: s = "()"
 * Output: true
 *
 *
 * Example 2:
 *
 *
 * Input: s = "()[]{}"
 * Output: true
 *
 *
 * Example 3:
 *
 *
 * Input: s = "(]"
 * Output: false
 *
 *
 *
 * Constraints:
 *
 *
 * 1 <= s.length <= 10^4
 * s consists of parentheses only '()[]{}'.
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    fn is_valid(s: String) -> bool {
        let bracket_pairs: HashMap<char, char> = [('(', ')'), ('{', '}'), ('[', ']'), ('<', '>')]
            .iter()
            .cloned()
            .collect();
    
        let mut stack: Vec<char> = Vec::new();
    
        for ch in s.chars() {
            if bracket_pairs.contains_key(&ch) {
                stack.push(ch);
            } else if bracket_pairs.values().any(|&value| value == ch) {
                if let Some(open) = stack.pop() {
                    if bracket_pairs[&open] != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false; // Invalid character
            }
        }
    
        stack.is_empty()
    }
    // pub fn is_valid(s: String) -> bool {
    //     let mut stack: Vec<char> = Vec::new();

    //     fn is_matching_pair(open: char, close: char) -> bool {
    //         match (open, close) {
    //             ('(', ')') | ('{', '}') | ('[', ']') => true,
    //             _ => false,
    //         }
    //     }

    //     for ch in s.chars() {
    //         match ch {
    //             '(' | '{' | '[' => stack.push(ch),
    //             ')' | '}' | ']' => {
    //                 if let Some(open) = stack.pop() {
    //                     if !is_matching_pair(open, ch) {
    //                         return false;
    //                     }
    //                 } else {
    //                     return false;
    //                 }
    //             }
    //             _ => return false, // Invalid character
    //         }
    //     }

    //     stack.is_empty()
    // }
}
// @lc code=end
