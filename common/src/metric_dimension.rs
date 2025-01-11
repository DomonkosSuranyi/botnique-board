use crate::metric_dimension::length::Meter;
use bevy::prelude::{Quat, Transform, Vec2};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, Mul, Neg, Sub};
use std::time::Duration;

macro_rules! impl_trait {
    (impl $trait:ident :: $method:ident ::< $other:ty > for $base:ident -> $output:ident) => {
        impl $trait<$other> for $base {
            type Output = $output;

            fn $method(self, rhs: $other) -> Self::Output {
                $output(self.0.$method(rhs.0))
            }
        }

        impl $trait<ThinVec<$other>> for $base {
            type Output = ThinVec<$output>;

            fn $method(self, rhs: ThinVec<$other>) -> Self::Output {
                ThinVec::<$output> {
                    x: self.$method(rhs.x),
                    y: self.$method(rhs.y),
                }
            }
        }
    };
}

impl_trait! { impl Div::div::<Second> for Meter -> MeterPerSec }
impl_trait! { impl Mul::mul::<MeterPerSec> for Second -> Meter }
impl_trait! { impl Div::div::<MeterPerSec> for Meter -> Second }
impl_trait! { impl Add::add::<MeterPerSec> for MeterPerSec -> MeterPerSec }
impl_trait! { impl Sub::sub::<Meter> for Meter -> Meter }
impl_trait! { impl Add::add::<Meter> for Meter -> Meter }

const PIXEL_PER_METER: u16 = 32;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ThinVec<T> {
    pub x: T,
    pub y: T,
}

pub mod length {
    use super::*;
    use bevy::math::{Vec2, Vec3};
    use bevy::prelude::Reflect;

    #[derive(Default, Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd, Reflect)]
    pub struct Meter(pub f32);

    pub type MeterVec2 = ThinVec<Meter>;

    pub struct MeterVec3 {
        pub xy: MeterVec2,
        pub z: Meter,
    }

    impl Meter {
        pub fn into_pixel(self) -> f32 {
            self.0 * (PIXEL_PER_METER as f32)
        }

        pub fn from_pixel(pixel: f32) -> Self {
            Meter(pixel / (PIXEL_PER_METER as f32))
        }
    }

    impl MeterVec2 {
        pub fn from_raw(x: f32, y: f32) -> MeterVec2 {
            MeterVec2 {
                x: Meter(x),
                y: Meter(y),
            }
        }

        pub fn from_pixel_vec(vec: Vec2) -> Self {
            Self {
                x: Meter::from_pixel(vec.x),
                y: Meter::from_pixel(vec.y),
            }
        }

        pub fn into_pixel_vec(self) -> Vec2 {
            Vec2::new(self.x.into_pixel(), self.y.into_pixel())
        }

        pub fn into_transform(self, z: Meter) -> Transform {
            Transform::from_xyz(self.x.into_pixel(), self.y.into_pixel(), z.into_pixel())
        }
    }

    impl Neg for Meter {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Meter(self.0.neg())
        }
    }

    impl Mul<Vec3> for Meter {
        type Output = MeterVec3;

        /// The z coordinate will not be multiplied
        fn mul(self, rhs: Vec3) -> Self::Output {
            MeterVec3 {
                xy: self * rhs.truncate(),
                z: Meter::from_pixel(rhs.z),
            }
        }
    }

    impl Mul<f32> for Meter {
        type Output = Meter;

        fn mul(self, rhs: f32) -> Self::Output {
            Meter(self.0 * rhs)
        }
    }

    impl Div<f32> for Meter {
        type Output = Meter;

        fn div(self, rhs: f32) -> Self::Output {
            Meter(self.0 / rhs)
        }
    }

    impl Mul<Vec2> for Meter {
        type Output = MeterVec2;

        fn mul(self, rhs: Vec2) -> Self::Output {
            MeterVec2 {
                x: Meter(self.0 * rhs.x),
                y: Meter(self.0 * rhs.y),
            }
        }
    }

    pub fn magnitude(vector: &MeterVec2) -> Meter {
        Meter::from_pixel(vector.into_pixel_vec().length())
    }

    pub fn normalize(vector: MeterVec2) -> Vec2 {
        vector.into_pixel_vec().normalize()
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq)]
pub struct Second(pub f32);

