struct User {
    name : String,
    age : i32,
    college : String,
    srn : String,
    place : String
}

impl User {
    fn check_age (&self) -> bool {
        if self.age < 18{
                println!("Minor present alert !");
                return false
        } else {
                println!("Passed the age criteria !");
                return true
        }
    }
}
fn main(){
    let new_user = User {
        name : String::from("Hemant KR"),
        age : 19,
        college : String::from("PES"),
        srn : String::from("PES1UG24CS807"),
        place : String::from("Bangalore")
    };

    if new_user.check_age() {
        println!("{}", new_user.name);
        println!("{}", new_user.age);
        println!("{}", new_user.college);
        println!("{}", new_user.srn);
        println!("{}", new_user.place);
    }
}