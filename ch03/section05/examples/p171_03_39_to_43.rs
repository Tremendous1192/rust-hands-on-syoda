// Import the 'Debug', 'Copy', and 'Clone' traits for debugging and cloning support
#[derive(Debug, Copy, Clone)]
// Define an enumeration 'Kind' with variants: 'Person' and 'Student'
enum Kind {
    Person,
    Student,
}

// Import the 'Rng' trait from the 'rand' crate for generating random numbers
use rand::Rng;

// Implement methods for the 'Kind' enumeration
impl Kind {
    // Function to generate a random 'Kind' variant
    fn random() -> Kind {
        // List of possible 'Kind' variants
        let list = [Kind::Person, Kind::Student];
        // Generate a random index within the range of the list
        let index = rand::thread_rng().gen_range(0..list.len());
        // Return the randomly selected 'Kind' variant
        list[index]
    }
}

// Define a struct 'Mydata' with fields: name (String) and kind (Kind)
#[derive(Debug)]
struct Mydata {
    name: String,
    kind: Kind,
}

// Function to create a 'Mydata' instance with a random 'Kind'
fn mydata(name: &str) -> Mydata {
    Mydata {
        name: String::from(name),
        kind: Kind::random(),
    }
}

// Function to print all items in a vector of 'Mydata'
fn print_all(data: Vec<Mydata>) {
    for item in data {
        println!("{:?}", item);
    }
}

fn main() {
    // Create instances of 'Mydata' with random 'Kind'
    let taro = mydata("Taro");
    let hanako = mydata("Hanako");
    let sachiko = mydata("Sachiko");

    // Create a vector of 'Mydata' instances
    let data = vec![taro, hanako, sachiko];

    // Print all items in the vector
    print_all(data);
}
