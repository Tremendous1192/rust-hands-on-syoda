// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Function to create a 'Person' instance with the given parameters
fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

// Implementation block for methods associated with 'Person'
impl Person {
    // Method to print information about a person
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

fn main() {
    // Create a 'Person' instance 'taro' using the function
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);

    // Call the 'print' method on the 'taro' instance
    taro.print();

    // Create another 'Person' instance 'hanako' using the function
    let hanako = person(String::from("Hanako"), String::from("hanako@flower"), 28);

    // Call the 'print' method on the 'hanako' instance
    hanako.print();
}
