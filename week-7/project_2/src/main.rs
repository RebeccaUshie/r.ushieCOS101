use std::io;

fn main() {
    println!("Hello, How many people do you plan to interview");

    let mut input = String::new();
     io::stdin().read_line(&mut input).expect("Failed to read input");
     let input: u32 = input.trim().parse().expect("Failed to parse input");


     let mut candidates: Vec<(String, u32)> = Vec::new();
    for x in 0..input{
println!("Kindly input your nameee");
let mut name = String::new();
     io::stdin().read_line(&mut name).expect("Failed to read input");
     let name: String = name.trim().parse().expect("    Failed to parse input");

     println!("Kindly input your years of experience");
     let mut experience = String::new();
     io::stdin().read_line(&mut experience).expect("Failed to read input");
     let experience: u32 = experience.trim().parse().expect("Failed to parse input");

     let candidate = (name, experience);
     candidates.push(candidate);
    }

    candidates.sort_by(|a,b| b.1.cmp(&a.1));
        println!("{:?}", candidates);
    println!("Developer with most experience is {} with {} years of experience", candidates[0].0, candidates[0].1);
}
