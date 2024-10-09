/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) {
        //TODO
        match self.root {
            Some(ref mut node) => {
                (*node).insert(value);
            },
            None => {
                self.root = Some(Box::new(TreeNode::new(value)));
            },
            
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        fn search_node<T:Ord>(node :&Option<Box<TreeNode<T>>>,value: T) -> bool{
            match node {
                Some(ref node) =>{
                    if node.value == value {
                        return true
                    }else if node.value > value {
                        return search_node(&node.left,value)
                    }else {
                        return search_node(&node.right,value)
                    }
                },
                None => return false,
            }
        }
        search_node(&self.root,value)
    }
}

impl<T> TreeNode<T>
where
    T: Ord,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if value < self.value {
            match self.left {
                Some(ref mut left_node) => left_node.insert(value),
                None => self.left = Some(Box::new(TreeNode::new(value))),
            }
        } else if value > self.value {
            match self.right {
                Some(ref mut right_node) => right_node.insert(value),
                None => self.right = Some(Box::new(TreeNode::new(value))),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


