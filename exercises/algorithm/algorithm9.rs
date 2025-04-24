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
    count: usize,        // 元素数量
    items: Vec<T>,       // 实际存储
    comparator: fn(&T, &T) -> bool, // 比较函数, 用于决定是最大堆还是最小堆
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            // 初始化时用默认值占位, 所以我们需要的堆顶在 1 号位置
            items: vec![T::default()],
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
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;   // 新元素的索引

        // 上浮过程
        while idx > 1 {
            let parent_idx = self.parent_idx(idx);
            // 调用比较函数
            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                // 如果一直能比较, 那么最多比较到节点 1 比较就结束了
                idx = parent_idx;
            } else {
                // 如果以及找到正确位置了, 那么就停下来
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }
    // 检查给定节点是否有至少一个子节点, 对于上浮和下沉操作至关重要
    // 左子节点不存在那么一定不存在右子节点
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
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);

        if right > self.count {
            left
        } else {
            if (self.comparator)(&self.items[left], &self.items[right]) {
                left
            } else {
                right
            }
        }
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
        //TODO
        // 取出堆顶元素 (索引1的元素)
        // 此时堆没有顶部节点了, 所以将最后一个元素移到堆顶, 简单方便
        // 执行 "下沉"(sink) 操作, 恢复堆的性质
		
        if self.count == 0 {
            return None;
        }

        // 取出堆顶元素
        // 与 remove 不同, swap_remove 将取出指定元素并使用最后一个元素取代它, 而不是移动后续所有元素
        // 有更高的性能, 平均 O(1)
        let top = self.items.swap_remove(1);
        self.count -= 1;

        if self.count > 0 {
            // 下沉过程, 从堆顶开始
            let mut idx = 1;
            // 如果存在子节点
            while self.children_present(idx) {
                // 获取我们需要的最大或最小子节点
                let child_idx = self.smallest_child_idx(idx);
                // 如果符合要求就下沉
                if !(self.comparator)(&self.items[idx], &self.items[child_idx]) {
                    self.items.swap(idx, child_idx);
                    idx = child_idx;
                } else {
                    break;
                }
            }
        }
        Some(top)
        
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



// Top-K 算法
// /// 获取最小的 K 个元素
// pub fn top_k_min<T: Ord + Default>(nums: Vec<T>, k: usize) -> Vec<T> {
//     if k == 0 || nums.is_empty() {
//         return vec![];
//     }

//     let mut heap = MaxHeap::new(); // 最大堆保留最小的 K 个元素

//     for num in nums {
//         if heap.len() < k {
//             heap.add(num);
//         } else {
//             // 只要比堆顶小就踢掉堆顶自己成为新的堆顶. 然后重新堆化
//             if &num < heap.items.get(1).unwrap() {
//                 heap.next(); // 移除当前堆顶
//                 heap.add(num);
//             }
//         }
//     }

//     // 由于是最大堆, 所以最终结果将是降序
//     heap.collect()
// }

// #[test]
// fn test_top_k_min() {
//     let nums = vec![4, 1, 3, 12, 7, 5];
//     // 由于最大堆的行为会导致结果是反转的
//     assert_eq!(top_k_min(nums.clone(), 3), vec![4, 3, 1]);
//     assert_eq!(top_k_min(nums.clone(), 5), vec![7, 5, 4, 3, 1]);
//     assert_eq!(top_k_min(nums.clone(), 0), vec![]);
// }

// #[test]
// fn test_edge_cases() {
//     // 元素全部相同
//     assert_eq!(top_k_min(vec![2, 2, 2], 2), vec![2, 2]);
//     // K 大于数组长度
//     assert_eq!(top_k_min(vec![1, 2], 5), vec![2, 1]);
// }