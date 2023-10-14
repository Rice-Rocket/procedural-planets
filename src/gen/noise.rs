use bevy::prelude::*;

#[derive(Clone)]
pub struct NoiseSimplex3d {
    random: [i32; Self::SIZE as usize * 2],
}

impl Default for NoiseSimplex3d {
    fn default() -> Self {
        Self {
            random: [0; Self::SIZE as usize * 2],
        }
    }
}

impl NoiseSimplex3d {
    const SOURCE: [i32; 256] = [
        151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69, 142,
        8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219, 203,
        117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175, 74, 165,
        71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230, 220, 105, 92, 41,
        55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76, 132, 187, 208, 89,
        18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173, 186, 3, 64, 52, 217, 226, 250,
        124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206, 59, 227, 47, 16, 58, 17, 182, 189,
        28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163, 70, 221, 153, 101, 155, 167, 43, 172, 9,
        129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232, 178, 185, 112, 104, 218, 246, 97, 228, 251, 34,
        242, 193, 238, 210, 144, 12, 191, 179, 162, 241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31,
        181, 199, 106, 157, 184, 84, 204, 176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114,
        67, 29, 24, 72, 243, 141, 128, 195, 78, 66, 215, 61, 156, 180
    ];
    const SIZE: u32 = 256;

    const F3: f32 = 1.0 / 3.0;
    const G3: f32 = 1.0 / 6.0;

    const GRAD_3: [Vec3; 12] = [
        Vec3::new(1.0, 1.0, 0.0), Vec3::new(-1.0, 1.0, 0.0), Vec3::new(1.0, -1.0, 0.0),
        Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, 0.0, 1.0), Vec3::new(-1.0, 0.0, 1.0),
        Vec3::new(1.0, 0.0, -1.0), Vec3::new(-1.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 1.0),
        Vec3::new(0.0, -1.0, 1.0), Vec3::new(0.0, 1.0, -1.0), Vec3::new(0.0, -1.0, -1.0)
    ];

    pub fn new(seed: u32) -> Self {
        let mut noise_gen = Self {
            random: [0; Self::SIZE as usize * 2]
        };
        noise_gen.randomize(seed);
        return noise_gen;
    }

    pub fn evaluate(&self, p: Vec3) -> f32 {
        let x = p.x;
        let y = p.y;
        let z = p.z;

        let mut n0 = 0.0;
        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut n3 = 0.0;

        let s = (x + y + z) * Self::F3;

        let i = (x + s).floor();
        let j = (y + s).floor();
        let k = (z + s).floor();

        let t = (i + j + k) * Self::G3;

        let x0 = x - (i - t);
        let y0 = y - (j - t);
        let z0 = z - (k - t);

        let (i1, j1, k1);
        let (i2, j2, k2);

        if x0 >= y0 {
            if y0 >= z0 {
                i1 = 1.0;
                j1 = 0.0;
                k1 = 0.0;
                i2 = 1.0;
                j2 = 1.0;
                k2 = 0.0;
            } else if x0 >= z0 {
                i1 = 1.0;
                j1 = 0.0;
                k1 = 0.0;
                i2 = 1.0;
                j2 = 0.0;
                k2 = 1.0;
            } else {
                i1 = 0.0;
                j1 = 0.0;
                k1 = 1.0;
                i2 = 1.0;
                j2 = 0.0;
                k2 = 1.0;
            }
        } else {
            if y0 < z0 {
                i1 = 0.0;
                j1 = 0.0;
                k1 = 1.0;
                i2 = 0.0;
                j2 = 1.0;
                k2 = 1.0;
            } else if x0 < z0 {
                i1 = 0.0;
                j1 = 1.0;
                k1 = 0.0;
                i2 = 0.0;
                j2 = 1.0;
                k2 = 1.0;
            } else {
                i1 = 0.0;
                j1 = 1.0;
                k1 = 0.0;
                i2 = 1.0;
                j2 = 1.0;
                k2 = 0.0;
            }
        }

        let x1 = x0 - i1 + Self::G3;
        let y1 = y0 - j1 + Self::G3;
        let z1 = z0 - k1 + Self::G3;

        let x2 = x0 - i2 + Self::F3;
        let y2 = y0 - j2 + Self::F3;
        let z2 = z0 - k2 + Self::F3;

        let x3 = x0 - 0.5;
        let y3 = y0 - 0.5;
        let z3 = z0 - 0.5;

        let ii = (i as i32) & 0xff;
        let jj = (j as i32) & 0xff;
        let kk = (k as i32) & 0xff;

        let mut t0 = 0.6 - x0 * x0 - y0 * y0 - z0 * z0;
        if t0 > 0.0 {
            t0 *= t0;
            let gi0 = self.random[(ii + self.random[(jj + self.random[kk as usize]) as usize]) as usize] % 12;
            n0 = t0 * t0 * Self::dot(Self::GRAD_3[gi0 as usize], x0, y0, z0);
        }

        let mut t1 = 0.6 - x1 * x1 - y1 * y1 - z1 * z1;
        if t1 > 0.0 {
            t1 *= t1;
            let gi1 = self.random[(ii + i1 as i32 + self.random[(jj + j1 as i32 + self.random[kk as usize + k1 as usize]) as usize]) as usize] % 12;
            n1 = t1 * t1 * Self::dot(Self::GRAD_3[gi1 as usize], x1, y1, z1);
        }

        let mut t2 = 0.6 - x2 * x2 - y2 * y2 - z2 * z2;
        if t2 > 0.0 {
            t2 *= t2;
            let gi2 = self.random[(ii + i2 as i32 + self.random[(jj + j2 as i32 + self.random[kk as usize + k2 as usize]) as usize]) as usize] % 12;
            n2 = t2 * t2 * Self::dot(Self::GRAD_3[gi2 as usize], x2, y2, z2);
        }

        let mut t3 = 0.6 - x3 * x3 - y3 * y3 - z3 * z3;
        if t3 > 0.0 {
            t3 *= t3;
            let gi3 = self.random[(ii + 1 + self.random[(jj + 1 + self.random[kk as usize + 1]) as usize]) as usize] % 12;
            n3 = t3 * t3 * Self::dot(Self::GRAD_3[gi3 as usize], x3, y3, z3);
        }

        (n0 + n1 + n2 + n3) * 32.0
    }

    fn randomize(&mut self, seed: u32) {
        if seed != 0 {
            let bytes = Self::unpack_u32(seed);
            for i in 0..(Self::SIZE as usize) {
                self.random[i] = Self::SOURCE[i] ^ bytes[0] as i32;
                self.random[i] ^= bytes[1] as i32;
                self.random[i] ^= bytes[2] as i32;
                self.random[i] ^= bytes[3] as i32;

                self.random[i + Self::SIZE as usize] = self.random[i];
            }
        } else {
            for i in 0..(Self::SIZE as usize) {
                self.random[i] = Self::SOURCE[i];
                self.random[i + Self::SIZE as usize] = Self::SOURCE[i];
            }
        }
    }

    fn dot(g: Vec3, x: f32, y: f32, z: f32) -> f32 {
        g.x * x + g.y * y + g.z * z
    }

    fn unpack_u32(val: u32) -> [u8; 4] {
        [
            (val & 0x00ff) as u8,
            ((val & 0xff00) >> 8) as u8,
            ((val & 0x00ff0000) >> 16) as u8,
            ((val & 0xff000000) >> 24) as u8,
        ]
    }
}