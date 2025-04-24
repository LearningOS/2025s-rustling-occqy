/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    // 初始化一个 队列
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
    // 将元素加入队尾
    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }
    // 从队头移除元素
    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            // 移除队头元素 (注意这是 O(n) 操作)
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }
    // 从 队列 获取一个元素
    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }
    // 获取当前 队列 的长度
    pub fn size(&self) -> usize {
        self.elements.len()
    }
    // 判断当前 队列 是否为空栈
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}
// 为队列 添加 默认实现
impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

// 基于双队列的栈
pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        //TODO
        // 压入元素时优先压入主队列
        // 哪个队列不是空的, 哪个队列就是主队列
        // 如果都是空的默认压入一号队列
        // pop 操作可以保证正常情况下两个队列至少有一个为空
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q1.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        //TODO
        if self.is_empty() {
            return Err("Stack is empty");
        }
        // 确定哪个队列是非空的 (主队列)
        let (full, empty) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else {
            (&mut self.q2, &mut self.q1)
        };

        while full.size() > 1 {
            if let Ok(vall) = full.dequeue() {
                empty.enqueue(vall);
            }
        }
        // 弹出并返回主队列的最后一个元素
        full.dequeue()
		
    }
    // 只有当两个队列都为空时栈为空
    pub fn is_empty(&self) -> bool {
		//TODO
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}



// #[derive(Debug)]
// pub struct Queue<T> {
//     in_stack: Vec<T>,   // 用于入队操作的栈
//     out_stack: Vec<T>,  // 用于出队操作的栈
// }

// impl<T> Queue<T> {
//     pub fn new() -> Self {
//         Queue {
//             in_stack: Vec::new(),
//             out_stack: Vec::new(),
//         }
//     }

//     /// 入队操作 - O(1) 时间复杂度
//     pub fn enqueue(&mut self, elem: T) {
//         self.in_stack.push(elem);
//     }

//     /// 出队操作 - 均摊 O(1) 时间复杂度
//     pub fn dequeue(&mut self) -> Option<T> {
//         // 如果out_stack为空，将in_stack的所有元素转移到out_stack
//         if self.out_stack.is_empty() {
//             while let Some(elem) = self.in_stack.pop() {
//                 self.out_stack.push(elem);
//             }
//         }
//         self.out_stack.pop()
//     }

//     /// 查看队首元素 - 均摊 O(1) 时间复杂度
//     pub fn peek(&mut self) -> Option<&T> {
//         // 同样需要先确保 out_stack 有元素
//         if self.out_stack.is_empty() {
//             while let Some(elem) = self.in_stack.pop() {
//                 self.out_stack.push(elem);
//             }
//         }
//         self.out_stack.last()
//     }

//     /// 返回队列大小 - O(1) 时间复杂度
//     pub fn size(&self) -> usize {
//         self.in_stack.len() + self.out_stack.len()
//     }

//     /// 检查队列是否为空 - O(1) 时间复杂度
//     pub fn is_empty(&self) -> bool {
//         self.in_stack.is_empty() && self.out_stack.is_empty()
//     }
// }
// #[test]
// fn test_stack_queue() {
//     let mut q = Queue::new();
//     assert_eq!(q.dequeue(), None);
//     assert!(q.is_empty());

//     q.enqueue(1);
//     q.enqueue(2);
//     q.enqueue(3);
//     assert_eq!(q.size(), 3);
//     assert_eq!(q.peek(), Some(&1));

//     assert_eq!(q.dequeue(), Some(1));
//     assert_eq!(q.dequeue(), Some(2));

//     q.enqueue(4);
//     q.enqueue(5);
//     assert_eq!(q.size(), 3);
//     assert_eq!(q.peek(), Some(&3));

//     assert_eq!(q.dequeue(), Some(3));
//     assert_eq!(q.dequeue(), Some(4));
//     assert_eq!(q.dequeue(), Some(5));
//     assert_eq!(q.dequeue(), None);
//     assert!(q.is_empty());
// }