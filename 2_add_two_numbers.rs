/*
 * @lc app=leetcode id=2 lang=rust
 *
 * [2] Add Two Numbers
 */

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut carry = 0;
        let mut head = ListNode::new(0);
        let mut current = &mut head;
        // move l1,l2 for modifying value
        let mut p = l1;
        let mut q = l2;
        while p.is_some() || q.is_some() {
            let sum = carry
                + p.take().map_or(0, |node| {
                    p = node.next;
                    node.val
                })
                + q.take().map_or(0, |node| {
                    q = node.next;
                    node.val
                });
            carry = sum / 10;
            current.next = Some(Box::new(ListNode::new(sum % 10)));
            current = current.next.as_mut().unwrap();
        }
        if carry > 0 {
          current.next = Some(Box::new(ListNode::new(carry)));
        }

        head.next
    }
}
// @lc code=end
