fn main() {
    // Create a mutable vector with integer data
    let mut data = vec![123, 456, 789];

    // Print the initial state of the vector
    println!("{:?} ", data);

    // Remove the element at index 1 from the vector
    data.remove(1);

    // Print the vector after the removal
    println!("{:?} ", data);

    // Insert the value 100 at index 2 in the vector
    data.insert(2, 100);

    // Print the vector after the insertion
    println!("{:?} ", data);
}
