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

fn send_tx(tx: mpsc::Sender<u32>, data: Arc<Vec<u32>>) {
    let tx_clone = mpsc::Sender::clone(&tx); // 克隆发送者以便在线程中使用

    thread::spawn(move || {
        for val in data.iter() {
            println!("sending {:?}", val);
            tx_clone.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();

    let first_half = Arc::new(queue.first_half);
    let second_half = Arc::new(queue.second_half);

    send_tx(tx.clone(), first_half);
    send_tx(tx, second_half);

    let mut total_received: u32 = 0;
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue.length);
}