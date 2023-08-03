use super::{lab::LabPixel, rgb::RgbPixel};

/*
    WARNING!
    This may not be 100% accurate. Converting an image from RGB to LCH and back results in some errors.
*/

#[derive(Debug, Clone, Copy)]
pub struct LchPixel(pub f32, pub f32, pub f32);

impl From<(f32, f32, f32)> for LchPixel {
    fn from(value: (f32, f32, f32)) -> Self {
        let (l, c, h) = value;
        LchPixel(l, c, h)
    }
}

impl From<RgbPixel> for LchPixel {
    fn from(value: RgbPixel) -> Self {
        Self::from_lab(&LabPixel::from_rgb(&value))
    }
}

impl From<LabPixel> for LchPixel {
    fn from(value: LabPixel) -> Self {
        Self::from_lab(&value)
    }
}

impl Into<RgbPixel> for LchPixel {
    fn into(self) -> RgbPixel {
        self.as_lab().as_rgb()
    }
}

impl Into<LabPixel> for LchPixel {
    fn into(self) -> LabPixel {
        self.as_lab()
    }
}

impl LchPixel {
    pub fn get(&self) -> (f32, f32, f32) {
        (self.0, self.1, self.2)
    }

    pub fn from_lab(lab: &LabPixel) -> LchPixel {
        let (l, a, b) = lab.get();

        let arc_calc = b.atan2(a).to_degrees();

        LchPixel(
            l,
            (a.powf(2.0) + b.powf(2.0)).sqrt(),
            if arc_calc >= 0.0 { arc_calc } else { arc_calc + 360.0 },
        )
    }

    pub fn as_lab(&self) -> LabPixel {
        let (l, c, h) = self.get();

        LabPixel::from((
            l,
            c * h.to_radians().cos(),
            c * h.to_radians().sin(),
        ))
    }

    pub fn add_luma(&mut self, luma: f32) -> &mut Self {
        self.0 = (self.0 + luma).clamp(0.0, 100.0);
        self
    }

    pub fn add_chroma(&mut self, chroma: f32) -> &mut Self {
        self.1 = (self.1 + chroma).clamp(0.0, 132.0);
        self
    }

    pub fn add_hue(&mut self, hue: f32) -> &mut Self {
        self.2 = self.2 + hue;
        self
    }
}