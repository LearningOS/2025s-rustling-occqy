// iterators1.rs
//
// When performing operations on elements within a collection, iterators are
// essential. This module helps you get familiar with the structure of using an
// iterator and how to go through elements within an iterable collection.
//
// Make me compile by filling in the `???`s
//
// Execute `rustlings hint iterators1` or use the `hint` watch subcommand for a
// hint.



fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: Step 1   将向量转换为迭代器

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));     // 第一次调用 next 方法，返回第一个元素的引用
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // Step 4 : 验证迭代器遍历完所有元素后的情况
}


// fn main() {
//     let numbers = vec![1, 2, 3];
//     let iter = numbers.iter();

//     // 使用 collect OR sum 方法将迭代器元素收集到一个新的向量中，这会耗尽迭代器
//     let collected: Vec<&i32> = iter.collect();

//     // 由于迭代器已被耗尽，再次调用 next 方法会直接返回 None
//     println!("{:?}", iter.next()); // 输出: None
// }


// 当你只需要读取集合中的元素，而不需要修改它们，同时又要保留集合的所有权时，适合使用 iter 方法
// fn main() {
//     let numbers = vec![1, 2, 3];
//     let iter = numbers.iter();
//     for num in iter {
//         // 只能读取元素，不能修改
//         println!("{}", num); 
//     }
//     // 迭代结束后，numbers 仍然可用
//     println!("Original vector: {:?}", numbers); 
// }


// 当你需要在迭代过程中修改集合中的元素，同时又要保留集合的所有权时，适合使用 iter_mut 方法
// fn main() {
//     let mut numbers = vec![1, 2, 3];
//     let iter_mut = numbers.iter_mut();
//     for num in iter_mut {
//         // 可以修改元素
//         *num *= 2; 
//     }
//     // 迭代结束后，numbers 被修改
//     println!("Modified vector: {:?}", numbers); 
// }


// 当你不再需要原始集合，并且想要完全消费集合中的元素时，适合使用 into_iter 方法。
// fn main() {
//     let numbers = vec![1, 2, 3];
//     let into_iter = numbers.into_iter();
//     for num in into_iter {
//         // 处理元素
//         println!("{}", num); 
//     }
//     // 迭代结束后，numbers 不再可用
//     // 下面这行代码会导致编译错误
//     // println!("Original vector: {:?}", numbers); 
// }