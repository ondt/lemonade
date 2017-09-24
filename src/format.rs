pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Color { r, g, b, a, }
    }

    // Takes either: #rrggbb, #aarrggbb, #rbg, #argb
    // The '#' in front is not necessary
    pub fn from_hex(hex: &str) -> Result<Color, &'static str>{
        let mut s = String::from(hex);
        let mut c = [1.0, 0.0, 0.0, 0.0]; // argb

        // Remove the prepended #
        if s.get(0..1) == Some("#") {
            s.remove(0);
        }

        // Make sure the length is ok, and set the multiplier
        // used when iterating.
        let mut m: usize;
        let mut b: usize;
        match s.len() {
            3 => { m = 1; b = 1 }
            4 => { m = 1; b = 0 }
            6 => { m = 2; b = 1 }
            8 => { m = 2; b = 0 }
            _ => return Err("Provided string as invalid length"),
        }

        let mut it = s.chars().peekable();
        let mut n = 0;
        while it.peek().is_some() {
            let h: String = it.by_ref().take(m).collect();
            let v = i32::from_str_radix(&h, 16).unwrap();
            c[b + n] = v as f64 / (16.0_f64.powi(m as i32) - 1.0);
            n += 1;
        }

        Ok(Color {
            a: c[0],
            r: c[1],
            g: c[2],
            b: c[3],
        })
    }
}

impl Clone for Color {
    fn clone(&self) -> Self {
        Color::new(self.r, self.g, self.b, self.a)
    }
}

pub struct Text {
    pub bg: Color,
    pub fg: Color,
    pub text: String,
}

// Must be the last item of the vector!
pub struct Filler {
    pub bg: Color,
}

impl Filler {
    pub fn new(bg: Color) -> Self {
        Filler {
            bg
        }
    }
}

pub enum FormatItem {
    Text(Text),
    Filler(Filler),
}
