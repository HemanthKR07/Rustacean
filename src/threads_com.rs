use std::thread;
use std::sync::mpsc;

fn main() {
      let (tx, rx) = mpsc::channel();
      thread::spawn(move || {
                 let val = String::from("Hello, this is from thread");
                 tx.send(val).unwrap();
      });
      let value = rx.recv().unwrap();
      println!("Value received is '{}'", value);

      // As soon as the rx.recv() is executed, the execution of the main thread is stopped
      // until tx's msg is added into the queue.
      // 1. tx is non blocking and rx is blocked.
      // 2. tx is blocking only when its bounded buffer, and when its full.
      //
      // -> rx.try_recv() : This one is non blocking.
      // -> rx.recv_timeout(Duration::from_secs(5)); : Blocks the main threads execution only till
      // the duration.
}
