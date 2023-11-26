// ▼ List 4-17
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Initialize a mutable variable `num` with the value 1
    let mut num = 1;

    // Print a message indicating the start of the main thread
    println!("Main: start!");

    // Spawn a new thread (`h1`) capturing `num` by move
    let h1 = thread::spawn(move || {
        // Print a message indicating the start of the first spawned thread
        println!("H1: start!");

        // Loop in the first spawned thread to modify and print `num`
        for n in 1..5 {
            num = 10 * n;
            println!("H1: num_h={}.", num);
            thread::sleep(Duration::from_millis(10));
        }

        // Print a message indicating the end of the first spawned thread
        println!("H1: End.");
    });

    // Spawn another new thread (`h2`) capturing `num` by move
    let h2 = thread::spawn(move || {
        // Print a message indicating the start of the second spawned thread
        println!("H2: start!");

        // Loop in the second spawned thread to modify and print `num`
        for n in 1..5 {
            num += n;
            println!("H2: num_h={}.", num);
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
