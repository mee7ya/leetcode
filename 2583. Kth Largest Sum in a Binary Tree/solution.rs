// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
// 
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::VecDeque;
impl Solution {
    pub fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
        let mut acc: Vec<i64> = Vec::new();
        let mut queue: VecDeque<(Rc<RefCell<TreeNode>>, usize)> = VecDeque::new();
        queue.push_back((root.as_ref().unwrap().clone(), 0));
        
        while !queue.is_empty() {
            let (node_cell, level): (Rc<RefCell<TreeNode>>, usize) = queue.pop_front().unwrap();
            let node: Ref<TreeNode> = node_cell.borrow();
            if acc.len() <= level { acc.push(node.val as i64) } else { acc[level] += node.val as i64 }
            if let Some(left) = node.left.as_ref() { queue.push_back((left.clone(), level + 1)) }
            if let Some(right) = node.right.as_ref() { queue.push_back((right.clone(), level + 1)) }
        }

        acc.sort_unstable_by(|a, b| b.cmp(&a));
        if ((k - 1) as usize) < acc.len() { acc[(k-1) as usize] } else { -1 }
    }
}
