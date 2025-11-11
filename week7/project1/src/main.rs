use std::io;
fn trapezium() {
    let mut input1 = String::new();
    println!("Height: ");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let height:f64 = input1.trim().parse().expect("Not valid");
    let  mut input2 = String::new();
    println!("Base 1: ");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let base_1:f64 = input2.trim().parse().expect("Not valid");
    let  mut input3 = String::new();
    println!("Base 2 :");
    io::stdin().read_line(&mut input3).expect("Not valid");
    let base_2:f64 = input3.trim().parse().expect("Not valid");

    let area:f64 = (height/2.0)*(base_1+base_2);
    println!("The area of the trapezium is: {}",area );


}
fn rhombus() {
    let  mut input1 = String::new();
     println!("Diagonal 1 :");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let diagonal_1:f64 = input1.trim().parse().expect("Not valid");
    let mut input2 = String::new();
     println!("Diagonal 2 :");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let  diagonal_2:f64 = input2.trim().parse().expect("Not valid");
   
    let area:f64 = 0.5*diagonal_1*diagonal_2;
    println!("The area of the rhombus is: {}",area );


}
fn parallelogram() {
    let mut input1 = String::new();
     println!("Base  :");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let base:f64 = input1.trim().parse().expect("Not valid");
    
    let mut  input2 = String::new();
     println!("Altitude :");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let altitude:f64 = input2.trim().parse().expect("Not valid");

    let area:f64 = base*altitude;
    println!("The area of the parallelogram is: {}",area );


}
fn cube() {
    let mut input1 = String::new();
     println!("Lenght of side  :");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let lenght:f64 = input1.trim().parse().expect("Not valid");
    
    let area:f64 = 6.0*(lenght.powf(2.0));
    println!("The area of the cube is: {}",area );
}
fn cylinder() {
    let mut input1 = String::new();
     println!("Radius:");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let radius:f64 = input1.trim().parse().expect("Not valid");
    
    let mut input2 = String::new();
     println!("Height:");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let height:f64 = input2.trim().parse().expect("Not valid");

    let volume:f64 = (22.0/7.0)*height*(radius.powf(2.0));
    println!("The volume of the cylinder is: {}",volume );

}

fn main() {
     println!("\nEquation codes:");
        println!("A - Area of trapezium");
        println!("B - Area of Rhombus");
        println!("C - Area of parallelogram");
        println!("D - Area of Cube)");
        println!("E - Volume of Cylinder");

        loop {
            let mut input = String::new();
            println!("Equation code: ");
    io::stdin().read_line(&mut input).expect("Not valid");
    let code = input.trim();

    if code == "A"{
        trapezium();
    }
    else if code == "B" {
        rhombus();
        }
    else if code== "C"{
        parallelogram();
    }
    else if code=="D"{
    cube();
}
    else if code=="E"{
    cylinder();
}
    else {
        println!("Not a valid code");
    }
    let mut inputa = String::new();
    println!("Are you done?(y/n); ");
    io::stdin().read_line(&mut inputa).expect("Not valid");
    let answer = inputa.trim();
     if answer == "y" || answer == "Y"{
        break;
}
}


}
