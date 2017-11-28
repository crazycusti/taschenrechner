use std::io;

fn main() {

println!("rTaschenrechner 0.10\n");

// init variables
let mut z1 = String::new();


//debug
println!("z1: {}", z1);
// println!("z2: {}", z2);
// println!("z3: {}", z3);

// type in first number
println!("Gebe die erste Zahl ein!\n");

io::stdin()
    .read_line(&mut z1)
    .expect("Konnte Zahl nicht erkennen.");

let y1: i32 = z1
    .trim()
    .parse()
    .expect("Konnte Zahl nicht parsen.");

//debug
println!("z1: {}", z1);
// println!("z2: {}", z2);
// println!("z3: {}", z3);

println!("Danke!");
}
