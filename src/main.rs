use std::io;

fn main() {

println!("rTaschenrechner 0.10\n");

// init variables
let mut z1 = String::new(); // Erste Zahl
let mut z2 = String::new(); // Zweite Zahl


// debug, hide it when you release it
// println!("z1: {}", z1);
// println!("z2: {}", z2);


// type in first number
println!("Gebe die erste Zahl ein!");

io::stdin()
    .read_line(&mut z1)
    .expect("Konnte erste Zahl nicht erkennen.");

let y1: i32 = z1
    .trim()
    .parse()
    .expect("Konnte erste Zahl nicht parsen.");

// debug, hide it when you release it
// println!("z1: {}", z1);
// println!("z2: {}", z2);
// println!("y1: {}", y1);

// type in second numer
println!("\nGebe die zweite Zahl ein!");

io::stdin()
    .read_line(&mut z2)
    .expect("Konnte zweite Zahl nicht erkennen.");

let y2: i32 = z2
    .trim()
    .parse()
    .expect("Kontte zweite Zahl nicht parsen.");

// calculate this shit

let x1: i32 = y1 + y2;

println!("\nIhr Ergebnis ist {}", x1);

// debug, hide it when you release it
// println!("z1: {}", z1);
// println!("y1: {}", y1);
// println!("z2: {}", z2);
// println!("y2: {}", y2);
// println!("x1: {}", x1);






println!("\nDanke!");
}
