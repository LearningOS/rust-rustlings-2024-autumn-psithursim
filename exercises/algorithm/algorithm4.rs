/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/

use std::cmp::Ordering;
use std::fmt::Debug;

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
        self.root = Self::insert_value(self.root.take(), value);
    }

    // Helper function to insert a value into the tree
    fn insert_value(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(TreeNode::new(value))),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        node.left = Self::insert_value(node.left.take(), value);
                    }
                    Ordering::Greater => {
                        node.right = Self::insert_value(node.right.take(), value);
                    }
                    Ordering::Equal => (), // Ignore duplicates
                }
                Some(node)
            }
        }
    }

    // Search for a value in the BST
    fn search(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        Self::search_value(self.root.as_ref(), value)
    }

    // Helper function to search for a value in the tree
    fn search_value(node: Option<&Box<TreeNode<T>>>, value: &T) -> bool
    where
        T: PartialEq,
    {
        match node {
            None => false,
            Some(node) => match value.cmp(&node.value) {
                Ordering::Less => Self::search_value(node.left.as_ref(), value),
                Ordering::Greater => Self::search_value(node.right.as_ref(), value),
                Ordering::Equal => true,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(&1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(&5), true);
        assert_eq!(bst.search(&3), true);
        assert_eq!(bst.search(&7), true);
        assert_eq!(bst.search(&2), true);
        assert_eq!(bst.search(&4), true);

        
        assert_eq!(bst.search(&1), false);
        assert_eq!(bst.search(&6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(&1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}