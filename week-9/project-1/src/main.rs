use std::io::Write;
use std::fs;

fn main() {
    

    

    let categories = "Nigerian Brewries Limited\n

    LAGER     STOUT       NON-ALCOHOLIC\n
    33 Export   |   Legend     |   Maltina\n
    Desperados  |   Turbo King |   Amstel Malta\n
    Goldberg    |   Williams   |   Malta Gold\n
    Gulder      |              |   Fayrouz\n
    Heineken    |              | 
    Star        |              |
                       





                    ";
let mut file = std::fs::File::create("high_quality_drinks.txt")
    .expect("create failed");
    
    file.write_all(categories.as_bytes()).expect("write failed");



println!("File successfully created");

}
