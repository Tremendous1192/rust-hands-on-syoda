// ▼ List 4-21
// Import necessary modules from the standard library
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Create a multiple-producer, single-consumer channel with a sender (`tx`) and receiver (`rx`)
    let (tx, rx) = mpsc::channel();

    // Print a message indicating the start of the main thread
    println!("Main: start!");

    // Spawn the first thread (`h1`) to send values through the channel
    let h1 = thread::spawn(move || {
        // Initialize a mutable variable `num`
        let mut num = 1;

        // Print a message indicating the start of the first spawned thread
        println!("H1: start!");

        // Loop in the first spawned thread to increment and send values through the channel
        for n in 1..5 {
            num += n;
            tx.send(num).unwrap();
            println!("H1: num={}.", num);
            thread::sleep(Duration::from_millis(10));
        }

        // Print a message indicating the end of the first spawned thread
        println!("H1: End.");
    });

    // Spawn the second thread (`h2`) to receive values from the channel
    let h2 = thread::spawn(move || {
        // Print a message indicating the start of the second spawned thread
        println!("H2: start!");

        // Loop in the second spawned thread to receive and print values from the channel
        for _n in 1..5 {
            let num_recv = rx.recv().unwrap();
            println!("H2: num_recv={}.", num_recv);
            thread::sleep(Duration::from_millis(20));
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
