
macro_rules! c {
    ($r:expr, $g:expr, $b:expr) => (
        Colour { r: $r, g: $g, b: $b }        
    )
}

struct Colour {
    r: u8,
    g: u8,
    b: u8
}

struct McColour;

impl McColour {
    pub const Black: Colour = c!(0, 0, 0);
    pub const DarkBlue: Colour = c!(0, 0, 170);
    pub const DarkGreen: Colour = c!(0, 170, 0);
    pub const DarkAqua: Colour = c!(0, 170, 170);
    pub const DarkRed: Colour = c!(170, 0, 0);
    pub const DarkPurple: Colour = c!(170, 0, 170);
    pub const Gold: Colour = c!(255, 170, 0);
    pub const Gray: Colour = c!(170, 170, 170);
    pub const DarkGray: Colour = c!(85, 85, 85);
    pub const Blue: Colour = c!(85, 85, 255);
    pub const Green: Colour = c!(85, 255, 85);
    pub const Aqua: Colour = c!(85, 255, 255);
    pub const Red: Colour = c!(255, 85, 85);
    pub const LightPurple: Colour = c!(255, 85, 255);
    pub const Yellow: Colour = c!(255, 255, 85);
    pub const White: Colour = c!(255, 255, 255);
    pub const MinecoinGold: Colour = c!(221, 214, 5);
}

const WIDTH: usize = 76;
const HEIGHT: usize = 56;


fn main() {

    let x = c!(0, 1, 2);

    println!("Hello, world!");
}
