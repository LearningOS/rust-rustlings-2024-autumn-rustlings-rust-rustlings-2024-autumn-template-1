/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 使用一个默认值的向量
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1;
        if self.count < self.items.len() {
            self.items[self.count] = value; // 直接添加
        } else {
            self.items.push(value); // 如果没有空间则推入新元素
        }
        self.bubble_up(self.count); // 上浮
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx) // 只有左子节点
        } else {
            if (self.comparator)(&self.items[self.left_child_idx(idx)], &self.items[self.right_child_idx(idx)]) {
                self.left_child_idx(idx) // 左子节点更小
            } else {
                self.right_child_idx(idx) // 右子节点更小
            }
        }
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut current_idx = idx;
        while current_idx > 1 {
            let parent_idx = self.parent_idx(current_idx); // 先计算父节点索引
            if (self.comparator)(&self.items[current_idx], &self.items[parent_idx]) {
                self.items.swap(current_idx, parent_idx);
                current_idx = parent_idx;
            } else {
                break; // 如果当前节点不小于父节点，停止上浮
            }
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut current_idx = idx;
        while self.children_present(current_idx) {
            let smallest_child = self.smallest_child_idx(current_idx);
            if (self.comparator)(&self.items[smallest_child], &self.items[current_idx]) {
                self.items.swap(current_idx, smallest_child);
                current_idx = smallest_child;
            } else {
                break; // 当前节点已符合堆性质
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 确保 T 实现 Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let top_value = self.items[1].clone(); // 克隆堆顶
        self.items[1] = self.items[self.count].clone(); // 将最后一个元素放到堆顶
        self.count -= 1; // 减少元素数量
        self.bubble_down(1); // 下沉
        Some(top_value)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // 确保 T 实现 Clone
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + Clone, // 确保 T 实现 Clone
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
