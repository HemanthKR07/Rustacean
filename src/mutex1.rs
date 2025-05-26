use std::sync::Mutex;

fn main() {
       let num = Mutex::new(44);
       println!("Before num : {:?}", num);
       {
             let mut add = num.lock().unwrap();
             *add += 1;
       }
       println!("Current num : {:?}", num);
}
