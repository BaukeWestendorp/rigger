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
    s.split('}')
        .filter_map(|row| {
            let row = row.trim_start_matches('{').trim();
            if row.is_empty() { None } else { Some(row) }
        })
        .flat_map(|row| {
            row.split(',').filter_map(|num| {
                let num = num.trim();
                if num.is_empty() { None } else { num.parse::<f32>().ok() }
            })
        })
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
        Ok(Self::from_cols_array(&[v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8]]))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub matrix: Mat3,
    pub translation: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self { matrix: Mat3::IDENTITY, translation: Vec3::ZERO };

    pub fn from_cols_array(array: &[f32; 12]) -> Self {
        Self {
            matrix: Mat3::from_cols_array(array[0..9].try_into().unwrap()),
            translation: Vec3::new(array[9], array[10], array[11]),
        }
    }

    pub fn from_mat4_str(s: &str) -> Result<Self, ParseMathError> {
        let v = parse_floats(s);
        if v.len() != 16 {
            return Err(ParseMathError(format!(
                "expected 16 floats for mat4, got {}: {:?}",
                v.len(),
                s
            )));
        }
        #[rustfmt::skip]
        let cols = [
            v[0], v[1], v[2],
            v[4], v[5], v[6],
            v[8], v[9], v[10],
            v[12] / 1000.0, v[13] / 1000.0, v[14] / 1000.0,
        ];
        Ok(Self::from_cols_array(&cols))
    }
}

impl FromStr for Transform {
    type Err = ParseMathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<f32>> = s
            .split('}')
            .filter_map(|row| {
                let row = row.trim_start_matches('{').trim();
                if row.is_empty() { None } else { Some(parse_floats(row)) }
            })
            .collect();

        if rows.len() != 4 {
            return Err(ParseMathError(format!(
                "expected 4 row groups, got {}: {:?}",
                rows.len(),
                s
            )));
        }
        if !rows.iter().all(|r| r.len() == 3) {
            return Err(ParseMathError(format!("each row must have 3 floats: {:?}", s)));
        }

        #[rustfmt::skip]
        let cols = [
            rows[0][0], rows[0][1], rows[0][2],
            rows[1][0], rows[1][1], rows[1][2],
            rows[2][0], rows[2][1], rows[2][2],
            rows[3][0] / 1000.0, rows[3][1] / 1000.0, rows[3][2] / 1000.0,
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

impl Mul for Transform {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            matrix: self.matrix * rhs.matrix,
            translation: self.matrix * rhs.translation + self.translation,
        }
    }
}

impl Mul<Transform> for &Transform {
    type Output = Transform;

    fn mul(self, rhs: Transform) -> Transform {
        *self * rhs
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

pub(crate) fn parse_transform_or_identity(s: Option<&str>) -> Transform {
    s.and_then(|str_val| str_val.parse().ok()).unwrap_or(Transform::IDENTITY)
}

pub(crate) fn parse_transform_from_mat4(s: &str) -> Transform {
    Transform::from_mat4_str(s).expect("failed to parse Transform from mat4")
}
