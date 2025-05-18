fn main() {
    let mut nums = vec![1,2,3];
/*    let iter = nums.iter();
    
    for val in iter {
            println!("{}", val);
    }
 */   
    
    for i in nums.iter_mut() {
            *i *= 2;
    }
    println!("{:?}", nums)


    /*while let Some(val) = iter_mut.next(){
                println!("{}", val);
    }*/
}

 
1. iter
2, iter_mut
3. into_iter

I) Consumer Iterator : This one consumers the iterator, in future we wont be able tro use the iterator. 
                        iter.Sum();
II) Iterator Adapter : This one uses th eadapter and creates a new iterator using it.
                       1. let iter = vec.iter();
                          let iter_new = iter.map(|x| x * 2);
                       
                       2. let iter_new = iter_val.filter(|x| x%2==0).map(|x| *x * 2);

