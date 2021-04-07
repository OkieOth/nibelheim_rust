/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

mod modules;

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
    let mut mine: Vec<Option<modules::mine::MineSpot>> = Vec::new();

    modules::mine::init_mine_with_gold(&mut mine, mine_size);

    let mut output: String = "Mine [".to_owned();

    let last_index = mine.len() -1; 
    for index in 0..mine.len() {
        let opt: Option<&modules::mine::MineSpot> = mine[index].as_ref();
        output.push_str(modules::mine::print(opt));

        if index < last_index {
            output.push_str(", ");
        }
    }
    output.push_str("]");
    println!("{}", output);

    let dwarf = modules::dwarf::Dwarf::new();
    dwarf.debug_print();
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
            let spot: Option<modules::mine::MineSpot> = modules::mine::init_gold_and_stuff();
            if spot.is_some() {
                found = true
            }
        }    
        assert!(found);
    }
}