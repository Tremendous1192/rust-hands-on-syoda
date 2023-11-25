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
fn person(name: &str, mail: &str, age: i32) -> impl Print {
    Person {
        name: String::from(name),
        mail: String::from(mail),
        age,
    }
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
fn student(name: &str, mail: &str, grade: i32) -> impl Print {
    Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    }
}

fn main() {
    // Create a 'Person' instance 'taro' using the 'person' function
    let taro = person("Taro", "taro@yamada", 39);
    // Call the 'print' function with a reference to 'taro' instance
    print(&taro);

    // Create a 'Student' instance 'hanako' using the 'student' function
    let hanako = student("Hanako", "hanako@flower", 2);
    // Call the 'print' function with a reference to 'hanako' instance
    print(&hanako);
}

// Function to print any type that implements the 'Print' trait
fn print(ob: &impl Print) {
    ob.print();
}
