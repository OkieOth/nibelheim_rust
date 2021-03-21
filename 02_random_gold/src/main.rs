/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

use rand::{thread_rng, Rng};

fn has_gold() -> bool {
    if thread_rng().gen_range(0..2) == 1 {
        true
    }
    else {
        false
    }
}

fn init_mine_with_gold(mine: &mut [bool]) {
    println!("Current size of the mine: {}", mine.len());
    for index in 0..mine.len() {
        mine[index] = has_gold();
    }
}

fn main() {
    const MINE_SIZE :usize = 8;
    let mut mine: [bool; MINE_SIZE] = [false; MINE_SIZE];

    init_mine_with_gold(&mut mine);

    let mut output: String = "Mine [".to_owned();

    let last_index = mine.len() -1; 
    for index in 0..mine.len() {
        if mine[index] {
            output.push_str("true");
        } else {
            output.push_str("false");
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