use std::io;

fn main () {
      
      println!("*****Public Service APS Level Checker*****");

 
let office_administrator = vec!["Intern", "Administrator", "Senior Administrator",
                             "Office manager", "Director", "CEO"];
 let academic = vec!["-", "Research Assistant", "PhD Canditate", "Post-Doc Researcher",
                 "Senior lecturer", "Dean"];
let lawyer = vec!["Paralegal", "Junior Associate", "Associate", "Senior Associate 1-2", 
               "Senior Associate 3-4", "Partner"];
 let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher",
                "Deputy Principal", "Principal"]; 
let position = vec!["APS 1-2", "APS 3-5", "APS 5-8", "EL1 8-10", "EL2 10-13", "SES"];


 let mut path_choice = String::new();
 println!("\nSelect your Career Path:\n1. ADMIN \n2. ACADEMIC \n3. LAWYER \n4. TEACHER");
 io::stdin().read_line(&mut path_choice).expect("Failed to read input");

 let chosen_roles = match path_choice.trim() {
    "1" | "ADMIN" => &office_administrator,
    "2" | "ACADEMIC" => &academic,
     "3"| "LAWYER" => &lawyer,
    "4" | "TEACHER" => &teacher,
    _ => {
        println!("Invalid path. ");
        return;
    }


     };

let mut exp = String::new();
println!("Enter years of experience(0-5):");
io::stdin().read_line(&mut exp).expect("Failed to read input");
let experience: u32 = exp.trim().parse().expect("Invalid input");

let index: usize = if experience >= 5 { 5 } else {experience as usize};


let expected_app_level = match position.get(index) {
    Some(level) => *level,
    None => "UNKNOWN",

};

let expected_role_title = match chosen_roles.get(index) {

    Some(role) => *role,
    None => "UNKNOWN",
};

println!("---------Results---------");
println!("Experience entered: {} year(s)",experience);
println!("APS Level: {}",expected_app_level);
println!("Role Title: {}",expected_role_title);


















}