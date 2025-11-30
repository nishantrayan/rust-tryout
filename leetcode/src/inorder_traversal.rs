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
use std::cell::RefCell;
use std::rc::Rc;

pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    if root.is_none() {
        vec![]
    } else {
        let node = root.unwrap();
        let mut result = inorder_traversal(node.borrow().left.clone());
        result.push(node.borrow().val);
        result.extend(inorder_traversal(node.borrow().right.clone()));
        result
    }

}
mod tests {
    use super::*;
    #[test]
    fn test_one_node() {
        let root = TreeNode::new(1);
        assert_eq!(
            inorder_traversal(Some(Rc::new(RefCell::new(root)))),
            vec![1]
        );
    }

    #[test]
    fn test_simple_tree() {
        let root = Rc::new(RefCell::new(TreeNode::new(1)));
        let left = TreeNode::new(2);
        let right = TreeNode::new(3);
        root.borrow_mut().left = Some(Rc::new(RefCell::new(left)));
        root.borrow_mut().right = Some(Rc::new(RefCell::new(right)));
        assert_eq!(
            inorder_traversal(Some(root)),
            vec![2, 1, 3]
        );
    }
}
