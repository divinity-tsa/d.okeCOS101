use std::io; 
use std::f64; 

fn main() {
    println!(" Quadratic Equation Solver");

    // --- Input for Coefficient 'a' ---
    let mut a_input = String::new();
    println!("Enter the coefficient 'a':");
    io::stdin().read_line(&mut a_input).expect("Failed to read line for 'a'");
    let a: f64 = a_input.trim().parse().expect("Please enter a valid number for 'a'");

    
    // --- Input for Coefficient 'b' ---
    let mut b_input = String::new();
    println!("Enter the coefficient 'b':");
    io::stdin().read_line(&mut b_input).expect("Failed to read line for 'b'");
    let b: f64 = b_input.trim().parse().expect("Please enter a valid number for 'b'");

    // --- Input for Coefficient 'c' ---
    let mut c_input = String::new();
    println!("Enter the coefficient 'c':");
    io::stdin().read_line(&mut c_input).expect("Failed to read line for 'c'");
    let c: f64 = c_input.trim().parse().expect("Please enter a valid number for 'c'");
    
    

    let discriminant: f64 = b * b - 4.0 * a * c;

    // A variable to store the denominator (2a)
    let two_a: f64 = 2.0 * a;
   if a == 0.0 {
        println!("\nError: The coefficient 'a' cannot be zero for a quadratic equation.");
        
    } 
    
    else if discriminant > 0.0 {
        println!("\nTwo distinct real roots (Discriminant > 0):");
        
        let sqrt_final: f64 = discriminant.sqrt();

        // Calculate the two roots
        let root1: f64 = (-b + sqrt_final) / two_a;
        let root2: f64 = (-b - sqrt_final) / two_a;

        println!("Root 1: {}", root1);
        println!("Root 2: {}", root2);
    } 
    
    else if discriminant == 0.0 {
        println!("\nExactly one real root (Discriminant = 0):");

        
        let root: f64 = -b / two_a;

        println!("Root (x): {}", root);
    } 
    
    else { 
        println!("\nNo real roots (Discriminant < 0):");
        
    }
}
