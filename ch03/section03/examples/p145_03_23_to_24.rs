// Define a trait 'Print' with a method 'print'
trait Print {
    fn print(&self);
}

// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Implement the 'Print' trait for 'Person'
impl Print for Person {
    fn print(&self) {
        println!("{}<{}>({}).", self.name, self.mail, self.age);
    }
}

// Function to create a 'Person' instance with the given parameters
fn person(name: String, mail: String, age: i32) -> Person {
    Person { name, mail, age }
}

// Define a struct 'Student' with fields: name (String), mail (String), and grade (i32)
struct Student {
    name: String,
    mail: String,
    grade: i32,
}

// Implement the 'Print' trait for 'Student'
impl Print for Student {
    fn print(&self) {
        println!("grade{}: {}<{}>.", self.grade, self.name, self.mail);
    }
}

// Function to create a 'Student' instance with the given parameters
fn student(name: String, mail: String, grade: i32) -> Student {
    Student { name, mail, grade }
}

fn main() {
    // Create a 'Person' instance 'taro' using the 'person' function
    let taro = person(String::from("Taro"), String::from("taro@yamada"), 39);

    print(taro);

    // Create a 'Student' instance 'hanako' using the 'student' function
    let hanako = student(String::from("Hanako"), String::from("hanako@flower"), 2);

    print(hanako);
}

// Function to print any type that implements the 'Print' trait
fn print(ob: impl Print) {
    ob.print();
}
