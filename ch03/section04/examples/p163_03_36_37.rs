// Define a trait 'Print' with a default method 'print'
trait Print {
    fn print(&self) {
        println!("not implemented...");
    }
}

// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Define a struct 'Student' with fields: name (String), mail (String), and grade (i32)
struct Student {
    name: String,
    mail: String,
    grade: i32,
}

// Implement the 'Print' trait for 'Person'
impl Print for Person {
    fn print(&self) {
        println!("Person: {}<{}>({})", &self.name, &self.mail, &self.age);
    }
}

// Implement the 'Print' trait for 'Student'
impl Print for Student {
    fn print(&self) {
        println!(
            "Student [grade {}] {}<{}>",
            &self.grade, &self.name, &self.mail
        );
    }
}

// Function to create a boxed 'Person' instance with the given parameters
fn person(name: &str, mail: &str, age: i32) -> Box<Person> {
    Box::new(Person {
        name: String::from(name),
        mail: String::from(mail),
        age: age,
    })
}

// Function to create a boxed 'Student' instance with the given parameters
fn student(name: &str, mail: &str, grade: i32) -> Box<Student> {
    Box::new(Student {
        name: String::from(name),
        mail: String::from(mail),
        grade,
    })
}

// Function to print all items in a vector that implement the 'Print' trait
fn print_all<T: Print + ?Sized>(data: Vec<Box<T>>) {
    for item in data {
        item.print();
    }
}

fn main() {
    // Create instances of 'Person' and 'Student'
    let taro = person("Taro", "taro@yamada", 39);
    let hanako = student("Hanako", "hanako@flower", 2);
    let jiro = person("Jiro", "jiro@change", 28);
    let sachiko = student("Sachiko", "sachiko@happy", 4);

    // Create vectors of boxed 'Person' and 'Student' instances
    let data_p: Vec<Box<Person>> = vec![taro, jiro];
    let data_s: Vec<Box<Student>> = vec![hanako, sachiko];

    // Print all items in the vectors
    print_all(data_p);
    print_all(data_s);
}
