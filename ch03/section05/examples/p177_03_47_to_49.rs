// Import the 'Debug' trait for debugging support
#[derive(Debug)]
// Define an enumeration 'Kind' with variants: 'Person' and 'Student', each containing their respective structs
enum Kind {
    Person(Person),
    Student(Student),
}

// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
#[derive(Debug)]
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Define a struct 'Student' with fields: name (String), mail (String), and grade (i32)
#[derive(Debug)]
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
        age: age,
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

// Define a trait 'Print' with a method 'to_string'
trait Print {
    fn to_string(&self) -> String;
}

// Implement the 'Print' trait for 'Person'
impl Print for Person {
    fn to_string(&self) -> String {
        String::from(&self.name) + "<" + &self.mail + ">(" + &self.age.to_string() + ")"
    }
}

// Implement the 'Print' trait for 'Student'
impl Print for Student {
    fn to_string(&self) -> String {
        String::from(&self.name) + "[grade " + &self.grade.to_string() + "]<" + &self.mail + ">"
    }
}

// Function to print all items in a vector of 'Kind'
fn print_all(data: Vec<Kind>) {
    for item in data {
        println!("{:?}", item);
    }
}

fn main() {
    // Create instances of 'Person' and 'Student' wrapped in 'Kind' enum variants
    let taro = Kind::Person(person("Taro", "taro@yamada", 39));
    let hanako = Kind::Student(student("Hanako", "hanako@flower", 2));
    let jiro = Kind::Person(person("Jiro", "jiro@change", 28));
    let sachiko = Kind::Student(student("Sachiko", "sachiko@happy", 4));

    // Create a vector of 'Kind' containing the instances
    let data: Vec<Kind> = vec![taro, hanako, jiro, sachiko];

    // Print all items in the vector
    print_all(data);
}
