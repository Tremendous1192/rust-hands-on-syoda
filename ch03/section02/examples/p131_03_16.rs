// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Function to print information about a person
fn print_person(p: Person) {
    println!("I'm {} ({} years old). Mail to {}.", p.name, p.age, p.mail);
}

fn main() {
    // Create an instance 'taro' of the 'Person' struct
    let taro = Person {
        name: String::from("Taro"),
        mail: String::from("taro@yamada"),
        age: 39,
    };

    // Call the 'print_person' function with the 'taro' instance
    print_person(taro);

    // Create another instance 'hanako' of the 'Person' struct
    let hanako = Person {
        name: String::from("Hanako"),
        mail: String::from("hanako@flower"),
        age: 28,
    };

    // Call the 'print_person' function with the 'hanako' instance
    print_person(hanako);
}
