use std::collections::HashMap;


fn tuple_to_hashmap(vec : Vec<(String, i32)>) -> HashMap<String, i32> {
       let mut hashmap = HashMap::new();
       for (key, value) in vec {
                    hashmap.insert(key, value);
       }
       return hashmap;
}


fn main() {
    let mut user = HashMap::new();

    user.insert(String::from("Hemanth"), 20);
    user.insert(String::from("Icarus"), 20);

    match user.get("Hemanth") {
        Some(x) => {
                println!("Age of Hemanth KR is : {}", x); 
        },
        None => {
                println!("Failed to fetch the value.");
        }
    }

    let vector : Vec<(String,i32)> = vec![
                       (String::from("Hemanth"),20),
                       (String::from("Icarus"), 20), 
                       (String::from("Trump"), 69)
                    ];
    let result = tuple_to_hashmap(vector);

    println!("\nThis is the hashmap I have received  : {:?}", result);
}
