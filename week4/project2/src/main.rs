use std::io;
fn main() {
    let mut input1 = String::new();
    println!("How many years have you worked: ");
    io::stdin().read_line(&mut input1).expect("not valid");
    let years:u32 = input1.trim().parse().expect("not valid");

    if years >= 10  {
        
        let mut input2 = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut input2).expect("Not valid");
        let age:u32 = input2.trim().parse().expect("Not valid");

        if age >= 40 && age > years {
            println!("Experienced");
            let incentive_1 = 1560000;
            println!("The incentive is N{} ", incentive_1);
        }
        else if age >= 30 && age <40  && age > years {
            println!("Experienced");
            let incentive_1 = 1480000;
            println!("The incentive is N{} ", incentive_1);
        }
        else if age <30 && age > years {
            println!("Experienced");
            let incentive_1 = 1300000;
            println!("The incentive is N{} ", incentive_1);
        }
        else {
            println!("Not valid because your age is less than your number of years");
            }
        }
        
        else if years< 10 {
            let mut input2 = String::new();
        println!("Age: ");
        io::stdin().read_line(&mut input2).expect("Not valid");
        let age:u32 = input2.trim().parse().expect("Not valid");
        if age > years {

            println!("Not experienced");
            let incentive_1 = 100000;
            println!("The incentive is N{} ", incentive_1);
        }
        else {
            println!("Not valid because your age is less than your number of years");
        }

        }




        
}
