use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
       let counter = Arc::new(Mutex::new(0));
       let mut vector = vec![];

       for _ in 1..6{
             let counter = Arc::clone(&counter);
             let handler = thread::spawn(move || {
                      let mut num = counter.lock().unwrap();
                      *num += 1;
             });
             vector.push(handler);
       }
       for j in vector{
          j.join().unwrap();
       }
       println!("Val : {:?}", *counter.lock().unwrap());
}
