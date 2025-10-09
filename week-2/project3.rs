fn main() {
	let p:f64 = 510000.00;
	let d:f64 = 5.0;
	let years:f64 = 3.0;
	let value:f64 = p*(1.0-(d/100.0)).powf(years);
	println!("The new value is N{:.2}", value);
}