use std::io;
use roots::Roots;
// hi
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
    let clearscreen = "\x1b[2J";
    let mut eq: u32 = 0;
    if eq == 0 {};
       loop {
        println!("Enter a number!\n1: Linear\n2: Quadratic\n3: Cubic\n4: Quartic");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line.");
        option = String::from(option.trim());
        match option.as_str() {
           "1" => { eq = 1; println!("{}Chosen: Linear Equation", {clearscreen}); break },
           "2" => { eq = 2; println!("{}Chosen: Quadratic Equation", {clearscreen}); break },
           "3" => { eq = 3; println!("{}Chosen: Cubic Equation", {clearscreen}); break },
           "4" => { eq = 4; println!("{}Chosen: Quartic Equation", {clearscreen}); break },
           _ => {println!("{option} is not an option!");
                continue}
        };
    }
    
    if eq == 1 {
        let sol = linear();
        println!("Solution to linear equation is: {sol}")
    } else if eq == 2 {
        let sol = quad();
        print_sols(sol);
    } else if eq == 3 {
        let sol = cubic();
        print_sols(sol);
    } else if eq == 4 {
        let sol = quartic();
        print_sols(sol);
    } else { 
        println!("Failed")
    }
}
