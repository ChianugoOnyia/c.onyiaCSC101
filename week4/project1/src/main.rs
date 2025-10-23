use std::io;
fn main() {
    let mut input1 = String::new();
    println!("Value of a :");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let  a:f32 = input1.trim().parse().expect("Not valid");

let mut input2 = String::new();
    println!("Value of b :");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let  b:f32 = input2.trim().parse().expect("Not valid");


let mut input3 = String::new();
    println!("Value of c :");
    io::stdin().read_line(&mut input3).expect("Not valid");
    let  c:f32 = input3.trim().parse().expect("Not valid");
let power = 2.0;
    let discriminant:f32 = (b.powf(power))-(4.0*a*c);
    if discriminant > 0.0 {
        let root1:f32 = (-b + (discriminant.sqrt()))/(2.0*a);
        let root2:f32 = (-b - (discriminant.sqrt()))/(2.0*a);
            println!("The roots of the equation are {} and {}",root1, root2 );


    }
    else if discriminant == 0.0{
        let root1:f32 = (-b + (discriminant.sqrt()))/(2.0*a);
            println!("The root of the equation is {} ",root1 );

    }
    else {
        println!("This is not a valid quadratic equation");
    }
    
}
