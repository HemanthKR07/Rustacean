use std::fmt::Display;

fn printer<'a, T>(a : & 'a str, b : & 'a str, ann : T) -> &'a str where T: Display, {
        println!("Announcement is {ann}");
        if a.len() > b.len() {
                return a;
        } else {
                return b;
        }
}

fn main(){}
