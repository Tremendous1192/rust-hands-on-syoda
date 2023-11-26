// ▼ List 4-14
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Spawn a new thread with a closure
    thread::spawn(|| {
        // Loop in the spawned thread to print numbers 1 to 99
        for n in 1..100 {
            println!("Thread:No,{}.", n);
        }
    });

    // Pause the main thread for a short duration (1 millisecond in this case)
    thread::sleep(Duration::from_millis(1)); //☆

    // Loop in the main thread to print numbers 1 to 99
    for n in 1..100 {
        println!("Main: No,{}.", n);
    }
}
