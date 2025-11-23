use std::io;
fn main() {
    let mut array1 = Vec::new();
    let mut array2 = Vec::new();

    loop {
    let mut input1 = String::new();
    println!("Fullname: ");
    io::stdin().read_line(&mut input1).expect("Not valid");
    
    array1.push(input1);

    
    let mut input2 = String::new();
    println!("Years:  ");
    io::stdin().read_line(&mut input2).expect("Not valid");
    let years:u32 = input2.trim().parse().expect("Not valid");
    array2.push(years);

    
    for i in 0..array1.len() {
        println!("Name:{} ",array1[i] );
        println!("Years:{}",array2[i] );
        println!("");

    }
    let max = array2.iter().max();
    /*for m in 0..array2.len() {
        if array2[m] = max {
            let index = m;
        }*/
    

    println!("Person with max number of years:" );
    
    println!("Highest number of years:{:?}",max );

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
    
    }}




