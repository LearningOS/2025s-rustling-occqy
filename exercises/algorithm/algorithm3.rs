/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


// fn sort<T>(array: &mut [T]){
// 	//TODO
// }
// 冒泡排序 (Bubble Sort)
fn sort<T: Ord>(array: &mut [T]) {
    for i in 0..array.len() {                   // 外层循环控制排序轮数
        for j in 0..array.len() - i - 1 {       // 内层循环控制每轮比较次数
            if array[j] > array[j + 1] {        // 如果前一个元素大于后一个
                array.swap(j, j + 1);            // 交换它们的位置
            }
        }
    }
}



//--------
// // 插入排序 (Insertion Sort)
// fn insertion_sort<T: Ord>(array: &mut [T]) {
//     for i in 1..array.len() {                   // 从第二个元素开始 (索引1)
//         let mut j = i;                          // j 是当前要插入的元素位置
//         while j > 0 && array[j] < array[j - 1] { // 当前元素比前一个小
//             array.swap(j, j - 1);               // 交换它们
//             j -= 1;                             // 继续向前比较
//         }
//     }
// }

// fn quick_sort<T, Ord>(array: &mut [T]) {
//     if array.len() <= 1 {       // 基本情况: 空数组或单元素数组已排序
//         return;
//     }

//     let pivot = partition(array);   // 获取基准位置
//     quick_sort(&mut array[..pivot]);    // 递归排序左半部分
//     quick_sort(&mut array[pivot + 1..]);     // 递归排序右半部分
// }

// fn partition<T: Ord>(array: &mut [T]) -> usize {
//     let pivot = array.len() - 1;        // 选择最后一个元素作为基准
//     let mut i = 0;              // i 是小于基准的元素的边界, 为了避免每次都移动基准
//     for j in 0..pivot {     // 遍历除基准外的所有元素
//         if array[j] <= array[pivot] {       // 当前元素小于等于基准
//             array.swap(i, j);       // 把它放到i的位置
//             i += 1;     // 移动i边界
//         }
//     }
//     array.swap(i, pivot);       // 把基准放到正确位置
//     i       // 返回基准的最终位置
// }



//---------
// // 这次我们先看辅助函数, 负责将数组的一部分 "堆化"
// fn heapify<T: Ord>(array: &mut [T], root: usize, end:usize) {
//     // 假设根节点最大
//     let mut largest = root;
//     // 因为通常来说, 每层元素的个数是上一层的两倍, 所以上层节点的序号的二倍就是本层节点序号的开始
//     // 但因为这里和二叉堆不同, 根节点索引为 0
//     // 所以需要通过 +1 和 +2 获得本层两个节点的索引
//     // 这相当于将一个二叉堆平铺在数组上
//     let left = 2 * root + 1;    // 左子节点索引
//     let right = 2 * root + 2;   // 右子节点索引

//     // 我们只处理这一层, 用 end 作为边界
//     // 找出 root, left, right 中最大的
//     if  left < end && array[left] > array[largest] {
//         largest = left;
//     }
//     if right < end && array[right] > array[largest] {
//         largest = right;
//     }
//     // 如果最大不是 root
//     if largest != root {
//         // 交换它们
//         array.swap(root, largest);
//          // 下沉操作可能破坏堆的性质, 递归堆化受影响的子树
//         heapify(array, largest, end);
//     }
// }

// fn heap_sort<T: Ord>(array: &mut [T]) {
//     if array.len() <= 1 {    // 基本情况
//         return;
//     }

//     // 在这里, 我们相对于把数组结构视为一颗无序二叉树
//     // 先构建最大堆
//     // 需要从最后一个非叶子节点开始, 即最后一个节点的父节点, 向上层层堆化
//     // 如果是左子节点直接倒过来就是 `(idx-1)/2 = (len-1-1)/2 = len/2-1`
//     // 右子节点理论需要额外 -1, 所以多出来 1/2 `len/2-1+1/2`
//     // 但这里是整数除法, 舍去小数, 仍然是 `len/2-1`
//     // 且 `..` 不包含右值, 所以 `-1` 也不需要了
//     for i in (0..array.len() / 2).rev() {
//         heapify(array, i, array.len());
//     }

//     // 提取元素
//     // 需要一个和数组长度相等且从后往前的索引
//     for i in (1..array.len()).rev() {
//         // 交换当前最大元素放与指定索引
//         array.swap(0, i);
//         // 然后我们下一轮循环的堆化和交换都忽略这个已经被移动到末尾的最大值
//         // 对剩余元素重新堆化
//         heapify(array, 0, i);
//     }
// }


// 混合排序(Timsort)
// fn tim_sort<T: Ord + Clone>(array: &mut [T]) {
//     const MIN_RUN: usize = 32;  // 最小 Run 长度

//     let len = array.len();
//     if len <= MIN_RUN {     // 小数组直接插入排序
//         // 这个我们之前实现过
//         insertion_sort(array);
//         return;
//     }

//     // 将数组分成 MIN_RUN 大小的块并排序
//     for i in (0..len).step_by(MIN_RUN) {
//         let end = std::cmp::min(i + MIN_RUN, len);
//         insertion_sort(&mut array[i..end]);
//     }

//     // 合并已排序的 Run
//     // 初始块大小
//     let mut size = MIN_RUN;
//     // 只要块仍然比整个数组小
//     while size < len {
//         // 每次处理两个相邻的块
//         for lest in (0..len).step_by(2 * size) {
//             // 计算中间点和右边界
//             // 第一个块的结尾
//             let mid = std::cmp::min(left + size, len);
//             // 第二个块的结尾
//             let right = std::cmp::min(left + 2 * size, len);
//             // 如果有两个块可以合并
//             if mid < right {
//                 // 合并它们
//                 merge(array, left, mid, right);
//             }
//         }
//         // 每次合并后对小块大小的要求大小翻倍
//         size *= 2;
//     }
// }

// fn merge<T: Ord + Clone>(array: &mut [T], left: usize, mid: usize, right: usize) {
//     let left_part = array[left..mid].to_vec();   // 复制左半部分
//     let right_part = array[mid..right].to_vec(); // 复制右半部分

//     let mut i = 0;      // 左部分索引
//     let mut j = 0;      // 右部分索引
//     let mut k = left;   // 合并位置索引


//     // 这和我们合并链表的方式相同
//     // 合并两个已排序数组
//     while i < left_part.len() && j < right_part.len() {
//         if left_part[i] <= right_part[j] {
//             array[k] = left_part[i].clone();
//             i += 1;
//         } else {
//             array[k] = left_part[j].clone();
//             j += 1;
//         }
//         k += 1;
//     }

//     // 复制剩余元素
//     while i < left_part.len() {
//         array[k] = left_part[i].clone();
//         i += 1;
//         k += 1;
//     }

//     while j < right_part.len() {
//         array[k] = right_part[j].clone();
//         j += 1;
//         k += 1;
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}