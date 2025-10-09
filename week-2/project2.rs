 fn main() {
	let n:f64 = 5.0;
	let t:f64 = 2.0 *450000.00;
	let m:f64 = 1500000.00;
	let h:f64 = 3.0*750000.00;
	let d:f64 = 3.0*2850000.00;
	let a:f64 = 250000.00;
	let sum:f64 = n+t+m+h+d+a;
	let average:f64 = sum/n;
	println!("The sum is N{:.2}", sum);
	println!("The average is N{:.2}", average);

}