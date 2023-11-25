// Import the 'Debug' trait for automatic debugging support
#[derive(Debug)]
// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Import the 'Debug' trait for automatic debugging support
#[derive(Debug)]
// Define a struct 'Student' with fields: name (String), mail (String), and grade (i32)
struct Student {
    name: String,
    mail: String,
    grade: i32,
}

// Function to create a 'Person' instance with the given parameters
fn person(name: &str, mail: &str, age: i32) -> Person {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
}

// Function to create a 'Student' instance with the given parameters
fn student(name: &str, mail: &str, grade: i32) -> Student {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn main() {
    // Create a 'Person' instance 'taro' using the 'person' function
    let taro = person("Taro", "taro@yamada", 39);
    // Print the 'taro' instance for debugging
    println!("{:?}", taro);

    // Create a 'Student' instance 'hanako' using the 'student' function
    let hanako = student("Hanako", "hanako@flower", 2);
    // Print the 'hanako' instance for debugging
    println!("{:?}", hanako);
}
