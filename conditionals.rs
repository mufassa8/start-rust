pub fn run() {
    let age: u8 = 22;
    let check_id: bool = true;
    let knows_person_of_age = true;

    println!("{}", age);
    // If/Else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you linke to drink?");
    }else if age < 21 && check_id {
        println!("Bartender: SOrry, you have to leave");
    }else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthand If
    let is_of_age = if age >= 21 { true} else { false };
    println!("Is of Age {}", is_of_age);
}