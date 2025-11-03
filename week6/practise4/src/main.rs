fn main() {
    let fullname = "Onyia Chianugo Daniel";
    let department = "Computer Science";
    let uni = "Pan-Atlantic Univeristy";

    let mut school = "School of Science".to_string();
    school.push_str(" and Technology");

    println!("My name is: {}", fullname );
    println!("The lenght my fullname is: {}", fullname.len());
    println!("I am a student of {} Department", department );
    println!("{}",school );
    println!("{}",uni );
}
