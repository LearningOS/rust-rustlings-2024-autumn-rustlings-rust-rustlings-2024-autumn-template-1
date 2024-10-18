// threads3.rs
//
// Execute `rustlings hint threads3` or use the `hint` watch subcommand for a
// hint.


use std::sync::mpsc;
use std::sync::{Arc, Mutex};
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

fn send_tx(q: Arc<Queue>, tx: Arc<Mutex<mpsc::Sender<u32>>>) {
    let qc1 = Arc::clone(&q);
    let tx1 = Arc::clone(&tx);
    let tx2 = Arc::clone(&tx);

    thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            let tx = tx1.lock().unwrap();
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        
        let qc2 = Arc::clone(&q);
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            let tx = tx2.lock().unwrap();
            tx.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx = Arc::new(Mutex::new(tx)); // Wrap tx in Arc and Mutex
    let queue = Arc::new(Queue::new()); // Wrap queue in Arc
    let queue_length = queue.length;

    send_tx(queue, tx);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}
