use std::fmt::Debug;
use std::ptr::NonNull;

type UsizeLink = Option<NonNull<UsizeNode>>;

#[derive(Debug)]
struct UsizeNode
{
    elem: usize,
    left: UsizeLink,
    right: UsizeLink,
}

pub struct UsizeTree
{
    root: UsizeLink,
    len: usize,
}

impl UsizeTree
{
    // create a new empty tree
    pub fn new() -> Self { Self { root: None, len: 0 } }

    // returns the number of elements in the tree
    pub fn len(&self) -> usize { self.len }

    // returns whether tree is empty or not
    pub fn is_empty(&self) -> bool { self.len == 0 }

    fn find_link_mut(&mut self, elem: usize) -> &mut UsizeLink
    {
        unsafe {
            let mut cur_node = &mut self.root;
            while let Some(cur) = cur_node
            {
                let parent = &mut (*cur.as_ptr());
                if elem == parent.elem
                {
                    return cur_node;
                }

                cur_node = if elem < parent.elem { &mut parent.left } else { &mut parent.right };
            }
            cur_node
        }
    }

    fn find_link(&self, elem: usize) -> &UsizeLink
    {
        unsafe {
            let mut cur_node = &self.root;
            while let Some(cur) = cur_node
            {
                let parent = &mut (*cur.as_ptr());
                if elem == parent.elem
                {
                    return cur_node;
                }

                cur_node = if elem < parent.elem { &mut parent.left } else { &mut parent.right };
            }
            cur_node
        }
    }

    // insert an element into the tree
    // return if insertion was successful
    // insertion fails if the value is already in the tree
    pub fn insert(&mut self, elem: usize) -> bool
    {
        unsafe {
            let insert_link = self.find_link_mut(elem);
            if insert_link.is_some()
            {
                // don't insert duplicates
                return false;
            }

            let new_node = UsizeNode { elem, left: None, right: None };
            let new = Some(NonNull::new_unchecked(Box::into_raw(Box::new(new_node))));
            *insert_link = new;
            self.len += 1;
            true
        }
    }

    // search the tree for a value equal to passed elem
    // returns whether equal value was found or not
    pub fn contains(&self, elem: &usize) -> bool { self.find_link(*elem).is_some() }

    // remove an element from the tree
    // return if removal was successful
    // removal fails if the value isn't in the tree
    pub fn remove(&mut self, elem: usize) -> bool
    {
        unsafe {
            let remove_link = self.find_link_mut(elem);

            if remove_link.is_some()
            {
                let remove_node = &mut (*remove_link.unwrap().as_ptr());

                if remove_node.left.is_none()
                {
                    let old = Box::from_raw(remove_link.unwrap().as_ptr());
                    *remove_link = remove_node.right.take();
                    drop(old);
                }
                else if remove_node.right.is_none()
                {
                    let old = Box::from_raw(remove_link.unwrap().as_ptr());
                    *remove_link = remove_node.left.take();
                    drop(old);
                }
                else
                {
                    let mut successor_link = &mut remove_node.right;
                    while (*successor_link.unwrap().as_ptr()).left.is_some()
                    {
                        successor_link = &mut (*successor_link.unwrap().as_ptr()).left;
                    }

                    remove_node.elem = (*successor_link.unwrap().as_ptr()).elem;

                    let successor_node = &mut (*successor_link.unwrap().as_ptr());
                    let old = Box::from_raw(successor_link.unwrap().as_ptr());
                    *successor_link = successor_node.right.take();
                    drop(old);
                }
                self.len -= 1;
                return true;
            }

            false
        }
    }

    // clear the tree
    // postcondition: tree is empty
    pub fn clear(&mut self)
    {
        unsafe {
            while let Some(root_node) = self.root
            {
                let root_elem = (*root_node.as_ptr()).elem;
                self.remove(root_elem);
            }
        }
    }
}

impl Drop for UsizeTree
{
    fn drop(&mut self) { self.clear(); }
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_new_len()
    {
        let tree = UsizeTree::new();
        assert_eq!(tree.len(), 0);
    }

    #[test]
    fn test_new_is_empty()
    {
        let tree = UsizeTree::new();
        assert_eq!(tree.is_empty(), true);
    }

    #[test]
    fn test_insert_single_basic()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);

        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_insert_multiple_basic()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_insert_multiple_with_collisions()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.len(), 1);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.len(), 2);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.len(), 3);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.len(), 4);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.len(), 5);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.len(), 6);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);

        assert_eq!(tree.insert(3), false);
        assert_eq!(tree.len(), 7);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn test_contains()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.contains(&1), false);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.contains(&17), false);

        assert_eq!(tree.insert(3), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.contains(&17), false);
    }

    #[test]
    fn test_remove()
    {
        let mut tree = UsizeTree::new();

        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(2), false);
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(7), true);
        assert_eq!(tree.insert(3), false);

        assert_eq!(tree.remove(17), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 7);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(5), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.contains(&5), true);
        assert_eq!(tree.len(), 7);

        assert_eq!(tree.remove(5), true);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&2), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 6);

        assert_eq!(tree.remove(2), true);
        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&1), true);
        assert_eq!(tree.contains(&3), true);
        assert_eq!(tree.contains(&4), true);
        assert_eq!(tree.contains(&6), true);
        assert_eq!(tree.contains(&7), true);
        assert_eq!(tree.len(), 5);

        assert_eq!(tree.remove(1), true);
        assert_eq!(tree.remove(3), true);
        assert_eq!(tree.remove(4), true);
        assert_eq!(tree.remove(6), true);
        assert_eq!(tree.remove(7), true);
        assert_eq!(tree.contains(&1), false);
        assert_eq!(tree.contains(&2), false);
        assert_eq!(tree.contains(&3), false);
        assert_eq!(tree.contains(&4), false);
        assert_eq!(tree.contains(&5), false);
        assert_eq!(tree.contains(&6), false);
        assert_eq!(tree.contains(&7), false);
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);

        assert_eq!(tree.remove(17), false);
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);
    }

    #[test]
    fn test_clear()
    {
        let mut tree = UsizeTree::new();
        assert_eq!(tree.insert(1), true);
        assert_eq!(tree.insert(2), true);
        assert_eq!(tree.insert(3), true);
        assert_eq!(tree.insert(4), true);
        assert_eq!(tree.insert(5), true);
        assert_eq!(tree.insert(6), true);
        assert_eq!(tree.insert(7), true);
        tree.clear();
        assert_eq!(tree.len(), 0);
        assert_eq!(tree.is_empty(), true);
    }
}
