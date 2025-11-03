use std::io;

fn main() {
    // Declare mutable strings for raw input
    let mut grade_point_input = String::new();
    let mut credits_input = String::new();

    // Variables to hold the final numerical values (inferred as f32)
    let grade_point;
    let credits;

    println!("--- Pure Variable Assignment GPA Input ---");
    println!("(Please enter numerical values: e.g., 4.0 for A, 3.0 for B)");

    // 1. Get Grade Point Input (Numerical Value)
    print!("Enter Grade Point Value: ");
    io::stdin().read_line(&mut grade_point_input).expect("Failed to read grade point");
    
    // Convert the input string to an f32. If parsing fails, use 0.0 as the default.
    grade_point = grade_point_input
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);
    
    // 2. Get Credit Hours Input
    print!("Enter credit hours for the course: ");
    io::stdin().read_line(&mut credits_input).expect("Failed to read credits");
    
    // Convert the input string to an f32. If parsing fails, use 0.0 as the default.
    credits = credits_input
        .trim()
        .parse::<f32>()
        .unwrap_or(0.0);

    // Perform the core calculation based on the assigned variables
    let course_points: f32 = grade_point * credits;

    // Output the assigned variables and the result
    println!("\n--- Assigned and Calculated Variables ---");
    println!("Assigned Grade Point: {:.1}", grade_point);
    println!("Assigned Credits: {:.1}", credits);
    println!("Course Grade Points Earned: {:.2}", course_points);
}