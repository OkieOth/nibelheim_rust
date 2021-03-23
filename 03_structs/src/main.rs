/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

use rand::{thread_rng, Rng};

enum MineralType {
    MITHRIL,
    GOLD,
    SILVER,
    DIAMOND,
    IRON,
    CUPPER
}

struct MineSpot {
    mineral: MineralType,
    amount: u32
}

fn init_gold_and_stuff() -> Option<MineSpot> {
    if thread_rng().gen_range(0..2) == 1 {
        // this spot has minerals
        None
    }
    else {
        // this spot is empty
        None
    }
}

fn init_mine_with_gold(mine: &mut [Option<MineSpot>]) {
    println!("Current size of the mine: {}", mine.len());
    for index in 0..mine.len() {        
        mine[index] = init_gold_and_stuff();
    }
}

fn main() {
    const MINE_SIZE :usize = 10;
    let mut mine: [Option<MineSpot>; MINE_SIZE] = [None; MINE_SIZE];

    init_mine_with_gold(&mut mine);

    let mut output: String = "Mine [".to_owned();

    let last_index = mine.len() -1; 
    for index in 0..mine.len() {
        if mine[index].is_some() {
            let spot: MineSpot = mine[index].unwrap();
            match spot.mineral {
                MineralType::MITHRIL => output.push_str("M"), 
                MineralType::GOLD => output.push_str("G"),
                MineralType::SILVER => output.push_str("S"),
                MineralType::DIAMOND => output.push_str("D"),
                MineralType::IRON => output.push_str("I"),
                MineralType::CUPPER => output.push_str("C")            
            }
        } else {
            output.push_str(".");
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
    fn test_has_gold() {
        let mut found: bool = false;

        for _index in 0..10 {
            if has_gold() {
                found = true;
                break;
            }
        }    
        assert!(found);
    }
}