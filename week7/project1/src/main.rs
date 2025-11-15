use std::io;
fn trapezium() {
    let mut input1 = String::new();
    println!("Height(cm): ");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let height:f64 = input1.trim().parse().expect("Not valid");
    let  mut input2 = String::new();
    println!("Base 1(cm): ");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let base_1:f64 = input2.trim().parse().expect("Not valid");
    let  mut input3 = String::new();
    println!("Base 2(cm):");
    io::stdin().read_line(&mut input3).expect("Not valid");
    let base_2:f64 = input3.trim().parse().expect("Not valid");

    let area:f64 = (height/2.0)*(base_1+base_2);
    println!("The area of the trapezium is: {:.2}cm^2",area );


}
fn rhombus() {
    let  mut input1 = String::new();
     println!("Diagonal 1(cm):");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let diagonal_1:f64 = input1.trim().parse().expect("Not valid");
    let mut input2 = String::new();
     println!("Diagonal 2(cm):");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let  diagonal_2:f64 = input2.trim().parse().expect("Not valid");
   
    let area:f64 = 0.5*diagonal_1*diagonal_2;
    println!("The area of the rhombus is: {:.2}cm^2",area );


}
fn parallelogram() {
    let mut input1 = String::new();
     println!("Base(cm):");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let base:f64 = input1.trim().parse().expect("Not valid");
    
    let mut  input2 = String::new();
     println!("Altitude(cm):");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let altitude:f64 = input2.trim().parse().expect("Not valid");

    let area:f64 = base*altitude;
    println!("The area of the parallelogram is: {:.2}cm^2",area );


}
fn cube() {
    let mut input1 = String::new();
     println!("Lenght of side(cm):");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let lenght:f64 = input1.trim().parse().expect("Not valid");
    
    let area:f64 = 6.0*(lenght.powf(2.0));
    println!("The area of the cube is: {:.2}cm^2",area );
}
fn cylinder() {
    let mut input1 = String::new();
     println!("Radius(cm):");
    io::stdin().read_line(&mut input1).expect("Not valid");
    let radius:f64 = input1.trim().parse().expect("Not valid");
    
    let mut input2 = String::new();
     println!("Height(cm):");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let height:f64 = input2.trim().parse().expect("Not valid");

    let volume:f64 = (22.0/7.0)*height*(radius.powf(2.0));
    println!("The volume of the cylinder is: {:.2}cm^3",volume );

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

    if code == "A" || code =="a"{
        trapezium();
    }
    else if code == "B" || code=="b" {
        rhombus();
        }
    else if code== "C" || code =="c"{
        parallelogram();
    }
    else if code=="D" || code =="d"{
    cube();
}
    else if code=="E" || code=="e"{
    cylinder();
}
    else {
        println!("Not a valid code");
    }
    loop{
    let mut inputa = String::new();
    println!("Are you done?(y/n); ");
    io::stdin().read_line(&mut inputa).expect("Not valid");
    let answer = inputa.trim();
     if answer == "y" || answer == "Y"{
        return;
    }
    else if answer == "n" || answer == "N"{
        break;
    }
    else {
        println!("Not a valid answer");
    }
}

}


}
