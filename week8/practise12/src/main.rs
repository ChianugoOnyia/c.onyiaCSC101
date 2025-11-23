fn main() {
    let mut colors = ["red", "green", "yellow", "white"];
    println!("\n Original array = {:?}",colors );
    let sliced = &mut colors[1..3];

    println!("First slice = {:?}",sliced );
    sliced[1] = "purple";
    println!("Changed slice = {:?}", sliced);
}
