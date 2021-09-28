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
impl Solution {
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        if root.is_none() {
            return 0;
        }

        fn dfs(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
            if root.is_none() {
                return 0;
            }
            let root = root.unwrap();
            let mut root = root.borrow_mut();
            let mut ret = 0;
            if root.val == target_sum {
                ret += 1;
            }
            ret += dfs(root.left.clone(), target_sum - root.val);
            ret += dfs(root.right.clone(), target_sum - root.val);
            ret
        }
        
        let mut ret = dfs(root.clone(), target_sum);
        let root_cell = root.clone().unwrap();
        let mut root_ref = root_cell.borrow_mut();
        ret += crate::Solution::path_sum(root_ref.left.clone(), target_sum);
        ret += crate::Solution::path_sum(root_ref.right.clone(), target_sum);
        ret
    }
}