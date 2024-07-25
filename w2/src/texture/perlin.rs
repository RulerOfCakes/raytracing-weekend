use crate::primitive::{point3::Point3, vec3::Vec3};

#[derive(Debug)]
pub struct Perlin {
    randvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    // Ideally must be a power of 2
    const POINT_CNT: usize = 256;
    pub fn new() -> Self {
        let randvec = vec![Vec3::new(0.0, 0.0, 0.0); Self::POINT_CNT]
            .iter()
            .map(|_| Vec3::random_uniform(-1.0, 1.0))
            .collect();

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Self {
            randvec,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    // Generates noise values in the range [-1, 1]
    pub fn noise(&self, p: Point3) -> f64 {
        // u, v, w denote offset from the integer part of the point
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = p.x.floor() as i32;
        let j = p.y.floor() as i32;
        let k = p.z.floor() as i32;
        let mut c = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];

        #[allow(clippy::needless_range_loop)]
        for di in 0..2i32 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di as usize][dj as usize][dk as usize] = self.randvec[self.perm_x
                        [((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize]];
                }
            }
        }

        // perform trilinear interpolation using the surrounding cube(8 points)
        Self::trilinear_interpolation(c, u, v, w)
    }

    pub fn turbulence(&self, p: &Point3, depth: i32) -> f64 {
        let mut accum = 0.;
        let mut temp_p = *p;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.;
        }
        accum.abs()
    }

    fn generate_perm() -> Vec<usize> {
        let p: Vec<usize> = vec![0; Self::POINT_CNT]
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();

        Self::permute(p, Self::POINT_CNT)
    }

    fn permute(p: Vec<usize>, n: usize) -> Vec<usize> {
        let mut p = p;
        for i in (0..n).rev() {
            let target = rand::random::<usize>() % (i + 1);
            p.swap(i, target);
        }
        p
    }

    fn trilinear_interpolation(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        // use hermite cubic to smooth the noise, to resolve mach bands
        // On the unit interval with starting point p0 = (0, 0, 0), and 0 tangents at p0 and p1,
        // f(t) = t^2(3 - 2t)
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        #[allow(clippy::needless_range_loop)]
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_vec = Vec3::new(u - i as f64, v - j as f64, w - k as f64);
                    accum += (i as f64 * uu + (1.0 - i as f64) * (1.0 - uu))
                        * (j as f64 * vv + (1.0 - j as f64) * (1.0 - vv))
                        * (k as f64 * ww + (1.0 - k as f64) * (1.0 - ww))
                        * c[i][j][k].dot(&weight_vec);
                }
            }
        }
        accum
    }
}
