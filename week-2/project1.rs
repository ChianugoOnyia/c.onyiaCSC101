fn  main() {
	
	let principal:f64 = 520000000.00;
	let rate:f64 = 10.00;
	let years:f64 = 5.0;
	let amount:f64 = principal * (1.0 + (rate/100.0)).powf(years);
	let i:f64 = amount-principal;
	println!("The interest after 5 years is N{:.2}", i);

}
