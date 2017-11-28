use std::io;
use std::ops::{Add, Div, Mul, Sub};


fn main() {

println!("rTaschenrechner 0.1.1\n");

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

// type in operand

println!("\nGebe den Operanden ein. Erlaubte Zeichen: '+' '-' '*' '/'");

let mut z3 = String::new();
io::stdin()
    .read_line(&mut z3)
    .expect("Konnte Operanden nicht erkennen.");

z3.truncate(1); // remove whitespaces and other shitty signs

// debug, hide it when you release it
//println!("bla {:?}", z3);
//println!("bla {:?}", &*z3);
//println!("bla {:?}", y3);


// type in second numer
println!("\nGebe die zweite Zahl ein!");

io::stdin()
    .read_line(&mut z2)
    .expect("Konnte zweite Zahl nicht erkennen.");

let y2: i32 = z2
    .trim()
    .parse()
    .expect("Konnte zweite Zahl nicht parsen.");

// calculate this shit

let x2 = match &*z3 {
    "+" => Add::<i32>::add,
    "-" => Sub::<i32>::sub,
    "*" => Mul::<i32>::mul,
    "/" => Div::<i32>::div,
    _ => panic!("Operand konnt nicht verarbeitet werden."),
};

let x1 = x2(y1, y2);

println!("\nIhr Ergebnis ist: {}", x1);

// debug, hide it when you release it
// println!("z1: {}", z1);
// println!("y1: {}", y1);
// println!("z2: {}", z2);
// println!("y2: {}", y2);
// println!("x1: {}", x1);
//println!("z3: {}", z3);
//println!("y3: {}", y3);






println!("\nDanke!");
}
