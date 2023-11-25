fn main() {
    // Define tuples representing information about individuals 'Taro' and 'Hanako'
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);

    // Destructure the tuples into individual variables for easier access
    let (name, age, male) = taro;
    // Print information about 'Taro'
    println!("name: {}, age: {}, male?: {}", name, age, male);

    // Destructure the tuples for 'Hanako'
    let (name, age, male) = hanako;
    // Print information about 'Hanako'
    println!("name: {}, age: {}, male?: {}", name, age, male);
}
