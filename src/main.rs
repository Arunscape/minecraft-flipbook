#![allow(dead_code)]

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::io::Cursor;
use image::io::Reader as ImageReader;
use image::Rgb;
use std::convert::From;


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

impl From<Rgb<u8>> for Colour {
    fn from(rgb: Rgb<u8>) -> Self {
        let [r, g, b] = rgb.0;
        c!(r, g, b)
    }
}

impl Colour {
    pub fn l1_norm(&self, other: Self) -> usize {
        let dr2 = (self.r.abs_diff(other.r) as usize);
        let dg2 = (self.g.abs_diff(other.g) as usize);
        let db2 = (self.b.abs_diff(other.b) as usize);
        dr2 + dg2 + db2
    }

    pub fn l2_norm(&self, other: Self) -> usize {
        let dr2 = (self.r.abs_diff(other.r) as usize) << 1;
        let dg2 = (self.g.abs_diff(other.g) as usize) << 1;
        let db2 = (self.b.abs_diff(other.b) as usize) << 1;
        dr2 + dg2 + db2
    }
    pub fn closest_colour(&self, others: &HashMap<&'static str, Colour>) -> Colour {
        *others.iter().map(|(k, v)| (k, v, self.l1_norm(*v))).min_by(|(_, _, c1), (_, _, c2)| c1.cmp(c2)).unwrap().1
    }
}
static MC_COLOUR: Lazy<HashMap<&'static str, Colour>> = Lazy::new(|| {
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


fn main() -> Result<(), Box<dyn std::error::Error>>{

    let mut img = ImageReader::open("nyan.jpg")?.decode()?.to_rgb8();

    img.pixels_mut().for_each(|p| {
        let c = Colour::from(*p);
        let Colour { r, g, b } = c.closest_colour(&MC_COLOUR);
        *p = [r, g, b].into()
    });

    img.save("output.png")?;
    println!("Hello, world!");

    Ok(())
}
