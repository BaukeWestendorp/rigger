pub mod gdtf;
pub mod mvr;

use std::str::FromStr;

/// Re-export of glam.
pub use glam;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CieColor {
    pub x: f32,
    pub y: f32,
    pub yy: f32,
}

impl FromStr for CieColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
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
