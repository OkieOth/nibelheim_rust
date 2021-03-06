/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

use rand::{thread_rng, Rng};
use std::io;

enum MineralType {
    MITHRIL,
    GOLD,
    SILVER,
    DIAMOND,
    IRON,
    CUPPER,
    ROCK
}

struct MineSpot {
    mineral: MineralType
}

fn init_gold_and_stuff() -> Option<MineSpot> {
    if thread_rng().gen_range(0..2) == 1 {
        // this spot has minerals
        let spot: MineSpot = MineSpot {
            mineral: match thread_rng().gen_range(0..5) {
                0 => MineralType::MITHRIL,
                1 => MineralType::GOLD,
                2 => MineralType::SILVER,
                3 => MineralType::DIAMOND,
                4 => MineralType::IRON,
                5 => MineralType::CUPPER,
                _ => MineralType::ROCK
            }
        };
        Some(spot)
    }
    else {
        // this spot is empty
        None
    }
}

fn init_mine_with_gold(mine: &mut Vec<Option<MineSpot>>, mine_size: usize) {
    println!("Current size of the mine: {}", mine.len());
    for _index in 0..mine_size {        
        let spot: Option<MineSpot> = init_gold_and_stuff();
        mine.push(spot);
    }
}
    
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
    let mut mine: Vec<Option<MineSpot>> = Vec::new();

    init_mine_with_gold(&mut mine, mine_size);

    let mut output: String = "Mine [".to_owned();

    let last_index = mine.len() -1; 
    for index in 0..mine.len() {
        let opt: Option<&MineSpot> = mine[index].as_ref();
        if opt.is_some() {
            let spot = opt.unwrap();
            match spot.mineral {
                MineralType::MITHRIL => output.push_str("M"), 
                MineralType::GOLD => output.push_str("G"),
                MineralType::SILVER => output.push_str("S"),
                MineralType::DIAMOND => output.push_str("D"),
                MineralType::IRON => output.push_str("I"),
                MineralType::CUPPER => output.push_str("C"),            
                MineralType::ROCK => output.push_str("_")            
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
            let spot: Option<MineSpot> = init_gold_and_stuff();
            if spot.is_some() {
                found = true
            }
        }    
        assert!(found);
    }
}