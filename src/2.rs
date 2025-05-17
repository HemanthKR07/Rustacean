fn main(){
    let result : i32 = fib(8);
    println!("{}", result);
}

fn fib(a : i32) -> i32 {

    let mut first= 0;
    let mut second  = 1;
    let mut third = 1;
    
    if a == 0 {
        return first;
    } else if a == 1 {
        return second;
    }

    for _ in 1..a-2 {
            first = second;
            second = third;
            third = first + second;
    }
    return third;
}
