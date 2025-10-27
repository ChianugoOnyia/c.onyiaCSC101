use std::io;
fn main() {
    let mut input1 = String::new();
    println!("How much did you deposit: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");
    let p:f32 = input1.trim().parse().expect("Not valid");

    let mut input2 = String::new();
    println!("Interest rate: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let r:f32 = input2.trim().parse().expect("Not valid");

    let mut input3 = String::new();
    println!("Number of years: ");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let t:u32 = input3.trim().parse().expect("Not valid");

    let amount:f32 = p*((1.0+(r/100.0)).powf(t as f32));
    let compound_interest:f32 = amount - p;

    println!("At the end of {} years, the compound interest is N{:.2} and the amount is N{:.2}",t, compound_interest, amount );



}
