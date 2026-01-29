struct Lvl ( Vec<Vec<u8>> );

impl Lvl {
    pub fn generate() -> Lvl {
        let rows: Vec<Vec<u8>> = Vec::new();
        for _ in 0..10 {
            let mut new_row: Vec<u8> = Vec::new();
            for _ in 0..10 {
                new_row.push(0);
            }
        }
        Lvl(rows)
    }
}
