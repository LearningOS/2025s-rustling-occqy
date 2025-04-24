/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    // 要求我们的元素是可排序的
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
// }

// impl<T> TreeNode<T>
// // impl<T> BinarySearchTree<T>
// where
//     T: Ord,
// {

//     fn new() -> Self {
//         BinarySearchTree { root: None }
//     }

    // Insert a value into the BST
    
    fn insert(&mut self, value: T) {
        //TODO
        // 逻辑很清晰, 就像我们之前说的那样, 小值往左走, 大值往右走
        match value.cmp(&self.value) {
            Ordering::Less => {
                // 存在则递归调用
                if let Some(ref mut left) = self.left {
                    left.insert(value);
                // 不存在就直接插入
                } else {
                    self.left = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Greater => {
                if let Some(ref mut right) = self.right {
                    right.insert(value);
                } else {
                    self.right = Some(Box::new(TreeNode::new(value)));
                }
            }
            Ordering::Equal => {
                // 重复值处理: 这里我们选择不插入重复值
                // 也可以根据需求选择其他处理方式
            }
        }

    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool {
        //TODO
        match value.cmp(&self.value) {
            Ordering::Less => self.left.as_ref().map_or(false, |left| left.search(value)),
            Ordering::Greater => self.right.as_ref().map_or(false, |right| right.search(value)),
            Ordering::Equal => true,
        }
    }
}

impl<T> BinarySearchTree<T>
// impl<T> TreeNode<T>
where
    T: Ord,
{
    fn new() -> Self {
        BinarySearchTree { root: None }
    }
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if let Some(ref mut root) = self.root {
            root.insert(value);
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    fn search(&self, value: T) -> bool {
        self.root.as_ref().map_or(false, |root| root.search(value))
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);

        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


// //---------------
// #[derive(Debug, Default)]
// pub struct HuffmanNode {
//     pub character: Option<char>,    // None表示内部节点
//     pub frequency: usize,           //节点频率 (叶子节点为字符频率, 内部节点为子节点频率和)
//     pub left: Option<Box<HuffmanNode>>,
//     pub right: Option<Box<HuffmanNode>>,
// }

// // 实现一些基本的比较特性
// // 这是将节点放入最小堆的必要条件

// impl PartialOrd for HuffmanNode {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         Some(self.frequency.cmp(&other.frequency))
//     }
// }

// impl Ord for HuffmanNode {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.frequency.cmp(&other.frequency)
//     }
// }

// impl PartialEq for HuffmanNode {
//     fn eq(&self, other: &Self) -> bool {
//         self.frequency == other.frequency
//     }
// }

// impl Eq for HuffmanNode {}

// use std::collections::HashMap;

// // 构建树
// pub fn build_huffman_tree(text: &str) -> Option<Box<HuffmanNode>> {
//     if text.is_empty() {
//         return None;
//     }

//     // 统计字符频率
//     let mut freq_map = HashMap::new();
//     for c in text.chars() {
//         *freq_map.entry(c).or_insert(0) += 1;
//     }

//     // 创建最小堆并插入所有字符节点
//     let mut heap = MinHeap::new();
//     for (character, frequency) in freq_map {
//         heap.add(Box::new(HuffmanNode {
//             character: Some(character),
//             frequency,
//             left: None,
//             right: None,
//         }));
//     }

//     // 构建 Huffman 树
//     while heap.len() > 1 {
//         // 取出频率最小和次小的节点
//         let left = heap.next().unwrap();
//         let right = heap.next().unwrap();

//         let merged = Box::new(HuffmanNode {
//             character: None,
//             frequency: left.frequency + right.frequency,
//             left: Some(left),
//             right: Some(right),
//         });
//         // 将合并后的节点放回堆中
//         heap.add(merged);
//     }
//     heap.next()     // 返回最终的 Huffman 树根节点
// }

// // 由树构建编码
// pub fn build_huffman_codebook(root: &Box<HuffmanNode>) -> HashMap<char, String> {
//     let mut codebook = HashMap::new();
//     // 使用显式栈模拟递归
//     let mut stack = Vec::new();
//     // 处理单字符特殊情况
//     if let Some(ref c) = root.character {
//         codebook.insert(*c, "0".to_string());
//         return codebook;
//     }

//     // 初始状态, 根节点, 空路径
//     stack.push((root, String::new()));

//     // 深度优先遍历
//     // 取出当前栈顶和编码
//     while let Some((node, code)) = stack.pop() {
//         // 如果是含字符的叶子节点
//         if let Some(ref c) = node.character {
//             // 字符存入哈希表, 记录编码, 跳过后续分支处理
//             codebook.insert(*c, code);
//             continue;
//         }

//         // 左子树添加 "0", 右子树添加 "1"
//         // 每次遍历半棵树时都会再次推入两个新的子节点, 所以是深度优先
//         if let Some(ref left) = node.left {
//             stack.pish((left, code.clone() + "0"));
//         }

//         if let Some(ref right) = node.right {
//             stack.push((right, code + "1"));
//         }
//     }

//     codebook
// }

// pub fn huffman_encode(text: &str) -> (Option<Box<HuffmanNode>>, String) {
//     let tree = build_huffman_tree(text);

//     if tree.is_none() {
//         return (None, String::new());
//     }

//     let codebook = build_huffman_codebook(&tree.as_ref().unwrap());
//     let mut encoded = String::new();

//     for c in text.chars() {
//         encoded.push_str(&codebook[&c]);
//     }

//     (tree, encoded)
// }


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_huffman_single_char() {
//         let text = "aaaaa";
//         let (tree, encoded) = huffman_encode(text);
//         assert!(tree.is_some());
//         assert_eq!(encoded, "00000");
//     }

//     #[test]
//     fn test_huffman_encoding() {
//         let text = "this is an example of a huffman tree";
//         let (tree, encoded) = huffman_encode(text);
//         assert!(tree.is_some());

//         // 验证编码长度比原始文本短
//         let original_bits = text.len() * 8;
//         let encoded_bits = encoded.len();
//         assert!(encoded_bits < original_bits);

//         // 验证不同字符有不同的前缀编码
//         let codebook = build_huffman_codebook(&tree.unwrap());
//         let codes: Vec<&String> = codebook.values().collect();
//         for i in 0..codes.len() {
//             for j in i+1..codes.len() {
//                 assert!(!codes[i].starts_with(codes[j]));
//                 assert!(!codes[j].starts_with(codes[i]));
//             }
//         }
//     }

//     #[test]
//     fn test_empty_input() {
//         let (tree, encoded) = huffman_encode("");
//         assert!(tree.is_none());
//         assert!(encoded.is_empty());
//     }
// }