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
use std::mem::replace;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
         if root1.is_none() && root2.is_none() {
            return true
         }
         if root1.is_none() && root2.is_some() || root1.is_some() && root2.is_none() {
            return false
         }
         
         fn dfs_flip(node1: Rc<RefCell<TreeNode>>, node2: Rc<RefCell<TreeNode>>) -> bool {
            let mut node1 = node1.borrow_mut();
            let mut node2 = node2.borrow_mut();
            
            if node1.val != node2.val {
                return false
            }

            let default = Rc::<RefCell<TreeNode>>::new(RefCell::<TreeNode>::new(TreeNode::new(-1)));
            if node1.left.as_ref().unwrap_or(&default).borrow().val == node2.right.as_ref().unwrap_or(&default).borrow().val 
               && node1.right.as_ref().unwrap_or(&default).borrow().val == node2.left.as_ref().unwrap_or(&default).borrow().val {
                let temp = node2.left.take();
                let temp = replace(&mut node2.right, temp);
                node2.left = temp;
            }

            let mut result: bool = true;
            if node1.left.is_some() && node2.left.is_some() {
                result = result && dfs_flip(node1.left.as_ref().unwrap().clone(), node2.left.as_ref().unwrap().clone());
            } else if node1.left.is_some() || node2.left.is_some() {
                return false
            }
            if node1.right.is_some() && node2.right.is_some() {
                result = result && dfs_flip(node1.right.as_ref().unwrap().clone(), node2.right.as_ref().unwrap().clone());
            } else if node1.right.is_some() || node2.right.is_some() {
                return false
            }

            result
         }

         dfs_flip(root1.unwrap(), root2.unwrap())
    }
}
