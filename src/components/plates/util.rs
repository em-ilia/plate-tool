// Sources:
// https://iquilezles.org/articles/palettes/
// http://dev.thi.ng/gradients/

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ColorPalette {
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
}

impl ColorPalette {
    pub fn new(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> Self {
        ColorPalette { a, b, c, d }
    }

    pub fn get(&self, t: f64) -> [f64; 3] {
        [
            (self.a[0] + self.b[0] * f64::cos(6.28318 * (self.c[0] * t + self.d[0]))) * 255.0,
            (self.a[1] + self.b[1] * f64::cos(6.28318 * (self.c[1] * t + self.d[1]))) * 255.0,
            (self.a[2] + self.b[2] * f64::cos(6.28318 * (self.c[2] * t + self.d[2]))) * 255.0,
        ]
    }

    pub fn get_u8(&self, t: u8, n: u8) -> [f64; 3] {
        assert!(t > 0, "t must be greater than zero!");
        assert!(n > 0, "There cannot be zero points!");
        self.get((t - 1) as f64 / (n - 1) as f64)
    }
}

#[non_exhaustive]
pub struct Palettes;

#[allow(dead_code)]
impl Palettes {
    pub const RAINBOW: ColorPalette = ColorPalette {
        a: [0.500, 0.500, 0.500],
        b: [0.500, 0.500, 0.500],
        c: [0.800, 0.800, 0.800],
        d: [0.000, 0.333, 0.667],
    };
    pub const YELLOW_PINK: ColorPalette = ColorPalette {
        a: [0.500, 0.500, 0.320],
        b: [0.500, 0.500, 0.500],
        c: [0.100, 0.500, 0.360],
        d: [0.000, 0.000, 0.650],
    };
}
