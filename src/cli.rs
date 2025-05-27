use std::env;

fn main() {
    let args : Vec<String> = env::args().collect();
    let a = &args[1];
    let b = &args[2];

    println!(" A : {}", a);
    println!(" B : {}", b);
}
