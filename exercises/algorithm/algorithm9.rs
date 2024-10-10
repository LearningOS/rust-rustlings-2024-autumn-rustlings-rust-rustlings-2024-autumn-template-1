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
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {    //返回堆的长度
        self.count
    }

    pub fn is_empty(&self) -> bool {    //堆是否为空
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {   //添加元素
        fn sift_up<T>(heap: &mut Heap<T>, idx: usize) where T: Default {
            if idx == 1{
                return;
            }
            let parent_idx = heap.parent_idx(idx);
            if let Some(value_idx )= heap.items.get(idx){
                if let Some(value_parent_idx) = heap.items.get(parent_idx){
                    if ((heap.comparator))(value_idx, value_parent_idx){
                        heap.items.swap(idx, parent_idx);
                        sift_up(heap, parent_idx);
                    }
                }
            }
        }
        //TODO
        self.count += 1;
        self.items.push(value);
        sift_up(self, self.count);
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
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        fn sift_down<T>(heap: &mut Heap<T>, idx: usize) where T : Default {
            let left_child_idx = heap.left_child_idx(idx);
            let right_child_idx = heap.right_child_idx(idx);
            if let Some(left_value) = heap.items.get(left_child_idx){
                if let Some(value) = heap.items.get(idx){
                    if let Some(right_value) = heap.items.get(right_child_idx){
                        if (heap.comparator)(left_value, right_value){
                            if (heap.comparator)(left_value, value){
                                heap.items.swap(idx, left_child_idx);
                                sift_down(heap, left_child_idx);
                            }
                        }else{
                            if (heap.comparator)(right_value, value){
                                heap.items.swap(idx, right_child_idx);
                                sift_down(heap, right_child_idx);
                            }
                        }
                    }else {
                        if (heap.comparator)(left_value, value){
                            heap.items.swap(idx, left_child_idx);
                            sift_down(heap, left_child_idx);
                            
                        }
                    }
                }
            }
            else {
                return;
            }
        }


        if self.is_empty() {
            return None;
        }
        self.items.swap(1, self.count);
        self.count -= 1;
        if let Some(value) = self.items.pop() {
            sift_down(self,1);
            return Some(value);
        }
        None
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
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