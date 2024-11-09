use std::io;

fn main() {
    //Input values for a , b and c
    let mut inputa = String::new();
    let mut inputb = String::new();
    let mut inputc = String::new();

    println!("Enter the value for a");
    io::stdin().read_line(&mut inputa).expect("Not a valid string");
    let a:f64 =inputa.trim().parse().expect("Not a valid number");

    println!("Enter the value for b");
    io::stdin().read_line(&mut inputb).expect("Not a valid string");
    let b:f64 =inputb.trim().parse().expect("Not a valid number");

    println!("Enter the value for c");
    io::stdin().read_line(&mut inputc).expect("Not a valid string");
    let c:f64 =inputc.trim().parse().expect("Not a valid number");

//Calculation for discriminant
let discriminant = b * b - 4.0 * a * c;

if discriminant > 0.0 
{
    println!("Two distinct roots: {} and {}",(-b + discriminant.sqrt())/(2.0 * a), (-b - discriminant.sqrt())/2.0 * a );
}
else if discriminant == 0.0
{
    println!("Exactly one real root:{}",-b / (2.0 * a) );
}
else 
{
    println!("No real roots");
}
 
}
