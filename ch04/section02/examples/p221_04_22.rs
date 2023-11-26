// ▼ List 4-22
// Import necessary modules from the standard library
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Create two channels with senders (`tx1`, `tx2`) and receivers (`rx1`, `rx2`)
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    // Send an initial value of 0 through the second channel (`tx2`)
    tx2.send(0).unwrap();

    // Print a message indicating the start of the main thread
    println!("Main: start!");

    // Spawn the first thread (`h1`) to communicate through channels
    let h1 = thread::spawn(move || {
        // Print a message indicating the start of the first spawned thread
        println!("H1: start!");

        // Loop in the first spawned thread to receive, manipulate, and send values through channels
        for _ in 1..5 {
            let val = rx2.recv().unwrap();
            let num = val + 1;
            println!("H1: num={}.", num);
            tx1.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }

        // Print a message indicating the end of the first spawned thread
        println!("H1: End.");
    });

    // Pause the main thread for 5 milliseconds
    thread::sleep(Duration::from_millis(5));

    // Spawn the second thread (`h2`) to communicate through channels
    let h2 = thread::spawn(move || {
        // Print a message indicating the start of the second spawned thread
        println!("H2: start!");

        // Loop in the second spawned thread to receive, manipulate, and send values through channels
        for _ in 1..5 {
            let val = rx1.recv().unwrap();
            let num = val * 2;
            println!("H2: num={}.", num);
            tx2.send(num).unwrap();
            thread::sleep(Duration::from_millis(10));
        }

        // Print a message indicating the end of the second spawned thread
        println!("H2: End.");
    });

    // Wait for the first spawned thread to finish its execution (join)
    let _res = h1.join();

    // Wait for the second spawned thread to finish its execution (join)
    let _res = h2.join();

    // Print a message indicating the end of the main thread
    println!("Main: End.");
}
