use crate::Point;
use rand::Rng;

fn gen_perlin(len: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut p = Vec::with_capacity(256);
    for _ in 0..len {
        p.push(rng.gen());
    }
    p
}

fn gen_perlin_perm(len: usize) -> Vec<usize> {
    let mut v: Vec<usize> = (0..len).collect();
    permute(&mut v);
    v
}

fn permute(v: &mut Vec<usize>) {
    let n = v.len();
    let mut rng = rand::thread_rng();
    for i in (0..n as usize).rev() {
        let target = rng.gen_range(0..=i);
        v.swap(i, target);
    }
}

#[derive(Clone)]
pub struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Self {
        let n_points = 256;
        Self {
            ranfloat: gen_perlin(n_points),
            perm_x: gen_perlin_perm(n_points),
            perm_y: gen_perlin_perm(n_points),
            perm_z: gen_perlin_perm(n_points),
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let i = (4.0 * p.x).floor() as usize;
        let j = (4.0 * p.y).floor() as usize;
        let k = (4.0 * p.z).floor() as usize;
        
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {

                }
            }
        }

        self.ranfloat[self.perm_x[i as usize] ^ self.perm_y[j as usize] ^ self.perm_z[k as usize]]
    }
}
