// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


// use std::sync::{Arc, Mutex};
// use std::thread;
// use std::time::Duration;

// struct JobStatus {
//     jobs_completed: Mutex<u32>,
// }

// fn main() {
//     let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
//     let mut handles = vec![];
//     for _ in 0..10 {
//         let status_shared = Arc::clone(&status);
//         let handle = thread::spawn(move || {
//             thread::sleep(Duration::from_millis(250));
//             // TODO: You must take an action before you update a shared value
//             let mut status_shared_ = status_shared.jobs_completed.lock().unwrap();
//             *status_shared_ += 1;
//         });
//         handles.push(handle);
//     }
//     for handle in handles {
//         handle.join().unwrap();
//         // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
//         // anything interesting in the output? Do you have to 'join' on all the
//         // handles?
//         let jobs_com = status.jobs_completed.lock().unwrap();
//         println!("jobs completed {}", *jobs_com);
//     }
// }

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let mut handles = vec![];
    for i in 1..=10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 加锁以获取对共享状态的独占访问权
            let mut status_shared_ = status_shared.jobs_completed.lock().unwrap();
            *status_shared_ = i; // 直接赋值为当前循环的计数值
        });
        handles.push(handle);
    }
    for handle in handles {
        // 等待每个线程的结束
        let _ = handle.join().unwrap();

        // 获取当前共享状态并打印
        let jobs_com = status.jobs_completed.lock().unwrap();
        println!("jobs completed {}", *jobs_com);
    }
}
