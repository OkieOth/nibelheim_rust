use std::collections::HashMap;

use crate::modules::mine as mine;

pub const MITHRIL: u8 = 1;
pub const GOLD: u8 = 2;
pub const SILVER: u8 = 3;
pub const DIAMOND: u8 = 4;
pub const IRON: u8 = 5;
pub const CUPPER: u8 = 6;


pub struct Dwarf {
    pub pocket: HashMap<u8, usize>
}

fn init_pocket() -> HashMap<u8, usize> {
    let mut x: HashMap<u8, usize> = HashMap::new();
    x.insert(MITHRIL,0);
    x.insert(GOLD,0);
    x.insert(SILVER,0);
    x.insert(DIAMOND,0);
    x.insert(IRON,0);
    x.insert(CUPPER,0);
    x
}

impl Dwarf {
    pub fn new() -> Dwarf {
        Dwarf {
            pocket: init_pocket()
        }
    }

    fn mineral_number_to_text(&self, mineral: &u8) -> String {
        match *mineral {
            MITHRIL => "Mithril".to_owned(), 
            GOLD => "Gold".to_owned(),
            SILVER => "Silver".to_owned(),
            DIAMOND => "Diamonds".to_owned(),
            IRON => "Iron".to_owned(),
            CUPPER => "Cupper".to_owned(),
            _ => "???".to_owned()
        }
    }

    pub fn debug_print(&self) {
        println!("I am a dwarf, and my pocket contains ...");
        for (k,v) in self.pocket.iter() {
            println!("    {}: {}", self.mineral_number_to_text(k), v);
        }
    }

    pub fn do_nothing(&self) {
    }

    fn set_mineral(&mut self, mineral: u8, value: usize) {
        self.pocket.insert(mineral,value);
    }

    pub fn increment_pocket(&mut self, mineral: u8) {
        match self.pocket.get(&mineral) {
            Some(&value) => self.set_mineral(mineral,value + 1),
            _ => self.do_nothing()
        }        
    }

    pub fn visit_mine_spot(&mut self, mine_spot: Option<&mine::MineSpot>) {
        if mine_spot.is_some() {
            let spot = mine_spot.unwrap();
            match spot.mineral {
                mine::MineralType::MITHRIL => self.increment_pocket(MITHRIL), 
                mine::MineralType::GOLD => self.increment_pocket(GOLD),
                mine::MineralType::SILVER => self.increment_pocket(SILVER),
                mine::MineralType::DIAMOND => self.increment_pocket(DIAMOND),
                mine::MineralType::IRON => self.increment_pocket(IRON),
                mine::MineralType::CUPPER => self.increment_pocket(CUPPER),
                _ => ()
            }
        }
    }
}