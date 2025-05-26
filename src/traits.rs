trait Summary {
        fn summarize (&self) -> String {
                    return String::from("hi from summarize");
        }
}

trait Fix {
        fn fixer (&self) -> String {
                    return String::from("hi from fixer");
        }
}

struct User {
    name : String,
    age : i32,
}

impl Summary for User {}
impl Fix for User {}

fn main() {
        let _user_ = User {
                name : String::from("Hemanth"),
                age : 20
        };

        notify(_user_);
}

fn notify <T : Summary + Fix> (input : T) {
        println!("{}", input.summarize());
        println!("{}", input.fixer());
}
