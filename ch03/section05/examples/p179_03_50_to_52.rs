// Define an enumeration 'Kind' with variants: 'Person', 'Cat', and 'Cow', each containing their respective structs
enum Kind {
    Person(Person),
    Cat(Cat),
    Cow(Cow),
}

// Define a struct 'Person' with fields: name (String), mail (String), and age (i32)
struct Person {
    name: String,
    mail: String,
    age: i32,
}

// Define a struct 'Cat' with fields: name (String), kind (CatKind), and feature (String)
struct Cat {
    name: String,
    kind: CatKind,
    feature: String,
}

// Define an enumeration 'CatKind' with variants: 'LongHair', 'ShortHair', and 'Sphynx'
#[derive(Debug)]
enum CatKind {
    LongHair,
    ShortHair,
    Sphynx,
}

// Define a struct 'Cow' with fields: kind (CowKind), weight (i32), and country (String)
struct Cow {
    kind: CowKind,
    weight: i32,
    country: String,
}

// Define an enumeration 'CowKind' with variants: 'Cow' and 'Beef'
#[derive(Debug)]
enum CowKind {
    Cow,
    Beef,
}

// Function to print all items in a vector of 'Kind'
fn print_all(data: Vec<Kind>) {
    for item in data {
        match item {
            Kind::Person(person) => {
                println!("Person: {}<{}>({}).", person.name, person.mail, person.age)
            }
            Kind::Cat(cat) => {
                println!("Cat: {}({:?}) Personality:\"{}\".", cat.name, cat.kind, cat.feature)
            }
            Kind::Cow(cow) => println!(
                "Cow: {:?} ({}kg) Origin:{}",
                cow.kind, cow.weight, cow.country
            ),
        }
    }
}

fn main() {
    // Create instances of 'Person', 'Cat', and 'Cow' wrapped in 'Kind' enum variants
    let taro = Kind::Person(Person {
        name: String::from("Taro"),
        mail: String::from("taro@yamada"),
        age: 39,
    });
    let tama = Kind::Cat(Cat {
        name: String::from("Tama"),
        kind: CatKind::ShortHair,
        feature: String::from("Affectionate and Lazy"),
    });
    let aug = Kind::Cow(Cow {
        kind: CowKind::Beef,
        weight: 498,
        country: String::from("Australia"),
    });

    // Create a vector of 'Kind' containing the instances
    let data = vec![taro, tama, aug];

    // Print all items in the vector
    print_all(data);
}
