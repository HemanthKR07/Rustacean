use std::thread;
use std::sync::Mutex;

/*
 fn main() {
      let counter = Mutex::new(1);
      let mut vector = vec![];

      for i in 1..11 {
               let handler = thread::spawn(move || {
                         let mut num = counter.lock().unwrap();
                         *num += 1;
               })
               vector.push(handler);
      }

      for j in vector {
               j,join().unwrap();
      }
      println!("vector : {vec:?}");
} 

 This above code wont work because here in the first iteration in the mutex lock loop, i am moving the reference 
 of the counter variable, it works. And in the next iteration it wont, because it has already moved it referenc in 
 the first iteration, so it throws an error.

Multiple ownership and multiple threads.
 */ 



/*
use std::rc::Rc;

fn main() {
       let counter = Rc::new(Mutex::new(1));
       let mut vector = vec![];

       for i in 1,,11 {
              let counter = Rc::clone(&counter);
              let handler = thread::spawn(move || {
                       let mut num = counter.lock().unwrap();
                       *num += 1;
              });
              vector.push(handler);
       }

       for j in vector {
              j.join().unwrap();
       }
       println("{:?}", vector);
}


*/ 


// Rc is not safe to be shared amoung many threads. Its not safe.
// So we use Atomic reference counting. 
//


use std::sync::{Arc, Mutex};

fn main() {
         let counter = Arc::new(Mutex::new(1));
         let mut vector = vec![];

         for i in 1..11 {
                 let counter = Arc::clone(&counter);
                 let handler = thread::spawn(move || {
                           let mut num = Mutex.lock().unwrap();
                           *num += 1;
                  })
                 vector.push(handler);
         }
         for j in vector {
                 j.join().unwrap();
         }
         println("");
}
