// ▼ List 4-11
// Import the thread module from the standard library
use std::thread;

// Entry point of the program
fn main() {
    // Spawn a new thread with a closure
    thread::spawn(|| {
        // Code to be executed in the new thread
        println!("Thread:Start!");
        println!("Thread:End.");
    });

    // Code in the main thread
    println!("Main:Start!");
    println!("Main:End.");
}
