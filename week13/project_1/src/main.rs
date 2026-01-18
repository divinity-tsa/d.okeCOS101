use std::io::Read;


fn open_file(x:&str) {
    let mut file = std::fs::File::open(x).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("{}", contents);


}

fn main() {

    loop{
        let mut input = String::new();
        println!("Enter Status...\n
                  1) Administrator\n
                  2) Project Manager\n
                  3) Employee\n
                  4) Customer\n 
                  5) Vendor\n 
                  0) Exit\n 
                  Select a number from 1 - 5");
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        let status: i8 = input.trim().parse().expect("Not an integer. Please try again");
        match status {
            1 => open_file("globalcom_db.sql"),
            2 => open_file("project_tb.sql"),
            3 => open_file("staff_tb.sql"),
            4 => open_file("customer_tb.sql"),
            5 => open_file("dataplan_tb.sql"),
            0 => break,
            _ => {
                println!("Invalid status entered");
                continue;
            }
        }
    }
}