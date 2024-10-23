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
use std::cell::{RefCell, RefMut};
impl Solution {
    pub fn replace_value_in_tree(mut root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut map: Vec<i32> = Vec::new();
        let mut start = root.as_mut().unwrap();

        fn dfs(mut node: RefMut<TreeNode>, sibling_val: i32, level: usize, replace: bool, map: &mut Vec<i32>) {
            if replace {
                if level == 0 || level == 1 { node.val = 0 }
                else { node.val = map[level] - node.val - sibling_val }
            }
            else { if level < map.len() { map[level] += node.val } else { map.push(node.val) }}

            let has_left: bool = node.left.is_some();
            let has_right: bool = node.right.is_some();
            let left_val: i32 = if has_left { node.left.as_ref().unwrap().borrow().val } else { 0 };
            let right_val: i32 = if has_right { node.right.as_ref().unwrap().borrow().val } else { 0 };
            if has_left {
                let left: &mut Rc<RefCell<TreeNode>> = node.left.as_mut().unwrap();
                dfs(left.borrow_mut(), right_val, level + 1, replace, map)
            }
            if has_right {
                let right: &mut Rc<RefCell<TreeNode>> = node.right.as_mut().unwrap();
                dfs(right.borrow_mut(), left_val, level + 1, replace, map)
            }
        }

        dfs(start.borrow_mut(), 0, 0, false, &mut map);
        dfs(start.borrow_mut(), 0, 0, true, &mut map);
        
        drop(start);
        root
    }
}
