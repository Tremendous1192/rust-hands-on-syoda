// ▼ List 4-23
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // Create synchronous channels with capacity 1
    let (tx1, rx1) = mpsc::sync_channel(1);
    let (tx2, rx2) = mpsc::sync_channel(1);

    // Send an initial value through tx2
    tx2.send(0).unwrap();

    println!("Main: start!");

    // Spawn a thread (h1) that increments received values and sends them through tx1
    let h1 = thread::spawn(move || {
        println!("H1: start!");

        for _ in 1..5 {
            let val = rx2.recv().unwrap();
            let num = val + 1;
            println!("H1: num={}.", num);
            tx1.send(num).unwrap(); // Send incremented value through tx1
            thread::sleep(Duration::from_millis(10));
        }

        println!("H1: End.");
    });

    // Allow some time for h1 to start before spawning h2
    thread::sleep(Duration::from_millis(5));

    // Spawn a thread (h2) that doubles received values and sends them through tx2
    let h2 = thread::spawn(move || {
        println!("H2: start!");

        for _ in 1..5 {
            let val = rx1.recv().unwrap();
            let num = val * 2;
            println!("H2: num={}.", num);
            tx2.send(num).unwrap(); // Send doubled value through tx2
            thread::sleep(Duration::from_millis(10));
        }

        println!("H2: End.");
    });

    // Wait for threads to finish
    let _res = h1.join();
    let _res = h2.join();

    println!("Main: End.");
}
