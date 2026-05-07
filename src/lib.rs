pub mod gdtf;
pub mod mvr;
pub(crate) mod util;

use std::str;

/// Re-export of glam.
pub use glam;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CieColor {
    x: f32,
    y: f32,
    yy: f32,
}

impl CieColor {
    pub fn new(x: f32, y: f32, yy: f32) -> Self {
        Self { x, y, yy }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn yy(&self) -> f32 {
        self.yy
    }
}

impl Default for CieColor {
    fn default() -> Self {
        Self { x: 0.3127, y: 0.3290, yy: 100.0 }
    }
}

impl str::FromStr for CieColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().trim_start_matches('{').trim_end_matches('}');
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            return Err(());
        }
        let x = parts[0].trim().parse::<f32>().map_err(|_| ())?;
        let y = parts[1].trim().parse::<f32>().map_err(|_| ())?;
        let yy = parts[2].trim().parse::<f32>().map_err(|_| ())?;
        Ok(CieColor { x, y, yy })
    }
}
