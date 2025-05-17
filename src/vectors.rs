// Here we are using {:?} to print the vector,  bcz vectors are structs !!
// 
// We can print any values from the vector but not the entire vectors. Just like how we print
// values of  stuct and need :? to print the struct, the same way.
            
// println!("{:?}", &vec);
// fn even_or_not(vec : &Vec<i32>) -> Vec<i32>

// These 2 are of two different functions, 1st will take the ownership from the main function,
// where as 2nd will just borrow it from the main func and return the ownership after its job.


/*
fn even_or_not(vec: Vec<i32>) -> Vec<i32> {
    let mut new_vec = Vec::new();
    for val in vec {
            if val % 2 == 0 {
                    new_vec.push(val);
            }
    }
    return new_vec;
}
*/

fn remove_odd(vec : &mut Vec<i32>) {
  let mut i = 0;
  while i < vec.len() {
            if vec[i] % 2 != 0 {
                    vec.remove(i);
            } else {
                    i += 1;
            }
    }
}

fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    vec.push(6);
    println!("{:?}", vec);
    //println!("{:?}", even_or_not(vec));
    //println!("{:?}", vec);
    remove_odd(&mut vec);
    println!("{:?}", vec);
  
}


