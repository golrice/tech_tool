// 给定两个整数数组
// preorder 和 inorder
// 其中 preorder 是二叉树的先序遍历
// inorder 是同一棵树的中序遍历
// 请构造二叉树并返回其根节点。
//
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
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution {
    index: HashMap<i32, usize>,
}

impl Solution {
    pub fn new() -> Self {
        Solution {
            index: HashMap::new(),
        }
    }

    pub fn build_tree(
        &mut self,
        preorder: &Vec<i32>,
        inorder: &Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let n = preorder.len();
        for (i, &val) in preorder.iter().enumerate() {
            self.index.insert(val, i);
        }
        self.help(preorder, inorder, 0, n - 1, 0, n - 1)
    }

    fn help(
        &mut self,
        pre: &Vec<i32>,
        ino: &Vec<i32>,
        pl: usize,
        pr: usize,
        il: usize,
        ir: usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if pl > pr {
            return None;
        }

        let root_index = pl;
        let root_val = pre[root_index];
        let in_root_index = *self.index.get(&root_val).unwrap();
        let left_size = in_root_index - pl;

        let root = Rc::new(RefCell::new(TreeNode::new(root_val)));
        root.borrow_mut().left = self.help(
            pre,
            ino,
            root_index + 1,
            root_index + left_size,
            il,
            in_root_index - 1,
        );
        root.borrow_mut().right =
            self.help(pre, ino, pl + left_size + 1, pr, in_root_index + 1, ir);

        Some(root)
    }
}
