// threads1.rs
//
// This program spawns multiple threads that each run for at least 250ms, and
// each thread returns how much time they took to complete. The program should
// wait until all the spawned threads have finished and should collect their
// return values into a vector.
//
// Execute `rustlings hint threads1` or use the `hint` watch subcommand for a
// hint.



use std::thread;
use std::time::{Duration, Instant};

fn main() {
    // 线程句柄被存储在 handles 向量中
    let mut handles = vec![];
    for i in 0..10 {
        // 使用 thread::spawn 方法创建 10 个线程
        handles.push(thread::spawn(move || {
            let start = Instant::now();
            thread::sleep(Duration::from_millis(250));
            println!("thread {} is complete", i);
            start.elapsed().as_millis()
        }));
    }

    let mut results: Vec<u128> = vec![];
    // 使用 for 循环遍历 handles 向量
    for handle in handles {
        // TODO: a struct is returned from thread::spawn, can you use it?
        // 对每个线程句柄调用 join 方法
        // join 方法会阻塞当前线程，直到对应的子线程执行完毕
        // 使用 expect 方法处理 join 方法的返回值，如果出现错误，会打印错误信息并终止程序
        let  result = handle.join().expect("Thread panicked");
        // 将子线程的返回值添加到 results 向量中
        results.push(result);
    }

    // 检查 results 向量的长度是否为 10，如果不是，则抛出恐慌
    if results.len() != 10 {
        panic!("Oh no! All the spawned threads did not finish!");
    }

    // 遍历 results 向量，打印每个线程的编号和执行时间
    println!();
    for (i, result) in results.into_iter().enumerate() {
        println!("thread {} took {}ms", i, result);
    }
}
