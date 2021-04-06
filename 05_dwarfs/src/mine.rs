use rand::{thread_rng, Rng};

pub enum MineralType {
    MITHRIL,
    GOLD,
    SILVER,
    DIAMOND,
    IRON,
    CUPPER,
    ROCK
}

pub struct MineSpot {
    pub mineral: MineralType
}

pub fn init_gold_and_stuff() -> Option<MineSpot> {
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

pub fn init_mine_with_gold(mine: &mut Vec<Option<MineSpot>>, mine_size: usize) {
    for _index in 0..mine_size {        
        let spot: Option<MineSpot> = init_gold_and_stuff();
        mine.push(spot);
    }
}    
