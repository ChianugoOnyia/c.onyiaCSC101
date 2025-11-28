use std::io::Write;

fn main() {
    let line_1 = "Lager        Stout             Non-Alcoholic\n";
    let line_2 = "33 Export    Legend            Maltina\n";
    let line_3 = "Desperados   Turbo King        Amstel Malta\n";
    let line_4 = "Goldberg     William           Malta Gold\n";
    let line_5 = "Gulder                         Fayrouz\n";
    let line_6 = "Heineken\n";
    let line_7 = "Star\n";


    let mut file = std::fs::File::create("Nigerian_Breweries.txt").expect("Create Failed");
    file.write_all("Nigerian Breweries Portfolio\n".as_bytes()).expect("Write failed");
     file.write_all(line_1.as_bytes()).expect("Write failed");
     file.write_all(line_2.as_bytes()).expect("Write failed");
     file.write_all(line_3.as_bytes()).expect("Write failed");
     file.write_all(line_4.as_bytes()).expect("Write failed");
     file.write_all(line_5.as_bytes()).expect("Write failed");
     file.write_all(line_6.as_bytes()).expect("Write failed");
     file.write_all(line_7.as_bytes()).expect("Write failed");
     println!("\nData written to file."); 

}