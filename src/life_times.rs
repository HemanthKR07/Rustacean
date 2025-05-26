// Here first is of one lifetime, second is some lifetime
// and the returning string is the intersection os both 
// first and second strings.

fn longest<'a> (first : &'a str, second : &'a str) -> &'a str {
            if first.len() > second.len() {
                        return first;
            } else {
                        return second;
            }
}

fn main() {
    let result;
    let str1 = String::from("hemanth");
    //{
        let str2 = String::from("Blr");
        result = longest(&str1,&str2);
    //}

    println!("The result is {}", result);
}
