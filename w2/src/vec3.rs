use rand::{Error, Rng};
use rand_distr::Distribution;
use rand_distr::NormalError;
use std::ops;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f64) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Self::Output {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn zero() -> Vec3 {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    pub fn random_uniform(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: rand::thread_rng().gen_range(min..max),
            y: rand::thread_rng().gen_range(min..max),
            z: rand::thread_rng().gen_range(min..max),
        }
    }
    pub fn random_gaussian(mean: f64, stddev: Option<f64>) -> Result<Vec3, NormalError> {
        let mut rng = rand::thread_rng();
        let normal = rand_distr::Normal::new(mean, stddev.unwrap_or(1.0))?;

        Ok(Vec3 {
            x: normal.sample(&mut rng),
            y: normal.sample(&mut rng),
            z: normal.sample(&mut rng),
        })
    }
    pub fn random_unit() -> Vec3 {
        let v = Vec3::random_gaussian(0., None).unwrap();
        v.unit()
    }
    pub fn random_in_disk(radius: f64) -> Vec3 {
        let r = rand::thread_rng().gen_range(0.0..radius);
        let theta = rand::thread_rng().gen_range(0.0..2.0 * std::f64::consts::PI);

        Vec3 {
            x: r * theta.cos(),
            y: r * theta.sin(),
            z: 0.0,
        }
    }
    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn near_zero(&self) -> bool {
        static EPS: f64 = 1e-8;
        self.x.abs() < EPS && self.y.abs() < EPS && self.z.abs() < EPS
    }
    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn unit(&self) -> Vec3 {
        *self / self.length()
    }
}
