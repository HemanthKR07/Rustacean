struct User <'a> {
    name : &'a str
}

fn main(){
        let fname = String::from("Hemanth");
        let student = User {name : &fname};
        println!("The name of the student is {} .", student.name);
}
