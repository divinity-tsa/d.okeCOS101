use std::io;

fn main() {
    
    let mut age_input = String::new(); 
    println!("Enter your age:"); 
    io::stdin().read_line(&mut age_input).expect("Failed to read age");

    
    let age: i32 = age_input.trim().parse().expect("Please enter a valid number for age");

    
    let mut experience_input = String::new(); 
    println!("Is the employee experienced? (Y/N)");

    
    io::stdin().read_line(&mut experience_input).expect("Failed to read experience status");
    
    
    let experienced = experience_input.trim().to_uppercase() == "Y"; 

    
    if age >= 40 && experienced { 
        println!("Your incentive: 1_560_000"); 
    } 
    
    else if age >= 30 && age < 40 && experienced { 
        println!("Your incentive: 1_480_000"); 
    } 
    
    else if age <= 28 && experienced { 
        println!("Your incentive: 1_300_000"); 
    } 
    else {
        println!("Your incentive: 100_000"); 
    }
}