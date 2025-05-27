use std::rc::Rc;

// RC
// here when I clone b and c; the refernece count of a increases to 3. That is the number of vars
// pointing to a value.
// But in multithreaded architecture, what if 2 or more threads try to update this ref_count at the
// same time, the value can get corrupted, there's no mutex involved here to take care of this
// ref_count.

fn main() {
    let a = Rc::new(String::from("hemanth"));

    let b = Rc::clone(&a);
    let c = Rc::clone(&a);

    println!("A : {}",a);
    println!("B : {}",b);
    println!("C : {}",c);
}
