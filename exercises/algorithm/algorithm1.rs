/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/


use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
// 节点结构体（Node）
struct Node<T> {
    val: T,
     // 指向下一个节点的指针
    next: Option<NonNull<Node<T>>>,
}

// Node结构体关联函数
impl<T> Node<T> {
    // 创建新节点
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
// 链表结构体（LinkedList）
struct LinkedList<T> {
    length: u32,
    // 指向下一个节点的指针
    start: Option<NonNull<Node<T>>>,
    // 指向链表最后一个节点的指针
    end: Option<NonNull<Node<T>>>,
}

// 为 LinkedList 实现 Default 特征
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

// 链表结构体关联函数
impl<T> LinkedList<T> {
    // 创建新链表
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    // 向链表添加元素
    pub fn add(&mut self, obj: T) {
        // 创建一个新的节点, 并用 Box 分配堆内存
        let mut node = Box::new(Node::new(obj));
        // 确保新节点的 next 指针为 None
        node.next = None;
        // 将 Box 转换为原始指针, 再包装成 NonNull 指针
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
         // 检查链表是否为空
        match self.end {
            // 如果链表为空, 新节点就是首节点
            None => self.start = node_ptr,
            // 否则将当前尾节点的 next 指向新节点
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        // 更新尾指针为新节点
        self.end = node_ptr;
        // 增加链表长度
        self.length += 1;
    }

    // 实现获取链表元素  // 公开方法, 调用私有递归方法
    pub fn get(&mut self, index: i32) -> Option<&T> {
        // 添加边界检查，返回 None 来表示索引越界
        if index < 0 || index >= self.length as i32 {
            return None;
        }
        self.get_ith_node(self.start, index)
    }

    // 递归查找第 index 个节点
    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
         // 检查当前节点指针
        match node {
            // 如果节点不存在返回 None
            None => None,
            // 如果节点存在
            Some(next_ptr) => match index {
                // 看看是否是第 index 个节点
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                // 否则继续递归查找下一个节点
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	// pub fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> Self
	// {
    //     let mut merged = LinkedList::new();
    //     while let Some(a_node) = list_a.start.take() {
    //         let a_val = unsafe { a_node.as_ref().val };
    //         while let Some(b_node) = list_b.start.take() {
    //             let b_val = unsafe { b_node.as_ref().val };
    //             if a_val <= b_val {
    //                 merged.add(a_val.clone());
    //                 list_a.start = Some(a_node);
    //                 break;
    //             } else {
    //                 merged.add(b_val.clone());
    //                 list_b.start = Some(b_node);
    //             }
    //         }
    //         if list_b.start.is_none() {
    //             merged.add(a_val.clone());
    //         }
    //     }
    //     while let Some(b_node) = list_b.start.take() {
    //         let b_val = unsafe { b_node.as_ref().val };
    //         merged.add(b_val.clone());
    //     }

    //     merged

	// 	// //TODO
	// 	// Self {
    //     //     length: 0,
    //     //     start: None,
    //     //     end: None,
    //     // }
	// }

    // 实现链表合并
    pub fn merge(list_a: LinkedList<T>, list_b: LinkedList<T>) -> Self
    // 合并要求我们的数据是可比较的
    where
        T: Ord,
    {
        // Collect elements from both lists
        // 收集两个链表的元素到向量中
        let mut a_elements = Vec::new();
        let mut current = list_a.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                a_elements.push(node.val);
                current = node.next;
            }
        }

        let mut b_elements = Vec::new();
        let mut current = list_b.start;
        while let Some(node_ptr) = current {
            unsafe {
                let node = Box::from_raw(node_ptr.as_ptr());
                b_elements.push(node.val);
                current = node.next;
            }
        }

        // Merge the two sorted vectors
        // 合并两个有序向量
        let mut merged = Vec::new();
        let mut a_iter = a_elements.into_iter();
        let mut b_iter = b_elements.into_iter();
        let mut a_peek = a_iter.next();
        let mut b_peek = b_iter.next();

        loop {
            match (a_peek.as_ref(), b_peek.as_ref()) {
                (Some(a_val), Some(b_val)) => {
                    if a_val <= b_val {
                        merged.push(a_peek.take().unwrap());
                        a_peek = a_iter.next();
                    } else {
                        merged.push(b_peek.take().unwrap());
                        b_peek = b_iter.next();
                    }
                }
                (Some(a_val), None) => {
                    merged.push(a_peek.take().unwrap());
                    merged.extend(a_iter);
                    break;
                }
                (None, Some(b_val)) => {
                    merged.push(b_peek.take().unwrap());
                    merged.extend(b_iter);
                    break;
                }
                (None, None) => break,
            }
        }

        // Create the merged linked list
        // 创建合并后的链表
        let mut merged_list = LinkedList::new();
        for val in merged {
            merged_list.add(val);
        }

        merged_list
    }
    
}

// 实现格式化输出（LinkedList）
impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

// 实现格式化输出（Node）
impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

// 测试用例
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