// ▼ List 4-12
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Spawn a new thread with a closure
    thread::spawn(|| {
        // Code to be executed in the new thread
        println!("Thread:Start!");

        // Simulate a delay of 10 milliseconds
        thread::sleep(Duration::from_millis(10));

        println!("Thread:End.");
    });

    // Code in the main thread
    println!("Main:Start!");

    // Simulate a delay of 100 milliseconds
    thread::sleep(Duration::from_millis(100));

    println!("Main:End.");
}
