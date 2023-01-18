use once_cell::sync::Lazy;
use std::collections::HashMap;

macro_rules! c {
    ($r:expr, $g:expr, $b:expr) => (
        Colour { r: $r, g: $g, b: $b }        
    )
}

#[derive(Debug, Clone, Copy)]
struct Colour {
    r: u8,
    g: u8,
    b: u8
}

impl Colour {

    pub fn l2_norm(&self, other: Self) -> usize {
        let dr2 = ((self.r-other.r) as usize) << 1;
        let db2 = ((self.r-other.r) as usize) << 1;
        let dg2 = ((self.r-other.r) as usize) << 1;
        dr2 + db2 + dg2
    }
    pub fn closest_colour(&self, others: &HashMap<&'static str, Colour>) -> &'static str {
        others.iter().map(|(k, v)| (k, v, self.l2_norm(*v))).min_by(|(_, _, c1), (_, _, c2)| c1.cmp(c2)).unwrap().0
    }
}
static McColour: Lazy<HashMap<&'static str, Colour>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("black", c!(0, 0, 0));
    m.insert("dark_blue", c!(0, 0, 170));
    m.insert("dark_green", c!(0, 170, 0));
    m.insert("dark_aqua", c!(0, 170, 170));
    m.insert("dark_red", c!(170, 0, 0));
    m.insert("dark_purple", c!(170, 0, 170));
    m.insert("gold", c!(255, 170, 0));
    m.insert("gray", c!(170, 170, 170));
    m.insert("dark_gray", c!(85, 85, 85));
    m.insert("blue", c!(85, 85, 255));
    m.insert("green", c!(85, 255, 85));
    m.insert("aqua", c!(85, 255, 255));
    m.insert("red", c!(255, 85, 85));
    m.insert("light_purple", c!(255, 85, 255));
    m.insert("yellow", c!(255, 255, 85));
    m.insert("white", c!(255, 255, 255));
    m.insert("minecoin_gold", c!(221, 214, 5));
    m
});



const WIDTH: usize = 76;
const HEIGHT: usize = 56;


fn main() {

    println!("Hello, world!");
}
