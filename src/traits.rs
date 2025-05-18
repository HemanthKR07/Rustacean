trait ForUser{
        fn new_age(&self);
}

struct User {
    name : String,
    age : i32,
}

impl ForUser for User {
        fn new_age(&self) {
                println!("I am {} and my age is {}. ", self.name, self.age);
        }
}

fn main(){
    let user = User {
            name : String::from("Hemanth"),
            age : 20,
    };
    user.new_age();
}
