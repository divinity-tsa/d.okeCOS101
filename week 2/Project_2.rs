fn main() {
	let t1:f64 = 450_000.00;
	let t2:f64 = 450_000.00;
	let m1:f64 = 1_500_000.00;
    let h1:f64 = 750_000.00;      // variables given to the sales record
    let h2:f64 = 750_000.00;
    let h3:f64 = 750_000.00;
    let d1:f64 = 2_850_000.00;
    let d2:f64 = 2_850_000.00;
    let d3:f64 = 2_850_000.00;
    let a1:f64 = 250_000.00;

	// sum of the sales record

	let sum = t1 + t2 + m1 + h1 + h2 + h3 + d1 + d2 + d3 + a1
	println!("The sum of the following sales record is {}", sum);
     
     // average of the sales record
	let average = sum / 10;
	println!("The average of the following sales record is {}", average);
}