
pub struct EarthTypePropertiesRules {
    pub min_resistance: u8;
    pub max_resistance: u8;
    pub min_content: u8;
    pub max_content: u8;
}


/*
Some attributes put on the specific earth types
*/
pub struct EarthTypePropertiesRules {
    pub resistance: u8;
    pub content: u8;
    pub haul_progress: u8;
}

/*
Type of earth that can be found in the mine
*/
pub enum EarthType {
    SAND,
    ROCK,
    MITHRIL,
    GOLD,
    SILVER,
    DIAMOND,
    IRON,
    CUPPER
}
