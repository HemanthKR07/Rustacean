fn main(){
    let my_string = String::from("Hemanth's Birthday is here !!");
    println!("{}", get_string_length(my_string));
}

fn get_string_length(strng : String) -> usize {
    strng.chars().count()
}

//println!("{}", get_string_length(&mystring));
//fn get_string_length(strn : &str){}