use std::io::Read;
use std::io::Write;

fn main() {

    let mut file = std::fs::File::create("PAU_SIMS.txt").expect("Failed to create file");

    let categories = "Student Name    Matric Number    Department    Level\n";

    let sn = vec!["Oluchi Mordi", "Adams Aliyu", "Shania Bolade", "Adekunle Gold",
                 "Bianca Edemoh "];
    let mn = vec!["ACC1021111", "ECO10110101", "CSC10328828", "EEE11020202", "MEE10202001"];
    let dept = vec!["Accounting", "Economics", "Computer", "Electrical", "Mechanical"];
    let lvl = vec!["300", "100", "200", "200", "100"];

    file.write_all(categories.as_bytes()).expect("write failed");


    for i in 0..5 {
        file.write_all(format!("{}       {}      {}    {}\n", sn[i], mn[i], dept[i], lvl[i]).as_bytes() ).expect("write failed");
    };

    println!("Data written successfully\n");

    let mut file = std::fs::File::open("PAU_SIMS.txt").unwrap();
    let mut cont = String::new();
    file.read_to_string(&mut cont).unwrap();
    println!("{}", cont);


}