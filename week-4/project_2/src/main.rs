use std::io;
fn main() {
     
     let mut input1 = String::new();
     let mut input2 = String::new();

     println!("Enter your age");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let age:i32= input1.trim().parse().expect("Not a valid number");

   println!("Expereienced?");
   io::stdin().read_line(&mut input2).expect("Not a valid string");
   let experience:bool= input2.trim().parse().expect("Not a valid boolean");

   if experience && age>=40
   {
    println!("Incentive = N1,560,000.");
   } 
   else if experience && age>=30 && age <=40
   {
    println!("Incentive = N1,480,000.");
   }
  else if experience && age <28 
  {
    println!("Incentive = N100,000");
  }

}
