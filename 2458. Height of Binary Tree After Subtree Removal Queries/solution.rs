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
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn tree_queries(root: Option<Rc<RefCell<TreeNode>>>, queries: Vec<i32>) -> Vec<i32> {
        let mut map: HashMap<i32, (i32, usize, usize)> = HashMap::new();
        let mut idx_to_val: Vec<i32> = Vec::new();
        let mut result: Vec<i32> = Vec::new();

        fn dfs(node: &Rc<RefCell<TreeNode>>, height: i32, map: &mut HashMap<i32, (i32, usize, usize)>, idx_to_val: &mut Vec<i32>) {
            idx_to_val.push(node.borrow().val);
            let start: usize = idx_to_val.len() - 1;

            if let Some(left) = node.borrow().left.as_ref() { dfs(left, height + 1, map, idx_to_val) }
            if let Some(right) = node.borrow().right.as_ref() { dfs(right, height + 1, map, idx_to_val) }
            
            idx_to_val.push(node.borrow().val);
            let end: usize = idx_to_val.len() - 1;

            map.insert(node.borrow().val, (height, start, end));
        }

        dfs(root.as_ref().unwrap(), 0, &mut map, &mut idx_to_val);

        let mut max_left: Vec<i32> = vec![0; idx_to_val.len()];
        let mut max_right: Vec<i32> = vec![0; idx_to_val.len()];

        max_left[0] = map.get(&root.as_ref().unwrap().borrow().val).unwrap().0;
        max_right[idx_to_val.len() - 1] = max_left[0];

        for i in (1..idx_to_val.len()) {
            max_left[i] = max_left[i - 1].max(map.get(&idx_to_val[i]).unwrap().0);
        }

        for i in (0..idx_to_val.len() - 1).rev() {
            max_right[i] = max_right[i + 1].max(map.get(&idx_to_val[i]).unwrap().0);
        }

        for query in queries {
            let (height, first, last) = *map.get(&query).unwrap();
            let left = if first > 0 { max_left[first - 1] } else { 0 };
            let right = if last < idx_to_val.len() - 1 { max_right[last + 1] } else { 0 };
            result.push(left.max(right));
        }

        result
    }
}
