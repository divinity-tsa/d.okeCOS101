struct Laptops {
    brand:String,
    quantity:u32,
    price:u32
}





fn main() {
    let class1 = Laptops {
        brand:String::from("HP"),
        quantity:10,
        price:650_000
    };
    let class2 = Laptops {
        brand:String::from("IBM"),
        quantity:6,
        price:755_000
    };
    let class3 = Laptops {
        brand:String::from("Toshiba"),
        quantity:4,
        price:850_000
    };


    let cost1 = class1.price * 3;
    let cost2 = class2.price * 3;
    let cost3 = class3.price * 3;

    println!("Welcome to Ogbeifuna Electronics Shops\n");
    println!("Supposing a customer purchases 3 Laptops from each brand...\n");
    println!("3 {} laptops: {}\n", class1.brand,cost1);
    println!("3 {} laptops: {}\n",class2.brand,cost2);
    println!("3 {} laptops: {}\n",class3.brand,cost3);

}