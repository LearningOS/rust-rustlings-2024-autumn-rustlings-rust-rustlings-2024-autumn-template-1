/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
//


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug, Default)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: std::cmp::PartialOrd + Clone> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let node = Box::new(Node::new(obj));
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });

        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }

        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => {
                if index == 0 {
                    Some(unsafe { &(*next_ptr.as_ptr()).val })
                } else {
                    self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1)
                }
            }
        }
    }

    pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self {
        let mut merged_list = LinkedList::new();
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        while current_a.is_some() || current_b.is_some() {
            if current_a.is_some() && current_b.is_some() {
                let node_a = unsafe { current_a.unwrap().as_ref() };
                let node_b = unsafe { current_b.unwrap().as_ref() };

                if node_a.val < node_b.val {
                    merged_list.add(node_a.val.clone());
                    current_a = node_a.next;
                } else {
                    merged_list.add(node_b.val.clone());
                    current_b = node_b.next;
                }
            } else if current_a.is_some() {
                let node_a = unsafe { current_a.unwrap().as_ref() };
                merged_list.add(node_a.val.clone());
                current_a = node_a.next;
            } else if current_b.is_some() {
                let node_b = unsafe { current_b.unwrap().as_ref() };
                merged_list.add(node_b.val.clone());
                current_b = node_b.next;
            }
        }

        merged_list
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut result = String::new();

        while let Some(node_ptr) = current {
            let node = unsafe { node_ptr.as_ref() };
            result.push_str(&format!("{}, ", node.val));
            current = node.next;
        }

        if result.len() > 2 {
            result.truncate(result.len() - 2);
        }

        write!(f, "{}", result)
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for &val in &vec_a {
            list_a.add(val);
        }
        for &val in &vec_b {
            list_b.add(val);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for (i, &target) in target_vec.iter().enumerate() {
            assert_eq!(target, *list_c.get(i as i32).unwrap());
        }
    }
}

