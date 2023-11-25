fn main() {
    // Define constant tuples representing information about individuals 'Taro' and 'Hanako'
    const TARO: (&str, i32, bool) = ("Taro", 39, true);
    const HANAKO: (&str, i32, bool) = ("Hanako", 28, false);

    // Destructure the tuples into individual variables for easier access
    let (name, age, male) = TARO;
    // Print information about 'Taro'
    println!("name: {}, age: {}, male?: {}", name, age, male);

    // Destructure the tuples for 'Hanako'
    let (name, age, male) = HANAKO;
    // Print information about 'Hanako'
    println!("name: {}, age: {}, male?: {}", name, age, male);
}
