// code for calculating average and grade
use std::io;
fn main() {
    let mut name = String::new();
    println!("What is your name? ");
    io::stdin().read_line(&mut name).expect("Not a valid input"); //Code for name

    let mut input1 = String::new();
    println!("Score for CA1: ");
    io::stdin().read_line(&mut input1).expect("Not a valid input");       
    let score_1:f32 = input1.trim().parse().expect("Not a valid input");

    let mut input2 = String::new();
    println!("Score for CA2: ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let score_2:f32 = input2.trim().parse().expect("Not a valid input");

    let mut input3 = String::new();
    println!("Score for CA3: ");
    io::stdin().read_line(&mut input3).expect("Not a valid input");
    let score_3:f32 = input3.trim().parse().expect("Not a valid input"); //Code for test scores input
  
    let average:f32 = (score_1 + score_2 + score_3)/3.0; //average
    if average>=70.0 && average<=100.0 {
        let grade = 'A';
        println!("{} has a an average of {:.2} and a grade of {}",name, average,grade);
    }
    else if average>=60.0 && average<70.0 {
       let grade = 'B';
       println!("{} has a an average of {:.2} and a grade of {}",name, average,grade);
   }
    else if average>=50.0 && average<60.0 {
        let grade = 'C';
        println!("{} has a an average of {:.2} and a grade of {}",name, average,grade);
    }
    else if average>=45.0 && average<50.0 {
        let grade = 'D';
        println!("{} has a an average of {:.2} and a grade of {}",name, average,grade);
    }
    else if average>=0.0 && average<45.0 {
       let grade = 'F';
       println!("\n{} has a an average of {:.2} and a grade of {}",name, average,grade);
    }
    else {
        println!("Not a valid average! Recheck your scores");
    }
    

}
