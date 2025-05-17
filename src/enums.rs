enum Direction {
    North, 
    South,
    East,
    West
}

fn main(){
    let my_direction = Direction::South;
    let my_friends_direction = my_direction;
    give_direction(my_friends_direction);
    
    let my_direction = Direction::East;
    give_direction(my_direction);

    let my_direction = Direction::West;
    give_direction(my_direction);

    let my_direction = Direction::North;
    give_direction(my_direction);

}

fn give_direction(direction1 : Direction){
        match direction1 {
                
                Direction::North => {
                        println!("Go 10m straight towards North.");
                },
            
                Direction::South => {
                        println!("Go 10m straight towards South.");
                },

                Direction::East => {
                        println!("Go 10m straight towards East.");
                },

                Direction::West => {
                        println!("Go 10m straight towards West.");
                }                
        }
}
