// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.



use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: Mutex<u32>,
}

fn main() {
    let status = Arc::new(JobStatus { jobs_completed: Mutex::new(0) });
    let mut handles = vec![];
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // TODO: You must take an action before you update a shared value
            let mut jobs_completed = status_shared.jobs_completed.lock().unwrap();  
            *jobs_completed += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        // TODO: Print the value of the JobStatus.jobs_completed. Did you notice
        // anything interesting in the output? Do you have to 'join' on all the
        // handles?
        let jobs_completed = status.jobs_completed.lock().unwrap();  
        println!("jobs completed {}", *jobs_completed);
    }
}
