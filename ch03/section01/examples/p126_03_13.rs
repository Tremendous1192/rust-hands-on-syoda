fn main() {
    // Create an array 'data' with elements 12, 34, 56, 78, 90
    let data = [12, 34, 56, 78, 90];

    // Slice the array 'data' from index 2 (inclusive) to index 4 (exclusive)
    let part = &data[2..4];

    // Print the sliced part and the original array 'data'
    println!("{:?} in {:?}", part, data);
}
