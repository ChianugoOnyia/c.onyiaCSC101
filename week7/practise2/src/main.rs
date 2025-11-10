use std::io;
fn checker(){
    let mut input = String::new();
    println!("Enter a character:");
    io::stdin().read_line(&mut input).expect("Failed");
    let ch:char = input.trim().parse().expect("Invalid");

    if ch >= '0' && ch <= '9'
    {
        println!("character '{}' is a digit", ch);
    }
else{
    println!("Charcter '{}' is not a digit", ch);
}
}
fn main() {
    println!("Welcome! This program checks whether a character variable
        contains a digit or not");
    checker()
}