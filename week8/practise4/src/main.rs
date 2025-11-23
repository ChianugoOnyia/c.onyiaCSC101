fn main() {
    let name = vec!["Mary", "Sam", "Sally", "Greg"];
    let age = vec![16,17,18,19];
    print!("\n  Age allocation:\n");
    for i in 0..age.len()
    {
        println!("{} is {} years old", name[i], age[i]);
    }
}
