#[derive(Debug, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn clear() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

    pub fn to_hsl(&self) -> ColorHSL {
        let r = self.r as f32 / 255.0;
        let g = self.g as f32 / 255.0;
        let b = self.b as f32 / 255.0;

        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;

        let h = if delta == 0.0 {
            0.0
        } else if cmax == r {
            ((g - b) / delta) % 6.0
        } else if cmax == g {
            ((b - r) / delta) + 2.0
        } else {
            ((r - g) / delta) + 4.0
        } * 60.0;

        let l = (cmax + cmin) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        return ColorHSL {
            h,
            s,
            l,
            a: self.a as f32 / 255.0,
        };
    }

    pub fn shift(&self) -> Color {
        println!("{:?}", self);
        return Color {
            r: self.r - 10,
            g: self.g - 10,
            b: self.b - 10,
            a: self.a,
        };
    }

    pub fn to_u32(&self) -> u32 {
        ((self.a as u32) << 24) | ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

pub struct ColorHSL {
    h: f32,
    s: f32,
    l: f32,
    a: f32,
}

impl ColorHSL {
    pub fn new(h: f32, s: f32, l: f32) -> ColorHSL {
        ColorHSL { h, s, l, a: 1.0 }
    }

    pub fn shift_values(&mut self, h: f32, s: f32, l: f32) {
        self.h = (self.h + h).clamp(0.0, 1.0);
        self.s = (self.s + s).clamp(0.0, 1.0);
        self.l = (self.l + l).clamp(0.0, 1.0);
    }

    pub fn to_rgb(&self) -> Color {
        let mut r = 0.0;
        let mut g = 0.0;
        let mut b = 0.0;

        if self.s == 0.0 {
            r = self.l;
            g = self.l;
            b = self.l;
        } else {
            let q = if self.l < 0.5 {
                self.l * (1.0 + self.s)
            } else {
                self.l + self.s - self.l * self.s
            };
            let p = 2.0 * self.l - q;
            r = ColorHSL::hue2rgb(p, q, self.h + 1.0 / 3.0);
            g = ColorHSL::hue2rgb(p, q, self.h);
            b = ColorHSL::hue2rgb(p, q, self.h - 1.0 / 3.0);
        }

        return Color::rgb(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
        );
    }

    fn hue2rgb(p: f32, q: f32, mut t: f32) -> f32 {
        if t < 0.0 {
            t += 1.0;
        }
        if t > 1.0 {
            t -= 1.0;
        }
        if t < 1.0 / 6.0 {
            return p + (q - p) * 6.0 * t;
        }
        if t < 1.0 / 2.0 {
            return q;
        }
        if t < 2.0 / 3.0 {
            return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
        }
        return p;
    }
}

// var r, g, b;

//     if(s == 0){
//         r = g = b = l; // achromatic
//     }else{
//         var hue2rgb = function hue2rgb(p, q, t){
//             if(t < 0) t += 1;
//             if(t > 1) t -= 1;
//             if(t < 1/6) return p + (q - p) * 6 * t;
//             if(t < 1/2) return q;
//             if(t < 2/3) return p + (q - p) * (2/3 - t) * 6;
//             return p;
//         }

//         var q = l < 0.5 ? l * (1 + s) : l + s - l * s;
//         var p = 2 * l - q;
//         r = hue2rgb(p, q, h + 1/3);
//         g = hue2rgb(p, q, h);
//         b = hue2rgb(p, q, h - 1/3);
//     }

//     return [Math.round(r * 255), Math.round(g * 255), Math.round(b * 255)];
macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color::rgb($r, $g, $b)
    };
}
pub(crate) use rgb;
