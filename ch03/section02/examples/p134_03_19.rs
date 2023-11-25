// Define a tuple struct 'Person' with fields: String, String, and i32
struct Person(String, String, i32);

// Function to print information about a person
fn print_person(p: Person) {
    println!("I'm {} ({} years old). Mail to {}.", p.0, p.2, p.1);
}

fn main() {
    // Create a 'Person' instance 'taro' using the tuple struct
    let taro = Person(String::from("Taro"), String::from("taro@yamada"), 39);

    // Call the 'print_person' function with the 'taro' instance
    print_person(taro);

    // Create another 'Person' instance 'hanako' using the tuple struct
    let hanako = Person(String::from("Hanako"), String::from("hanako@flower"), 28);

    // Call the 'print_person' function with the 'hanako' instance
    print_person(hanako);
}
