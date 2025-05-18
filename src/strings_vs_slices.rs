fn main(){
/*  STRINGS  

    let mut name = String::from("Hemanth");
    name.push_str(" KR");
    println!("{}", name);

    name.replace_range(7..name.len(), " is gonna take care of his parents.");
    println!("{}", name); */
    


    //  SLICES 
   /* let name = String::from("Hemanth KR ");
    let nam = &name[0..7];
    println!("{}", nam); */ 


    // SLICES PROBLEM
    

    let name = String::from("Hemanth is an asshole");
    let ans = get_name(&name);
    println!("{}",ans);
}


fn get_name(name : &str) -> &str {
        let mut index = 0;
        for i in name.chars() {
                if i == ' '{
                        break;
                }
                index += 1;
        }
        return &name[0..index];
}
