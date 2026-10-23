use std::fmt;
use std::ops::{Add, Mul};
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseMathError(pub String);

impl fmt::Display for ParseMathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for ParseMathError {}

fn parse_floats(s: &str) -> Vec<f32> {
    s.split(|c: char| c == ',' || c == '{' || c == '}' || c.is_whitespace())
        .filter(|part| !part.is_empty())
        .filter_map(|num| num.parse::<f32>().ok())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Vec2 {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 2 {
            return Err(ParseMathError(format!("expected 2 floats, got {}: {:?}", v.len(), s)));
        }
        Ok(Self::new(v[0], v[1]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2i {
    pub x: i32,
    pub y: i32,
}

impl Vec2i {
    pub const ZERO: Self = Self { x: 0, y: 0 };

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl FromStr for Vec2i {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 2 {
            return Err(ParseMathError(format!("expected 2 ints, got {}: {:?}", v.len(), s)));
        }
        Ok(Self::new(v[0] as i32, v[1] as i32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0 };

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl FromStr for Vec3 {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 3 {
            return Err(ParseMathError(format!("expected 3 floats, got {}: {:?}", v.len(), s)));
        }
        Ok(Self::new(v[0], v[1], v[2]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4 {
    pub const ZERO: Self = Self { x: 0.0, y: 0.0, z: 0.0, w: 0.0 };

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}

impl FromStr for Vec4 {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 4 {
            return Err(ParseMathError(format!("expected 4 floats, got {}: {:?}", v.len(), s)));
        }
        Ok(Self::new(v[0], v[1], v[2], v[3]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat3 {
    pub cols: [Vec3; 3],
}

impl Mat3 {
    pub const IDENTITY: Self = Self {
        cols: [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0)],
    };

    pub fn from_cols_array(array: &[f32; 9]) -> Self {
        Self {
            cols: [
                Vec3::new(array[0], array[1], array[2]),
                Vec3::new(array[3], array[4], array[5]),
                Vec3::new(array[6], array[7], array[8]),
            ],
        }
    }
}

impl FromStr for Mat3 {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 9 {
            return Err(ParseMathError(format!("expected 9 floats, got {}: {:?}", v.len(), s)));
        }

        #[rustfmt::skip]
        let cols = [
            v[0], v[3], v[6],
            v[1], v[4], v[7],
            v[2], v[5], v[8],
        ];

        Ok(Self::from_cols_array(&cols))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    pub cols: [Vec4; 4],
}

impl Mat4 {
    pub const IDENTITY: Self = Self {
        cols: [
            Vec4::new(1.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
            Vec4::new(0.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        ],
    };

    pub fn from_cols_array(array: &[f32; 16]) -> Self {
        Self {
            cols: [
                Vec4::new(array[0], array[1], array[2], array[3]),
                Vec4::new(array[4], array[5], array[6], array[7]),
                Vec4::new(array[8], array[9], array[10], array[11]),
                Vec4::new(array[12], array[13], array[14], array[15]),
            ],
        }
    }
}

impl FromStr for Mat4 {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = parse_floats(s);
        if v.len() != 16 {
            return Err(ParseMathError(format!("expected 16 floats, got {}: {:?}", v.len(), s)));
        }

        #[rustfmt::skip]
        let cols = [
            v[0], v[4], v[8], v[12],
            v[1], v[5], v[9], v[13],
            v[2], v[6], v[10], v[14],
            v[3], v[7], v[11], v[15],
        ];

        Ok(Self::from_cols_array(&cols))
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self { x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z }
    }
}

impl Mul<Vec3> for Mat3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            x: self.cols[0].x * rhs.x + self.cols[1].x * rhs.y + self.cols[2].x * rhs.z,
            y: self.cols[0].y * rhs.x + self.cols[1].y * rhs.y + self.cols[2].y * rhs.z,
            z: self.cols[0].z * rhs.x + self.cols[1].z * rhs.y + self.cols[2].z * rhs.z,
        }
    }
}

impl Mul<Mat3> for Mat3 {
    type Output = Mat3;

    fn mul(self, rhs: Mat3) -> Mat3 {
        Mat3 { cols: [self * rhs.cols[0], self * rhs.cols[1], self * rhs.cols[2]] }
    }
}

impl Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.cols[0].x * rhs.x
                + self.cols[1].x * rhs.y
                + self.cols[2].x * rhs.z
                + self.cols[3].x * rhs.w,
            y: self.cols[0].y * rhs.x
                + self.cols[1].y * rhs.y
                + self.cols[2].y * rhs.z
                + self.cols[3].y * rhs.w,
            z: self.cols[0].z * rhs.x
                + self.cols[1].z * rhs.y
                + self.cols[2].z * rhs.z
                + self.cols[3].z * rhs.w,
            w: self.cols[0].w * rhs.x
                + self.cols[1].w * rhs.y
                + self.cols[2].w * rhs.z
                + self.cols[3].w * rhs.w,
        }
    }
}

impl Mul<Mat4> for Mat4 {
    type Output = Mat4;

    fn mul(self, rhs: Mat4) -> Mat4 {
        Mat4 {
            cols: [self * rhs.cols[0], self * rhs.cols[1], self * rhs.cols[2], self * rhs.cols[3]],
        }
    }
}

pub(crate) fn parse_vec2(s: &str) -> Vec2 {
    s.parse().expect("failed to parse Vec2")
}

pub(crate) fn parse_vec3(s: &str) -> Vec3 {
    s.parse().expect("failed to parse Vec3")
}

pub(crate) fn parse_mat3(s: &str) -> Mat3 {
    s.parse().expect("failed to parse Mat3")
}

pub(crate) fn parse_mat4(s: &str) -> Mat4 {
    s.parse().expect("failed to parse Mat4 from mat4")
}

pub(crate) fn parse_mat4_or_identity(s: Option<&str>) -> Mat4 {
    match s {
        Some(s) => parse_mat4(s),
        None => Mat4::IDENTITY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec2_parsing() {
        assert_eq!("1.0, 2.0".parse::<Vec2>().unwrap(), Vec2::new(1.0, 2.0));
        assert_eq!("{1.5, 2.5}".parse::<Vec2>().unwrap(), Vec2::new(1.5, 2.5));
        assert!("1.0".parse::<Vec2>().is_err());
    }

    #[test]
    fn test_vec2i_parsing() {
        assert_eq!("10, 20".parse::<Vec2i>().unwrap(), Vec2i::new(10, 20));
        assert_eq!("{5, 15}".parse::<Vec2i>().unwrap(), Vec2i::new(5, 15));
    }

    #[test]
    fn test_vec3_parsing() {
        assert_eq!("{1.0, 2.0, 3.0}".parse::<Vec3>().unwrap(), Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vec4_parsing() {
        assert_eq!("{1.0, 2.0, 3.0, 4.0}".parse::<Vec4>().unwrap(), Vec4::new(1.0, 2.0, 3.0, 4.0));
    }

    #[test]
    fn test_mat3_parsing() {
        let input = "{1, 0, 0} {0, 1, 0} {0, 0, 1}";
        let mat: Mat3 = input.parse().unwrap();
        assert_eq!(mat, Mat3::IDENTITY);

        let input_transposed = "{1, 2, 3} {4, 5, 6} {7, 8, 9}";
        let mat: Mat3 = input_transposed.parse().unwrap();
        assert_eq!(mat.cols[0], Vec3::new(1.0, 4.0, 7.0));
        assert_eq!(mat.cols[1], Vec3::new(2.0, 5.0, 8.0));
        assert_eq!(mat.cols[2], Vec3::new(3.0, 6.0, 9.0));
    }

    #[test]
    fn test_mat4_parsing() {
        let input = "{1, 0, 0, 0} {0, 1, 0, 0} {0, 0, 1, 0} {0, 0, 0, 1}";
        let mat: Mat4 = input.parse().unwrap();
        assert_eq!(mat.cols[0], Vec4::new(1.0, 0.0, 0.0, 0.0));
        assert_eq!(mat.cols[1], Vec4::new(0.0, 1.0, 0.0, 0.0));
        assert_eq!(mat.cols[2], Vec4::new(0.0, 0.0, 1.0, 0.0));
        assert_eq!(mat.cols[3], Vec4::new(0.0, 0.0, 0.0, 1.0));
    }
}
