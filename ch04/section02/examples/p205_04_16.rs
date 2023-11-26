// ▼ List 4-16
// Import necessary modules from the standard library
use std::thread;
use std::time::Duration;

// Entry point of the program
fn main() {
    // Print a message indicating the start of the main thread
    println!("Main:start!");

    // Spawn a new thread and capture its handle in the variable `h`
    let h = thread::spawn(|| {
        // Spawn two new threads (`h1` and `h2`) concurrently using closures
        let h1 = thread::spawn(|| {
            // Loop in the first spawned thread to print numbers 1 to 9
            for n in 1..10 {
                println!("H1:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        let h2 = thread::spawn(|| {
            // Loop in the second spawned thread to print numbers 1 to 9
            for n in 1..10 {
                println!("H2:No,{}.", n);
                thread::sleep(Duration::from_millis(2));
            }
        });

        // Loop in the main spawned thread to print numbers 1 to 9
        for n in 1..10 {
            println!("Thread:No,{}.", n);
            thread::sleep(Duration::from_millis(1));
        }

        // Wait for the first spawned thread to finish its execution (join)
        let _res1 = h1.join(); //☆
                               // Wait for the second spawned thread to finish its execution (join)
        let _res2 = h2.join(); //☆
    });

    // Wait for the main spawned thread to finish its execution (join)
    let _res = h.join(); //☆

    // Print a message indicating the end of the main thread
    println!("Main:End.");
}
