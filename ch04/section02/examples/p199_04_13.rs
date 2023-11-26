// ▼ List 4-13
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Spawn a new thread with a closure
    thread::spawn(|| {
        // Loop in the spawned thread to print numbers 1 to 9
        for n in 1..10 {
            println!("Thread:No,{}.", n);

            // Simulate a delay of 50 milliseconds
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Loop in the main thread to print numbers 1 to 9
    for n in 1..10 {
        println!("Main: No,{}.", n);

        // Simulate a delay of 100 milliseconds
        thread::sleep(Duration::from_millis(100));
    }
}
