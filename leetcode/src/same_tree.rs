use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        return if let Some(p) = p {
            if let Some(q) = q {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            } else {
                false
            }
        } else {
            q.is_none()
        };
    }
}
mod tests {
    use super::*;

    #[test]
    fn test_same_tree_single_node() {
        let p = TreeNode::new(1);
        let q = TreeNode::new(1);
        assert_eq!(
            true,
            Solution::is_same_tree(
                Some(Rc::from(RefCell::from(p))),
                Some(Rc::from(RefCell::from(q)))
            )
        );
    }

    #[test]
    fn test_same_tree_no_nodes() {
        assert_eq!(true, Solution::is_same_tree(None, None));
    }

    fn create_tree(
        val: i32,
        left_val: Option<i32>,
        right_val: Option<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = RefCell::from(TreeNode::new(val));
        if let Some(left_val) = left_val {
            let left_node = TreeNode::new(left_val);
            node.borrow_mut().left = Some(Rc::from(RefCell::from(left_node)));
        }

        if let Some(right_val) = right_val {
            let right_node = TreeNode::new(right_val);
            node.borrow_mut().right = Some(Rc::from(RefCell::from(right_node)));
        }
        Some(Rc::from(RefCell::from(node)))
    }
    #[test]
    fn test_same_tree_simple_tree_match() {
        let p = create_tree(1, Some(2), Some(3));
        let q = create_tree(1, Some(2), Some(3));
        assert_eq!(true, Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_same_tree_simple_tree_no_match() {
        let p = create_tree(1, Some(2), None);
        let q = create_tree(1, None, Some(2));
        assert_eq!(false, Solution::is_same_tree(p, q));
    }

    #[test]
    fn test_same_tree_multi_level() {
        let p = create_tree(1, Some(2), Some(3));
        let node = create_tree(1, Some(2), None);
        let q = Some(Rc::from(RefCell::from(TreeNode {
            val: 3,
            left: node,
            right: None,
        })));
        assert_eq!(false, Solution::is_same_tree(p, q));
    }
}
