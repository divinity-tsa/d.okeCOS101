use std::io::Read;
use std::io::Write;

fn main(){

    let mut file = std::fs::File::create("Convicted_Ministers").expect("Failed to create file");

    file.write_all("S/N     NAME OF COMMISIONER         MINISTRY          GEOPOLITICAL ZONE\n".as_bytes()).expect("write failed");


let sn = vec!["1", "2", "3", "4", "5"];
let name = vec!["Aigbogun Alamba Daudu", "Murtala Afeez Bendu ", "Okorocha Calistus Ogbona",
 "Adewale Jimoh Akanbi", "Osazuwa Faith Etieye"];
 let min = vec!["   Internal Affairs", "    Justice", "Defense", "    Power & Steel", "    Petroleum"];
 let geo = vec!["South West", "         North East", "         South South", "   South West", "       South East"];


 for i in 0..5 {
    file.write_all(format!("{}      {}     {}  {}\n", sn[i], name[i], min[i], geo[i]).as_bytes()).expect("write failed");

 }

 let mut file = std::fs::File::open("Convicted_Ministers").unwrap();
 let mut cont = String::new();
 file.read_to_string(&mut cont).unwrap();
 println!("{}", cont);

}