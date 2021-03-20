/*
Mine simulator .. the mine is represented by an array
... a really poor one :D
*/

fn init_mine_with_gold(mine: &mut [bool]) {
    println!("Current size of the mine: {}", mine.len());
    mine[5] = true;
}

fn create_mine() -> [bool; 10] {
    let mine: [bool; 10] = [false; 10];
    mine
}

fn main() {
    let mut mine: [bool; 10] = create_mine();

    init_mine_with_gold(&mut mine);

    println!("Mine [{}, {}, {}, {}, {}, {}, {}, {}, {}, {}]",
        mine[0],mine[1],mine[2],mine[3],mine[4],mine[5],mine[6],mine[7],mine[8],mine[9])
}

#[cfg(test)]
mod tests {
    // this runs only if `cargo test is called`
    // make the functions outside visible
    use super::*;

    #[test]
    fn test_create_mine() {
        let mine: [bool; 10] = create_mine();
        assert_eq!(mine.len(), 10);
    }

    #[test]
    fn test_init_mine_with_gold() {
        let mut mine: [bool; 10] = create_mine();
        assert_eq!(mine.len(), 10);

        init_mine_with_gold(&mut mine);
        assert_eq!(mine[0], false);
        assert_eq!(mine[1], false);
        assert_eq!(mine[2], false);
        assert_eq!(mine[3], false);
        assert_eq!(mine[4], false);
        assert_eq!(mine[5], true);
        assert_eq!(mine[6], false);
        assert_eq!(mine[7], false);
        assert_eq!(mine[8], false);
        assert_eq!(mine[9], false);
    }
}