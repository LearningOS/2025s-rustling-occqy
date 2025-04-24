// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 用 Arc 包裹 Mutex，再包裹 JobStatus，实现线程安全的共享与修改
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        // 克隆 Arc 让每个线程都能访问共享数据
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            // 加锁获取对共享数据的独占访问权
            let mut status_mut = status_shared.lock().unwrap();
            // 更新共享数据
            status_mut.jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        // 等待所有线程完成
        handle.join().unwrap();
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        // 加锁获取最终的共享数据
        let completed_jobs = status.lock().unwrap().jobs_completed;
        println!("jobs completed {}", completed_jobs);
    }
}
