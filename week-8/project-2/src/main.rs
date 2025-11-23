fn main() {
    let developers = vec![("Valerie", 2), ("Semire", 3), ("Divine", 5),("Bronx", 8), ("Esther", 6),];

    let mut highest_name = "";
    let mut highest_years = 0;

    for dev in developers {
        let (name, years) = dev;

        if years > highest_years {
            highest_years = years;
            highest_name = name;
        }
    }

    println!("Most experienced developer is {} with {} years. ", highest_name, highest_years);
}
