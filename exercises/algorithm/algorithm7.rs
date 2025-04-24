/*
	stack
	This question requires you to use a stack to achieve a bracket match
*/


#[derive(Debug)]
struct Stack<T> {
	size: usize,
	data: Vec<T>,
}
impl<T> Stack<T> {
	// 初始化一个栈
	fn new() -> Self {
		Self {
			size: 0,
			data: Vec::new(),
		}
	}
	// 判断当前栈是否为空栈
	fn is_empty(&self) -> bool {
		0 == self.size
	}
	// 获取当前栈的长度
	fn len(&self) -> usize {
		self.size
	}
	// 清空当前栈
	fn clear(&mut self) {
		self.size = 0;
		self.data.clear();
	}
	// 向栈压入一个元素
	fn push(&mut self, val: T) {
		self.data.push(val);
		self.size += 1;
	}
	// 由栈弹出一个元素，注意弹出时栈可能为空
	fn pop(&mut self) -> Option<T> {
		// Todo
		if self.size == 0 {
			None
		} else {
			self.size -= 1;
			self.data.pop()
		}
	}
	// 获取一个不可变元素，需先判断是否为空栈
	fn peek(&self) -> Option<&T> {
		if 0 == self.size {
			return None;
		}
		self.data.get(self.size - 1)
	}
	// 获取一个可变元素，需先判断是否为空栈
	fn peek_mut(&mut self) -> Option<&mut T> {
		if 0 == self.size {
			return None;
		}
		self.data.get_mut(self.size - 1)
	}

	// 返回一个实现了迭代器Trait的实例栈
	fn into_iter(self) -> IntoIter<T> {
		IntoIter(self)
	}
	// 只读栈
	fn iter(&self) -> Iter<T> {
		let mut iterator = Iter { 
			stack: Vec::new() 
		};
		for item in self.data.iter() {
			iterator.stack.push(item);
		}
		iterator
	}
	// 可变内容栈
	fn iter_mut(&mut self) -> IterMut<T> {
		let mut iterator = IterMut { 
			stack: Vec::new() 
		};
		for item in self.data.iter_mut() {
			iterator.stack.push(item);
		}
		iterator
	}
}

// 获取所有权并消耗栈
struct IntoIter<T>(Stack<T>);
// 实现迭代器 Trait
impl<T: Clone> Iterator for IntoIter<T> {
	type Item = T;
	fn next(&mut self) -> Option<Self::Item> {
		if !self.0.is_empty() {
			self.0.size -= 1;
			self.0.data.pop()
		} 
		else {
			None
		}
	}
}
// 只读栈
struct Iter<'a, T: 'a> {
	stack: Vec<&'a T>,
}
// 实现迭代器 Trait
impl<'a, T> Iterator for Iter<'a, T> {
	type Item = &'a T;
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()
	}
}
// 可变内容栈
struct IterMut<'a, T: 'a> {
	stack: Vec<&'a mut T>,
}
// 实现迭代器 Trait
impl<'a, T> Iterator for IterMut<'a, T> {
	type Item = &'a mut T;		// 迭代器返回的可变引用生命周期为 'a
	fn next(&mut self) -> Option<Self::Item> {
		self.stack.pop()		// 假设 stack 存储的引用生命周期与 'a 一致
	}
}

// 判断给定的字符串中的括号是否匹配
fn bracket_match(bracket: &str) -> bool {
	//TODO
	// 创建一个空的栈，用于存储左括号
	let mut stack = Stack::new();
	// 定义括号匹配规则
	// 创建一个空的栈，用于存储左括号
	let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}')];
	// 遍历字符串中的每个字符
	for c in bracket.chars() {
		// 如果是左括号，压入栈
		if ['(', '[', '{'].contains(&c) {
			stack.push(c);
		}
		// 如果是右括号, 先找到右括号对应的左括号
		else if let Some(&(left, _)) = bracket_pairs.iter().find(|&&(_, right)| right == c) {
			// 检查对应左括号与栈顶是否匹配
			if stack.pop() != Some(left) {
				return false;
			}
		}
	}
	// 如果当前栈为空，说明所有括号都匹配，返回 true；否则返回 false
	stack.is_empty()
	
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn bracket_matching_1(){
		let s = "(2+3){func}[abc]";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_2(){
		let s = "(2+3)*(3-1";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_3(){
		let s = "{{([])}}";
		assert_eq!(bracket_match(s),true);
	}
	#[test]
	fn bracket_matching_4(){
		let s = "{{(}[)]}";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_5(){
		let s = "[[[]]]]]]]]]";
		assert_eq!(bracket_match(s),false);
	}
	#[test]
	fn bracket_matching_6(){
		let s = "";
		assert_eq!(bracket_match(s),true);
	}
}