fn main() {
    // Create tuples representing information about individuals 'Taro' and 'Hanako'
    let taro = ("Taro", 39, true);
    let hanako = ("Hanako", 28, false);

    // Print the entire tuples
    println!("{:?}", taro);
    println!("{:?}", hanako);

    // Print specific information from each tuple
    println!("name: {}, {}", taro.0, hanako.0);
    println!("age: {}, {}", taro.1, hanako.1);
    println!("male?: {}, {}", taro.2, hanako.2);
}
