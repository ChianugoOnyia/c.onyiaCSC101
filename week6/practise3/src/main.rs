fn main() {
    let name1 = "Onyia Chianugo";
    println!("My name is {}",name1 );

    let name2 = name1.replace("Onyia","Tom");
    println!("You can also call me {}",name2 );
    let faculty = "Faculty of science and technology";

    let school = faculty.replace("Faculty", "School");
    println!("I am a student of the {}",school );
}
