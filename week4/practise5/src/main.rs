use std::io;

fn main() {
    let mut input = String::new();

    println!("\nEnter your height in cm:");
    io::stdin().read_line(&mut input).expect("not valid");
    let height:f32 = input.trim().parse().expect("Not valid");

    if height >= 150.0 && height <=170.0 {
        println!("You are an average height person");

    }
    else if height >170.0 && height <= 195.0{
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0 
    {
        println!("You are a dwarf");

    }
    else {
        println!("Abnormal height");
    }
}
