use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, PartialEq, Eq)]
pub struct Tree {
    root: Rc<RefCell<TreeNode>>
}

impl Tree {
    pub fn new(root: Rc<RefCell<TreeNode>>) -> Self {
        Tree {root}
    }

    pub fn get_extreme_node(&self) -> Rc<RefCell<TreeNode>>{
        let mut current_node = Some(self.root.clone());
        let current_node = {
            while let Some(node) = current_node {
                if node.borrow().left.is_some() {
                    current_node = node.borrow().left.clone();
                } else if node.borrow().right.is_some() {
                    current_node = node.borrow().right.clone();
                } else {
                    return node.clone();
                }
            }
            current_node
        };

        match current_node {
            Some(val) => val,
            None => self.root.clone()
        }
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub sum_hd: i32,
    pub state: String,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>
}


impl TreeNode {
    pub fn new(sum_hd: i32, state: String) -> Self {
        TreeNode {
            sum_hd,
            state,
            left: None,
            right: None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_extreme_node() {
        let tree = Tree::new(Rc::new(RefCell::new(TreeNode::new(10, String::from("10")))));

        {
            let mut root_node = tree.root.borrow_mut();
            root_node.left = Some(Rc::new(RefCell::new(TreeNode::new(2, String::from("001")))));

            if let Some(rc_root_left_node) = &root_node.left {
                let mut root_left_node = rc_root_left_node.borrow_mut();
                root_left_node.right = Some(Rc::new(RefCell::new(TreeNode::new(3, String::from("010")))));

                if let Some(rc_root_left_right_node) = &root_left_node.right {
                    let mut root_left_right_node = rc_root_left_right_node.borrow_mut();
                    root_left_right_node.left = Some(Rc::new(RefCell::new(TreeNode::new(4, String::from("011")))));
                }
            }
        }

        let extreme_node = tree.get_extreme_node();
        assert_eq!(
            extreme_node,
            Rc::new(RefCell::new(TreeNode::new(4, String::from("011"))))
        )
    }
}
