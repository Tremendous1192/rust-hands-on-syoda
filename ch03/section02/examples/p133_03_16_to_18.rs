// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Function to create a 'Person' instance with provided values
fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

// Function to print information about a person
fn print_person(p: Person) {
    println!("I'm {} ({} years old). Mail to {}.", p.name, p.age, p.mail);
}

fn main() {
    // Create a 'Person' instance 'taro' using the 'person' function
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);

    // Call the 'print_person' function with the 'taro' instance
    print_person(taro);

    // Create another 'Person' instance 'hanako' using the 'person' function
    let hanako = person(String::from("Hanako"), String::from("hanako@flower"), 28);

    // Call the 'print_person' function with the 'hanako' instance
    print_person(hanako);
}
