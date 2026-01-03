use std::sync::mpsc;
use std::thread;

pub fn run_multithreading_example() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("{i} from spawned thread");
        }
    });

    for i in 0..10 {
        println!("{i} from main thread");
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}

pub fn demonstrate_move_ownership() {
    let v = vec![1, 2, 3];

    // Move ownership of v to the new thread
    let handle = thread::spawn(move || {
        println!("Vector in spawned thread: {:?}", v);
    });

    // This line would cause a compile-time error
    // print!("Vector in main thread: {:?}", v);

    handle.join().unwrap();
}

// find sum of numbers from 0 to 10^10 using multithreading
pub fn sum_large_range() -> u128 {
    // mpsc: multiple producer, single consumer
    let (tx, rx) = mpsc::channel();

    for i in 0..1000 {
        let producer: mpsc::Sender<u128> = tx.clone();
        thread::spawn(move || {
            let mut sum: u128 = 0;
            for j in ((i as u128) * 10_000_000)..((i + 1) * 10_000_000) {
                sum += j;
            }
            producer.send(sum).unwrap();
        });
    }

    // Drop the original transmitter to avoid hanging otherwise the receiver will wait forever for tx to finish
    drop(tx);

    let mut total_sum: u128 = 0;
    for x in rx {
        total_sum += x;
    }

    return total_sum;
}
