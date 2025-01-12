struct Laptops {
    name:String,
    price: i32,
    quantity:u32
}
fn main() {
    let Hp = Laptops {
        name:String::from("Hp"),
        price:650000,
        quantity:10
 };
    let IBM = Laptops {
        name:String::from("IBM"),
        price:755000,
        quantity:6

    };
    let Toshiba = Laptops {
        name:String::from("Toshiba"),
        price:550000,
        quantity:10
};
   let Dell = Laptops {
        name:String::from("Dell"),
        price:850000,
        quantity:4
};


    let sum = Hp.price*3 + IBM.price*3 + Toshiba.price*3 + Dell.price*3;
    println!("Your total cost is {}",sum );
    
}
