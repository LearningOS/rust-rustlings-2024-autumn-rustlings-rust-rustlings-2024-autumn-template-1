/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


/*
    single linked list merge
    This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T, // 节点的值
    next: Option<NonNull<Node<T>>>, // 指向下一个节点的指针
}

impl<T> Node<T> {
    // 创建新的节点
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32, // 链表的长度
    start: Option<NonNull<Node<T>>>, // 链表的起始节点
    end: Option<NonNull<Node<T>>>, // 链表的末尾节点
}

impl<T> Default for LinkedList<T> {
    // 默认实现，用于创建一个空链表
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    // 创建新的空链表
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 向链表中添加一个新元素
    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr, // 如果链表为空，将新节点设置为起始节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr }, // 将新节点添加到末尾
        }
        self.end = node_ptr; // 更新末尾节点指针
        self.length += 1; // 增加链表长度
    }

    // 获取指定索引的元素
    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    // 递归查找指定索引的节点
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }), // 找到节点，返回值
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1), // 递归查找下一个节点
            },
        }
    }

    // 合并两个有序链表
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    where
    T: PartialOrd + Clone, // 确保 T 实现了 PartialOrd 和 Clone 特征, // 确保 T 实现了 PartialOrd 特征，以支持比较操作
    {
        let mut merged_list = LinkedList::new(); // 新建合并链表
        // 初始化指针，指向两个输入链表的起始节点
        let mut current_a = list_a.start;
        let mut current_b = list_b.start;

        // 遍历两个链表，直到其中一个链表为空
    while current_a.is_some() && current_b.is_some() {
        // 获取当前节点的值，使用引用借用
        let value_a = unsafe { &(*current_a.unwrap().as_ptr()).val };
        let value_b = unsafe { &(*current_b.unwrap().as_ptr()).val };

        // 比较两个节点的值，将较小的值添加到合并链表
        if value_a <= value_b {
            merged_list.add(value_a.clone()); // 克隆 value_a 添加到合并链表
            current_a = unsafe { (*current_a.unwrap().as_ptr()).next }; // 移动到 list_a 的下一个节点
        } else {
            merged_list.add(value_b.clone()); // 克隆 value_b 添加到合并链表
            current_b = unsafe { (*current_b.unwrap().as_ptr()).next }; // 移动到 list_b 的下一个节点
        }
    }

    // 如果 list_a 还有剩余元素，将其添加到合并链表
    while current_a.is_some() {
        let value_a = unsafe { &(*current_a.unwrap().as_ptr()).val };
        merged_list.add(value_a.clone()); // 克隆 value_a 添加到合并链表
        current_a = unsafe { (*current_a.unwrap().as_ptr()).next }; // 移动到下一个节点
    }

    // 如果 list_b 还有剩余元素，将其添加到合并链表
    while current_b.is_some() {
        let value_b = unsafe { &(*current_b.unwrap().as_ptr()).val };
        merged_list.add(value_b.clone()); // 克隆 value_b 添加到合并链表
        current_b = unsafe { (*current_b.unwrap().as_ptr()).next }; // 移动到下一个节点
    }

    // 返回合并后的链表
    merged_list
}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    // 实现 Display trait，用于格式化输出链表
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }), // 输出链表的起始节点
            None => Ok(()), // 空链表
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    // 实现 Display trait，用于格式化输出节点
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }), // 输出当前节点值和下一个节点
            None => write!(f, "{}", self.val), // 最后一个节点
        }
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
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}