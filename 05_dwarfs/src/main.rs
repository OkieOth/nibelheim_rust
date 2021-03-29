/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

mod mine;

use std::io;


    
fn main() {
    let mut mine_size_str = String::new();
    let mut mine_size: usize;
    loop {
        println!("Please enter the mine size. Choose a number between 1 and 20");
        io::stdin()
            .read_line(&mut mine_size_str)
            .expect("Failed to read line");
        mine_size = mine_size_str.trim().parse().expect ("Please type a number between 1 and 20!");
        if (mine_size > 0) && (mine_size < 21) {
            break;
        }
    
    }
   
    // this init sucks
    let mut mine: Vec<Option<mine::MineSpot>> = Vec::new();

    mine::init_mine_with_gold(&mut mine, mine_size);

    let mut output: String = "Mine [".to_owned();

    let last_index = mine.len() -1; 
    for index in 0..mine.len() {
        let opt: Option<&mine::MineSpot> = mine[index].as_ref();
        if opt.is_some() {
            let spot = opt.unwrap();
            match spot.mineral {
                mine::MineralType::MITHRIL => output.push_str("M"), 
                mine::MineralType::GOLD => output.push_str("G"),
                mine::MineralType::SILVER => output.push_str("S"),
                mine::MineralType::DIAMOND => output.push_str("D"),
                mine::MineralType::IRON => output.push_str("I"),
                mine::MineralType::CUPPER => output.push_str("C"),            
                mine::MineralType::ROCK => output.push_str("_")            
            }
        } else {
            output.push_str(" ");
        }

        if index < last_index {
            output.push_str(", ");
        }
    }
    output.push_str("]");
    println!("{}", output)
}

#[cfg(test)]
mod tests {
    // this runs only if `cargo test is called`
    // make the functions outside visible
    use super::*;

    #[test]
    fn test_init_mine_with_gold() {
        let mut found: bool = false;

        for _index in 0..10 {
            let spot: Option<mine::MineSpot> = mine::init_gold_and_stuff();
            if spot.is_some() {
                found = true
            }
        }    
        assert!(found);
    }
}