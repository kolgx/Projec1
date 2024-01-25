
pub mod binary_tree_demo_model {

    mod binary_tree_node {
        use std::rc::Rc;
        use std::cell::RefCell;
        use std::fmt::Display;
        use std::fmt::Formatter;
        use std::fmt::Result;

        #[derive(Debug)]
        pub struct BinaryTreeNode<T> {
            pub value: T,
            pub left: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
            pub right: Option<Rc<RefCell<BinaryTreeNode<T>>>>,
        }

        impl<T> BinaryTreeNode<T> {
            pub fn new(value: T) -> BinaryTreeNode<T> {
                BinaryTreeNode {
                    value,
                    left: None,
                    right: None,
                }
            }
        }

        impl<T: Display> Display for BinaryTreeNode<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{}", self.value)
            }
        }
    }

}
