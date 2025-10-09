fn main() {
	let t:f64 = 2.0 * 450000.0;
	let m:f64 = 1500000.0;
    let h:f64 = 3.0 * 750000.0;      // variables given to the sales record
    let d:f64 = 3.0 * 2850000.0;
    let a:f64 = 250000.0;
    let n:f64 = 2.0 + 1.0 + 3.0 + 3.0 + 1.0;

	// sum of the sales record

	let sum = t + m + h + d + a;
	println!("The sum of the following sales record is {}", sum);
     
     // average of the sales record
	let average = sum / n;
	println!("The average of the following sales record is {}", average);

}