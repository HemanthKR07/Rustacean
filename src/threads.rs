use std::thread;
use std::time::Duration;

fn main() {
   let thread_handler = thread::spawn(||{ 
        for i in 1..10{
                  println!("Inside the thread : Round : {}\n", i);
                  thread::sleep(Duration::from_millis(1000));
        }
   });
    for _i in 1..6 {
            println!("This is from the main thread.");
            thread::sleep(Duration::from_millis(1000));
    }
    thread_handler.join().unwrap();
   // match {
   // Some(a) 
   // None
   // }
   //
   // Match and unwrap give the same output, in match is has some and none value, and in unwrap it
   // will panic if the val is none.
   

    for j in 1..6 {
        println!("this is after join : {}\n", j);
   }
}
