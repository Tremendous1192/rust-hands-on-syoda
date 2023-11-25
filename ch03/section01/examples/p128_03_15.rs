fn main() {
    // Create a mutable vector 'data' with elements 12, 34, 56, 78, 90
    let mut data = vec![12, 34, 56, 78, 90];

    // Create a mutable vector 'part' by slicing 'data' from index 2 (inclusive) to index 4 (exclusive)
    let mut part = data[2..4].to_vec();

    // Insert the value 999 at index 3 of the original vector 'data'
    data.insert(3, 999);

    // Push the value -1 to the 'part' vector
    part.push(-1);

    // Print the modified 'part' vector and the original 'data' vector
    println!("{:?} in {:?}", part, data);
}
