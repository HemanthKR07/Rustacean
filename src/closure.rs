
fn main(){
    // This is reference.
   let x = 10;
   println!("This is closure");
   let _hello = || println!("This is x : {}", x);
   _hello();

   // This is mut reference 
   let mut y = 10;
   let mut _gm = || {y += 1; println!("Y val  : {}", y);};
   _gm();

   // This is move reference
   let a = 10;
   let _gn = move || println!("A val : {}", a);
   _gn();
}
