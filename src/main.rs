use std::io;
use roots::Roots;
use clearscreen;

enum EquationType {
    Linear,
    Quadratic,
    Cubic,
    Quartic
}

fn vars(name: &str) -> f64 {
    loop {let mut var = String::new();
        println!("Enter a value for {name}:");
        io::stdin().read_line(&mut var).expect("Failed to read line");
        let trimmed_var = var.trim();
        match trimmed_var.parse::<f64>() {
            Ok(i) => return i,
            Err(..) => {println!("Not a viable input!"); continue}
        }
    }
}

fn linear() -> f64 {
    let b = vars("b");
    let m = vars("m");
    -b/m
}

fn quad() -> Roots<f64> {
    let a = vars("a");
    let b = vars("b");
    let c = vars("c");
    roots::find_roots_quadratic(a, b, c)
}

fn cubic() -> Roots<f64> {
    let a = vars("a");
    let b = vars("b");
    let c = vars("c");
    let d = vars("d");
    roots::find_roots_cubic(a, b, c, d)
}

fn quartic() -> Roots<f64> {
    let a = vars("a");
    let b = vars("b");
    let c = vars("c");
    let d = vars("d");
    let e = vars("e");
    roots::find_roots_quartic(a, b, c, d, e)
}

fn print_sols(sol: Roots<f64>) {
    let mut i = 1;
    match sol {
        Roots::Four(sol) => for solution in sol {println!("Solution {}: {}", {i}, {solution}); i += 1},
        Roots::Three(sol) => for solution in sol {println!("Solution {}: {}", {i}, {solution}); i += 1},
        Roots::Two(sol) => for solution in sol {println!("Solution {}: {}", {i}, {solution}); i += 1},
        Roots::One(sol) => for solution in sol {println!("Solution {}: {}", {i}, {solution}); i += 1},
        _ => println!("No (Non-Complex) result.")
        }   
}

fn main() {
    let eq = loop {
        println!("Enter a number!\n1: Linear\n2: Quadratic\n3: Cubic\n4: Quartic");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line.");
        match &*option.trim(){
           "1" => {clearscreen::clear().expect("failed to clear screen");println!("Chosen: Linear Equation"); break EquationType::Linear},
           "2" => {clearscreen::clear().expect("failed to clear screen"); println!("Chosen: Quadratic Equation"); break EquationType::Quadratic},
           "3" => {clearscreen::clear().expect("failed to clear screen"); println!("Chosen: Cubic Equation"); break EquationType::Cubic},
           "4" => {clearscreen::clear().expect("failed to clear screen"); println!("Chosen: Quartic Equation"); break EquationType::Quartic},
           _ => {println!("{option} is not an option!");
                continue}
        };
    };
    
    match eq {
        EquationType::Linear => { let sol = linear();
            println!("Solution to linear equation is: {sol}")},
        EquationType::Quadratic => {let sol = quad();
            print_sols(sol) }
        EquationType::Cubic => {let sol = cubic();
            print_sols(sol);}
        EquationType::Quartic => {let sol = quartic();
            print_sols(sol);}
    }
}
