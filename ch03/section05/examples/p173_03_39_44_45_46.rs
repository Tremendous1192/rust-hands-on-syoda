// Import the 'Debug', 'Copy', and 'Clone' traits for debugging and cloning support
#[derive(Debug, Copy, Clone)]
// Define an enumeration 'Kind' with variants: 'Person' and 'Student'
enum Kind {
    Person,
    Student,
}

// Define a struct 'Person' with fields: name (String), mail (String), age (i32), and kind (Kind)
struct Person {
    name: String,
    mail: String,
    age: i32,
    struct_kind: Kind,
}

// Define a struct 'Student' with fields: name (String), mail (String), grade (i32), and kind (Kind)
struct Student {
    name: String,
    mail: String,
    grade: i32,
    struct_kind: Kind,
}

// Function to create a boxed 'Person' instance with the given parameters
fn person(name: &str, mail: &str, age: i32) -> Box<Person> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
        struct_kind: Kind::Person,
    })
}

// Function to create a boxed 'Student' instance with the given parameters
fn student(name: &str, mail: &str, grade: i32) -> Box<Student> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
        struct_kind: Kind::Student,
    })
}

// Define a trait 'Print' with methods 'kind' and 'to_string'
trait Print {
    fn kind(&self) -> &Kind;
    fn to_string(&self) -> String;
}

// Implement the 'Print' trait for 'Person'
impl Print for Person {
    fn kind(&self) -> &Kind {
        &self.struct_kind
    }

    fn to_string(&self) -> String {
        String::from(&self.name) + "<" + &self.mail + ">(" + &self.age.to_string() + ")"
    }
}

// Implement the 'Print' trait for 'Student'
impl Print for Student {
    fn kind(&self) -> &Kind {
        &self.struct_kind
    }

    fn to_string(&self) -> String {
        String::from(&self.name) + "[grade " + &self.grade.to_string() + "]<" + &self.mail + ">"
    }
}

// Function to print all items in a vector that implement the 'Print' trait
fn print_all<T: Print + ?Sized>(data: Vec<Box<T>>) {
    for item in data {
        match item.kind() {
            Kind::Person => println!("Person:  {}", item.to_string()),
            Kind::Student => println!("Student: {}", item.to_string()),
        }
    }
}

fn main() {
    // Create instances of 'Person' and 'Student'
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    let jiro = person("Jiro", "jiro@change", 28);
    let sachiko = student("Sachiko", "sachiko@happy", 4);

    // Create a vector of boxed trait objects implementing 'Print'
    let data: Vec<Box<dyn Print>> = vec![taro, hanako, jiro, sachiko];

    // Print all items in the vector
    print_all(data);
}
