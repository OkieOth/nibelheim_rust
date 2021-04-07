use std::collections::HashMap;

pub const MITHRIL: u8 = 1;
pub const GOLD: u8 = 2;
pub const SILVER: u8 = 3;
pub const DIAMOND: u8 = 4;
pub const IRON: u8 = 5;
pub const CUPPER: u8 = 5;


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

    pub fn debug_print(&self) {
        println!("I am a dwarf");
    }
}