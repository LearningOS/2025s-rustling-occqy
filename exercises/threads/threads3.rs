// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.



use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> () {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);

    // 克隆第一个 Sender
    let tx1 = tx.clone();
    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // 使用原始 Sender
    thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length)
}



// 问题核心：
// 当需要为多个线程克隆 mpsc::Sender 时，避免为每个线程显式创建变量，
// 改用循环统一处理，提升代码简洁性和扩展性。

// 解决方案：
// 结构化数据： 将需要多线程处理的数据部分（如 first_half, second_half）组织成可遍历的集合（如 Vec）。
// 循环生成线程： 遍历每个数据部分，在循环内部自动克隆 Sender 并生成线程。
// 统一管理资源： 利用 Arc 共享数据和 clone() 自动管理 Sender。

// 修正代码：
// use std::sync::mpsc;
// use std::sync::Arc;
// use std::thread;
// use std::time::Duration;

// struct Queue {
//     length: u32,
//     first_half: Vec<u32>,
//     second_half: Vec<u32>,
//     // 若有 qc3，可继续添加字段
// }

// impl Queue {
//     fn new() -> Self {
//         Queue {
//             length: 10,
//             first_half: vec![1, 2, 3, 4, 5],
//             second_half: vec![6, 7, 8, 9, 10],
//         }
//     }
// }

// fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
//     let qc = Arc::new(q);
//     // 将需要处理的部分组织成 Vec（可扩展）
//     let parts = vec![
//         &qc.first_half,
//         &qc.second_half,
//         // 若有 qc3，添加 &qc.third_half
//     ];

//     for part in parts {
//         let tx_clone = tx.clone(); // 每次循环自动克隆 Sender
//         let qc_clone = Arc::clone(&qc); // 保持 Arc 引用计数
//         thread::spawn(move || {
//             for val in *part { // 解引用获取数据部分
//                 println!("发送: {:?}", val);
//                 tx_clone.send(*val).unwrap();
//                 thread::sleep(Duration::from_secs(1));
//             }
//         });
//     }
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let queue = Queue::new();
//     let queue_length = queue.length;

//     send_tx(queue, tx);

//     let mut total_received: u32 = 0;
//     for received in rx {
//         println!("收到: {}", received);
//         total_received += 1;
//     }

//     println!("总共收到 {} 个数字", total_received);
//     assert_eq!(total_received, queue_length);
// }

// 关键改进点：
// 数据组织： 使用 Vec 存储所有需要处理的数据部分引用，如 first_half, second_half。若有 third_half，只需添加到该 Vec。
// 循环处理： 遍历 parts，每次循环自动克隆 tx 并生成线程，无需手动管理每个 tx1, tx2。

// 自动资源管理：
// tx.clone() 在循环中自动为每个线程生成独立的 Sender。
// Arc::clone(&qc) 确保每个线程持有数据的所有权，避免生命周期问题。



// 扩展性示例：
// 若添加 third_half 字段：

// struct Queue {
//     length: u32,
//     first_half: Vec<u32>,
//     second_half: Vec<u32>,
//     third_half: Vec<u32>, // 新增部分
// }

// impl Queue {
//     fn new() -> Self {
//         Queue {
//             length: 15, // 修改总长度
//             first_half: vec![1, 2, 3, 4, 5],
//             second_half: vec![6, 7, 8, 9, 10],
//             third_half: vec![11, 12, 13, 14, 15], // 新增数据
//         }
//     }
// }

// // 在 send_tx 中只需更新 parts：
// let parts = vec![
//     &qc.first_half,
//     &qc.second_half,
//     &qc.third_half, // 添加新部分
// ];

// 优势：
// 代码简洁： 无需为每个线程重复编写克隆和线程生成代码。
// 易于扩展： 新增数据部分只需修改 parts 列表，无需改动线程逻辑。
// 资源安全： 通过 Arc 和 clone() 确保线程间安全共享数据。






// use std::sync::mpsc;
// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// struct Queue {
//     length: u32,
//     first_half: Vec<u32>,
//     second_half: Vec<u32>,
// }

// impl Queue {
//     fn new() -> Self {
//         Queue {
//             length: 10,
//             first_half: vec![1, 2, 3, 4, 5],
//             second_half: vec![6, 7, 8, 9, 10],
//         }
//     }
// }

// fn send_tx(q: Arc<Mutex<Queue>>, tx: mpsc::Sender<u32>) -> (thread::JoinHandle<()>, thread::JoinHandle<()>) {
//     let qc1 = Arc::clone(&q);
//     let qc2 = Arc::clone(&q);
//     let tx1 = tx.clone();
//     let tx2 = tx.clone();

//     let handle1 = thread::spawn(move || {
//         let q1 = qc1.lock().unwrap();
//         for val in &q1.first_half {
//             println!("sending {:?}", val);
//             if let Err(e) = tx1.send(*val) {
//                 eprintln!("Error sending data: {}", e);
//             }
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     let handle2 = thread::spawn(move || {
//         let q2 = qc2.lock().unwrap();
//         for val in &q2.second_half {
//             println!("sending {:?}", val);
//             if let Err(e) = tx2.send(*val) {
//                 eprintln!("Error sending data: {}", e);
//             }
//             thread::sleep(Duration::from_secs(1));
//         }
//     });

//     (handle1, handle2)
// }

// fn main() {
//     let (tx, rx) = mpsc::channel();
//     let queue = Arc::new(Mutex::new(Queue::new()));
//     let queue_length = queue.lock().unwrap().length;

//     let (handle1, handle2) = send_tx(queue, tx);

//     // 等待线程完成
//     handle1.join().unwrap();
//     handle2.join().unwrap();

//     let mut total_received: u32 = 0;
//     for _ in 0..queue_length as usize {
//         if let Ok(received) = rx.recv_timeout(Duration::from_secs(2)) {
//             println!("Got: {}", received);
//             total_received += 1;
//         } else {
//             eprintln!("Error receiving data");
//         }
//     }

//     println!("total numbers received: {}", total_received);
//     assert_eq!(total_received, queue_length);
// }
