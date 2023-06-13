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
    pub fn _new(a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) -> Self {
        ColorPalette { a, b, c, d }
    }

    pub fn get(&self, t: f64) -> [f64; 3] {
        [
            (self.a[0] + self.b[0] * f64::cos(std::f64::consts::TAU * (self.c[0] * t + self.d[0])))
                * 255.0,
            (self.a[1] + self.b[1] * f64::cos(std::f64::consts::TAU * (self.c[1] * t + self.d[1])))
                * 255.0,
            (self.a[2] + self.b[2] * f64::cos(std::f64::consts::TAU * (self.c[2] * t + self.d[2])))
                * 255.0,
        ]
    }

    pub fn get_u8(&self, t: u8) -> [f64; 3] {
        assert!(t > 0, "t must be greater than zero!");
        self.get((2f64.powi(-1*t.ilog2() as i32)) as f64 * (t as f64 + 0.5f64)-1.0f64)
    }

    pub fn get_uuid(&self, t: uuid::Uuid) -> [f64; 3] {
        log::debug!("{}", t.as_u128() as f64 / (u128::MAX) as f64);
        self.get(t.as_u128() as f64 / (u128::MAX) as f64)
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
