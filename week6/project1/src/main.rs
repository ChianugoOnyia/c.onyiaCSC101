use std::io;

fn main() {
    let mut total = 0.0;

    loop {
        println!("Code          Menu                  Price");
        println!("P       Poundo Yam/Edinkaiko Soup   N3200");
        println!("F       Fried Rice & Chicken        N3000");
        println!("A       Amala & Ewedu Soup          N2500");
        println!("E       Eba & Egusi Soup            N2000");
        println!("W       White Rice & Stew           N2500");

        // Get product code
        println!("Enter product code: ");
        let mut code = String::new();
        io::stdin().read_line(&mut code).expect("Not valid");
        let code = code.trim();

        // Get quantity
        println!("Enter quantity: ");
        let mut quantity = String::new();
        io::stdin().read_line(&mut quantity).unwrap();
        let qty: f64 = quantity.trim().parse().expect("Not valid");

        // Find price using simple if statements
        let mut cost = 0.0;
        if code == "P" || code == "p" {
            cost = 3200.0 * qty;
        } else if code == "F" || code == "f" {
            cost = 3200.0 * qty;
        } else if code == "A" || code == "a" {
            cost = 2500.0 * qty;
        } else if code == "E" || code == "e" {
            cost = 2000.0 * qty;
        } 
        else if code == "W" || code == "w" {
            cost = 2500.0 * qty;
        }else {
            println!("Invalid product code!");
            continue;
        }

        // Apply discount if cost > ₦500,000
        if cost > 10000.0 {
            println!("You got a discount of 5% ");
            cost = cost * 0.95;
        }

        println!("Subtotal for this item: N{:.2}", cost);
        total += cost;

        // Ask if they want to continue
        println!("Do you want to buy another item? (y/n): ");
        let mut again = String::new();
        io::stdin().read_line(&mut again).unwrap();

        if again.trim() == "n" || again.trim() == "N" {
            break;
        }
    }

    println!("\nYour total bill is ₦{:.2}", total);
}

