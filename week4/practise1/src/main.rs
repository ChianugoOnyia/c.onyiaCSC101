use std::io;
fn main() {
    println!("\nstudent information management system");
    println!("\nplease enter your name.");
    let mut name = String::new();
    io::stdin()
    .read_line(&mut name)
    .expect("failed to read input");
    println!("Your name is: {}", name);

    println!("\n enter your age.");
    let mut age = String::new();
    io::stdin().read_line(&mut age).expect("failed to read input");
 let age:f32 = age.trim().parse().expect("not an integer");
 println!("your age is {}", age);
    
   
}
