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
use std::cell::RefCell;
use std::mem::swap;
impl Solution {
    pub fn reverse_odd_levels(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>, level: usize) {
            if left.is_none() || right.is_none() {
                return
            }

            let left = left.unwrap();
            let right = right.unwrap();

            if level % 2 == 1 {
                let mut left_borrow = left.borrow_mut();
                let mut right_borrow = right.borrow_mut();
                swap(
                    &mut left_borrow.val,
                    &mut right_borrow.val
                )
            }
            dfs(left.borrow().left.clone(), right.borrow().right.clone(), level + 1);
            dfs(left.borrow().right.clone(), right.borrow().left.clone(), level + 1);
        }
        if let Some(node) = &root {
            dfs(node.borrow().left.clone(), node.borrow().right.clone(), 1);
        }
        root
    }
}
