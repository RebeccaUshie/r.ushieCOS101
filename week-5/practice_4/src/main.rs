fn main() {
    
    let fullname = "Chibudom John Umeh";
    let department = "Computer Science";
    let uni = "Pan Atlantic University";

    let mut school = "School of Science".to_string();
    //push string
    school.push_str("and Technology");

    println!("My name is {}",fullname );
    //check length
    println!("The length of my fullname is {}",fullname.len() );
    println!("I am a Student of {} Department",department );
    println!("{}",school );
    println!("{}",uni );
}
