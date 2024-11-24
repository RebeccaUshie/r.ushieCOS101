use std::io;

fn main() {
    let mut total = 0.0;
    loop {
        println!("Welcome to Big Becks Bukka!");
        println!("Our menu includes:");
        println!("
    P - Pounded Yam/Edikainko Soup -#3200
    F - Fried rice and Chicken  -#3000
    A - Amala and Ewedu Soup  -#2500
    E - Eba and Egusi Soup -#2000
    W - White rice and Stew -#2500", );

         let mut input = String::new();
         println!("Choose item from the menu (Kindly type I'm fine if done ordering):");
         io::stdin().read_line(&mut input).expect("Failed to read input");
         let item = input.trim();

         if item == "I'm fine" {
            break;
         }

         println!("You picked {}", item);
         println!("Enter quantity");
         let mut input = String::new();
         io::stdin().read_line(&mut input).expect("Failed to read input");
         let quantity: f32 = input.trim().parse().expect("Failed to parse input");

         let mut price:f32 = 0.0;

         if item == "P" {
            price = 3200.0;
         } else if item == "F" {
            price = 3000.0;
         } else if item == "A"{
            price = 2500.0;
         } else if item == "E"{
            price = 2000.0;
         } else {
            price = 2500.0;
         }
        let subtotal = price*quantity;
        total += subtotal;
    }

    if total > 10_000.0 {
       total = total*0.95;
       println!("Congratulations! You have a 5% discount.");
    }
println!("Your total order is {}",total );
println!("Thank you for shopping with us!");

}
