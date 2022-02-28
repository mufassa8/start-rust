// Primitve str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run(){
    let mut hello =  String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());
    
    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in byees
    println!("Capacity: {}", hello.capacity());
    
    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World' {}", hello.contains("World"));

    // Replace
    println!("Repalce: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }
    println!("{}", hello);
}