impl Second {
    pub fn into_duration(self) -> Duration {
        Duration::from_secs_f32(self.0)
    }
}

impl From<Duration> for Second {
    fn from(duration: Duration) -> Self {
        Second(duration.as_secs_f32())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct MeterPerSec(pub f32);

pub type MeterPerSecVec2 = ThinVec<MeterPerSec>;

impl MeterPerSec {
    pub fn from_pixel_per_sec(pixel_per_sec: f32) -> Self {
        MeterPerSec(pixel_per_sec / (PIXEL_PER_METER as f32))
    }

    pub fn into_pixel_per_sec(self) -> f32 {
        self.0 * (PIXEL_PER_METER as f32)
    }
}

impl Neg for MeterPerSec {
    type Output = Self;
    fn neg(self) -> Self::Output {
        MeterPerSec(self.0.neg())
    }
}

impl Mul<Vec2> for MeterPerSec {
    type Output = MeterPerSecVec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        MeterPerSecVec2 {
            x: MeterPerSec(self.0 * rhs.x),
            y: MeterPerSec(self.0 * rhs.y),
        }
    }
}

impl Div<f32> for MeterPerSec {
    type Output = MeterPerSec;

    fn div(self, rhs: f32) -> Self::Output {
        MeterPerSec(self.0 / rhs)
    }
}

impl Mul<f32> for MeterPerSec {
    type Output = MeterPerSec;

    fn mul(self, rhs: f32) -> Self::Output {
        MeterPerSec(self.0 * rhs)
    }
}

impl AddAssign<MeterPerSec> for MeterPerSec {
    fn add_assign(&mut self, rhs: MeterPerSec) {
        self.0.add_assign(rhs.0)
    }
}

impl Display for MeterPerSec {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} m/s", self.0)
    }
}

impl MeterPerSecVec2 {
    pub fn from_raw(x: f32, y: f32) -> Self {
        MeterPerSecVec2 {
            x: MeterPerSec(x),
            y: MeterPerSec(y),
        }
    }
    pub fn from_raw_vec(vec: Vec2) -> Self {
        MeterPerSecVec2::from_raw(vec.x, vec.y)
    }

    pub fn from_pixel_per_sec(vec: Vec2) -> Self {
        MeterPerSecVec2 {
            x: MeterPerSec::from_pixel_per_sec(vec.x),
            y: MeterPerSec::from_pixel_per_sec(vec.y),
        }
    }

    pub fn xy(&self) -> Vec2 {
        Vec2::new(self.x.0, self.y.0)
    }

    pub fn rotate(&self, rotation: &Quat) -> Self {
        Self::from_raw_vec(
            rotation
                .mul_vec3(self.xy().extend(0.0))
                .truncate(),
        )
    }

    pub fn into_pixel_per_sec_vec(self) -> Vec2 {
        Vec2::new(self.x.into_pixel_per_sec(), self.y.into_pixel_per_sec())
    }
}

impl Div<f32> for MeterPerSecVec2 {
    type Output = MeterPerSecVec2;

    fn div(self, rhs: f32) -> Self::Output {
        MeterPerSecVec2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl AddAssign<MeterPerSecVec2> for MeterPerSecVec2 {
    fn add_assign(&mut self, rhs: MeterPerSecVec2) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<MeterPerSecVec2> for MeterPerSecVec2 {
    type Output = MeterPerSecVec2;

    fn add(self, rhs: MeterPerSecVec2) -> Self::Output {
        MeterPerSecVec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
