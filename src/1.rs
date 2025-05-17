fn odd_or_even (a : i32) -> bool {
    if a % 2 == 0 {
        return true;
    } 
    return false;
}


fn main() {
    println!("Hello, world!");
    let result = odd_or_even(19);
   println!("{}", result);
}
