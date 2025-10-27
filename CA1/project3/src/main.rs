use std::io;
fn main() {
    println!("Code       Item            Price(N)");
    println!(" L         Laptop          550,000");
    println!(" M         Monitor         120,000");
    println!(" K         Keyboard        15,000");
    println!(" H         Headset         25,000");
    
    let mut code = String::new();
    println!("Item code: ");
    io::stdin().read_line(&mut code).expect("Not a valid input");
    

    let mut input2 = String::new();
    println!("Quantity ");
    io::stdin().read_line(&mut input2).expect("Not a valid input");
    let quantity:f32 = input2.trim().parse().expect("Not valid");
    
    

    

  
    if code == "L" {
        let  total_cost:f32 = quantity * 550000.0;
        if total_cost>500000.0 {
            let newcost = total_cost*0.97;
            println!("The total cost is {}", newcost);
        }
        else {
            println!("The total cost is {}", total_cost);
        }
        }

        
    
    else if code == "M" {
        let  total_cost:f32 = quantity * 120000.0;
        if total_cost>500000.0 {
            let newcost = total_cost*0.97;
            println!("The total cost is {}", newcost);
        }
        else {
            println!("The total cost is {}", total_cost);
        }
    }
    else if code == "K" {
        let  total_cost:f32 = quantity * 15000.0;
        if total_cost>500000.0 {
            let newcost = total_cost*0.97;
            println!("The total cost is {}", newcost);
        }
        else {
            println!("The total cost is {}", total_cost);
        }
    }
    else if code == "H" {
        let  total_cost:f32 = quantity * 25000.0;
        if total_cost>500000.0 {
            let newcost = total_cost*0.97;
            println!("The total cost is {}", newcost);
        }
        else  {
            println!("The total cost is {}", total_cost);
        }
     
    }
   


}
