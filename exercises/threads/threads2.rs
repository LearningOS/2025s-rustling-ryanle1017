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
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 在更新共享值之前，我们需要获取互斥锁
            let mut status_locked = status_shared.lock().unwrap();
            status_locked.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 我们需要先等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 然后打印最终的结果
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
