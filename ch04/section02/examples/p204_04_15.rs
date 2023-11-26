// ▼ List 4-15
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Print a message indicating the start of the main thread
    println!("Main:start!");

    // Spawn a new thread and capture its handle in the variable `h`
    let h = thread::spawn(|| {
        // Spawn two new threads concurrently using closures
        thread::spawn(|| {
            // Loop in the first spawned thread to print numbers 1 to 5
            for n in 1..6 {
                println!("H1:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        thread::spawn(|| {
            // Loop in the second spawned thread to print numbers 1 to 5
            for n in 1..6 {
                println!("H2:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        // Loop in the main spawned thread to print numbers 1 to 5
        for n in 1..6 {
            println!("Thread:No,{}.", n);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Wait for the spawned thread to finish its execution (join)
    let _res = h.join(); //☆

    // Print a message indicating the end of the main thread
    println!("Main:End.");
}
