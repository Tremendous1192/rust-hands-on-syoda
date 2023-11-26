// ▼ List 4-18
// Import necessary modules from the standard library
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Create a shared, thread-safe mutable variable `num` wrapped in a mutex
    let num = Arc::new(Mutex::new(1));

    // Print a message indicating the start of the main thread
    println!("Main: start!");

    // Clone the Arc<Mutex<i32>> to be moved into the first spawned thread (`h1`)
    let num_1 = Arc::clone(&num);

    // Spawn the first thread (`h1`) capturing the cloned `num_1`
    let h1 = thread::spawn(move || {
        // Acquire a lock on the mutex and create a mutable reference to `num_h1`
        let mut num_h1 = num_1.lock().unwrap();

        // Print a message indicating the start of the first spawned thread
        println!("H1: start!");

        // Loop in the first spawned thread to modify and print `num_h1`
        for n in 1..5 {
            *num_h1 += n;
            println!("H1: num_h={}.", *num_h1);
            thread::sleep(Duration::from_millis(1));
        }

        // Print a message indicating the end of the first spawned thread
        println!("H1: End.");
    });

    // Clone the Arc<Mutex<i32>> to be moved into the second spawned thread (`h2`)
    let num_2 = Arc::clone(&num);

    // Spawn the second thread (`h2`) capturing the cloned `num_2`
    let h2 = thread::spawn(move || {
        // Acquire a lock on the mutex and create a mutable reference to `num_h2`
        let mut num_h2 = num_2.lock().unwrap();

        // Print a message indicating the start of the second spawned thread
        println!("H2: start!");

        // Loop in the second spawned thread to modify and print `num_h2`
        for n in 1..5 {
            *num_h2 *= n;
            println!("H2: num_h={}.", *num_h2);
            thread::sleep(Duration::from_millis(1));
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
