fn main() {
    // Create an empty vector "city"
    let mut city: Vec<String> = Vec::new();

    // Print City Vector
    println!("The City vector has element {}", city.len());

    // Ask how many cities
    let mut input1 = String::new();
    println!("How many cities do you want to enter?");
    std::io::stdin().read_line(&mut input1).expect("Failed to read input");
    let city_num: i32 = input1.trim().parse().expect("Invalid input");

    let mut count = 1; // ← FIX: Define count before the loop

    for _ in 0..city_num {
        let mut input2 = String::new();
        println!("Enter City {}", count);
        std::io::stdin().read_line(&mut input2).expect("Failed to read input");
        let new_city: String = input2.trim().to_string(); // to_string() is enough
        city.push(new_city);
        count += 1; // increment count
    }

    println!("\nYour preferred cities are:\n");

    let mut count = 1;
    for i in city {
        println!("{} {}", count, i);
        count += 1;
    }
}
