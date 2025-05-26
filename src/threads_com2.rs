use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main(){
     let (tx, rx) = mpsc::channel();
     let _tx1 = tx.clone();

     thread::spawn(move || {
            let values = vec![
                  String::from("hi"),
                  String::from("from"),
                  String::from("Solana"),
                  String::from("foundation")
            ];

            for val in values {
                   tx.send(val).unwrap();
                   thread::sleep(Duration::from_secs(1));
            }
     });

     thread::spawn(move || {
            let more_values = vec![
                   String::from("from 2.1"),
                   String::from("from 2.2"),
                   String::from("from 2.3"),
                   String::from("from 2.4"),
            ];
            for more_vals in more_values {
                   _tx1.send(more_vals).unwrap();
                   thread::sleep(Duration::from_secs(1));
            }
     });

     for i in rx {
                println!("value received : {i}");
     }
}
