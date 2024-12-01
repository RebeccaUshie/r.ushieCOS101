use std::io;

fn main() {
    loop {
        println!("Welcome to Becca Calculates! 
                Here is a list of formulas available for calculation:");
        println!("A - Area of Trapezium");
        println!("B - Area of Rhombus");
        println!("C - Area of Parallelogram");
        println!("D - Area of Cube");
        println!("E - Volume of Cylinder");
        println!("F - End calculations");

        let mut choice = String::new();
          println!("Kindly select the formula of your choice:");
         io::stdin().read_line(&mut choice).expect("Failed to read input");
         let choice = choice.trim();
       

        if choice == "A" {
            trapezium();
        } else if choice == "B" {
            rhombus();
        } else if choice == "C" {
            parallelogram();
        } else if choice == "D" {
            cube();
        } else if choice == "E" {
            cylinder();
        } else if choice == "F" {
            println!("Thank you for working with us
                     We can assure you the answers are accurate
                     Kindly come again next time!
                     Shutting down software");
            break;
        }  else {
            println!("Kindly select from the menu above");
        }

    }
}

fn trapezium() {
    let height: f64 = get_input("Enter height: ").parse().expect("Invalid input for height.");
    let base1: f64 = get_input("Enter base1: ").parse().expect("Invalid input for base1.");
    let base2: f64 = get_input("Enter base2: ").parse().expect("Invalid input for base2.");
    let area = (height / 2.0) * (base1 + base2);
    println!("Awww! Your area of Trapezium is {}", area);
}

fn rhombus() {
    let diagonal1: f64 = get_input("Enter diagonal1: ").parse().expect("Invalid input for diagonal1.");
    let diagonal2: f64 = get_input("Enter diagonal2: ").parse().expect("Invalid input for diagonal2.");
    let area = 0.5 * diagonal1 * diagonal2;
    println!("Awww! Your area of Rhombus is {}", area);
}

fn parallelogram() {
    let base: f64 = get_input("Enter base: ").parse().expect("Invalid input for base.");
    let altitude: f64 = get_input("Enter altitude: ").parse().expect("Invalid input for altitude.");
    let area = base * altitude;
    println!("Awww! Your area of Parallelogram is {}", area);
}

fn cube() {
    let side: f64 = get_input("Enter the side length: ").parse().expect("Invalid input for side.");
    let area = 6.0 * side * side;
    println!(" Awww! Your surface Area of Cube is {}", area);
}

fn cylinder() {
    let radius: f64 = get_input("Enter radius: ").parse().expect("Invalid input for radius.");
    let height: f64 = get_input("Enter height: ").parse().expect("Invalid input for height.");
    let volume = 3.1415 * radius * radius * height;
    println!("Awww! Your volume of Cylinder is {}", volume);
}

fn get_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}