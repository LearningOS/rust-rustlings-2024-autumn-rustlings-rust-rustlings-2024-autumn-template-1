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
    // Wrap JobStatus in a Mutex for safe concurrent access
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status); // Clone the Arc to share it with the new thread
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // Lock the mutex to safely update the shared value
            let mut job_status = status_shared.lock().unwrap();
            job_status.jobs_completed += 1; // Increment the jobs_completed count
        });
        handles.push(handle); // Store the thread handle
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    // Lock the mutex to safely read the final value of jobs_completed
    println!("jobs completed {}", status.lock().unwrap().jobs_completed);
}
