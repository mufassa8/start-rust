// Variables hold primitive data or reference to data
// Variables are immutable by dfault
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;       //variables are immutable on default. To immute, 'mut' type should be fixed.
    println!("My name is {} and I am {}", name, age);
    age = 38;

    println!("My name is {} and I am {}", name, age);


    //Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple vars
    let ( mut my_name, my_age ) = ("Brad", 37);
    my_name = "John";
    println!("{} is {}", my_name, my_age);
}