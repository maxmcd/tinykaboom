use rayon::prelude::*;
use std::cmp::max;
use std::cmp::min;
use std::f32::consts::PI;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::ops::{Add, Mul, Sub};
use std::time::SystemTime;

const SPHERE_RADIUS: f32 = 1.2;
const NOISE_AMPLITUDE: f32 = 0.2;
const WIDTH: usize = 640;
const HEIGHT: usize = 480;

#[derive(Debug, Copy, Clone)]
struct Vec3([f32; 3]);

impl Vec3 {
    fn norm(&self) -> f32 {
        let (x, y, z) = (self.0[0], self.0[1], self.0[2]);
        (x * x + y * y + z * z).sqrt()
    }
    fn signed_distance(self, na: f32) -> f32 {
        let s = self * (SPHERE_RADIUS / self.norm());
        let displacement =
            ((16.0 * s.0[0]).sin() * (16.0 * s.0[1]).sin() * (16.0 * s.0[2]).sin()) * na;
        self.norm() - (SPHERE_RADIUS + displacement)
    }

    fn normalize(self) -> Self {
        self * (1.0 / self.norm())
    }
    fn distance_field_normal(self, na: f32) -> Self {
        let eps = 0.1;
        let d = self.signed_distance(na);
        let nx = (self + Vec3([eps, 0.0, 0.0])).signed_distance(na) - d;
        let ny = (self + Vec3([0.0, eps, 0.0])).signed_distance(na) - d;
        let nz = (self + Vec3([0.0, 0.0, eps])).signed_distance(na) - d;
        Vec3([nx, ny, nz]).normalize()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3([self.0[0] * other, self.0[1] * other, self.0[2] * other])
    }
}

impl Mul for Vec3 {
    type Output = f32;
    fn mul(self, other: Self) -> f32 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
}

fn partial_ord_max(a: f32, b: f32) -> f32 {
    if a < b {
        return b;
    }
    return a;
}

fn sphere_trace(na: f32, orig: Vec3, dir: Vec3) -> (bool, Vec3) {
    let mut pos = orig;
    for _i in 0..128 {
        let d = pos.signed_distance(na);
        if d < 0.0 {
            return (true, pos);
        }
        pos = pos + (dir * partial_ord_max(d * 0.1, 0.01));
    }
    return (false, pos);
}

fn write_frame(index: usize, sphere_radius: f32, noise_aplitude: f32) {
    println!(
        "Writing frame with radius {} and amplitude {}",
        sphere_radius, noise_aplitude
    );
    let fov = PI / 3.;

    let mut framebuffer: [Vec3; WIDTH * HEIGHT] = [Vec3([0.0, 0.0, 0.0]); WIDTH * HEIGHT];

    let render_timer = SystemTime::now();
    // actual rendering loop
    framebuffer
        .par_chunks_mut(WIDTH)
        .enumerate()
        .for_each(|(j, line)| {
            for (i, pixel) in line.iter_mut().enumerate() {
                let dir_x = ((i as f32) + 0.5) - (WIDTH as f32) / 2.;
                // this flips the image at the same time
                let dir_y = -((j as f32) + 0.5) + (HEIGHT as f32) / 2.;
                let dir_z = -(HEIGHT as f32) / (2. * ((fov / 2.).tan()));
                let (in_sphere, hit) = sphere_trace(
                    noise_aplitude,
                    Vec3([0.0, 0.0, 3.0]),
                    Vec3([dir_x, dir_y, dir_z]).normalize(),
                );
                *pixel = if in_sphere {
                    let light_dir = (Vec3([10.0, 10.0, 10.0]) - hit).normalize(); // one light is placed to (10,10,10)
                    let light_intensity =
                        partial_ord_max(0.4, light_dir * hit.distance_field_normal(noise_aplitude));
                    Vec3([1.0, 1.0, 1.0]) * light_intensity
                } else {
                    Vec3([0.2, 0.7, 0.8])
                }
            }
        });
    println!("Render time: {:?}", render_timer.elapsed().unwrap());

    let write_timer = SystemTime::now();
    let mut file = BufWriter::with_capacity(
        (WIDTH * HEIGHT) + 20,
        File::create(format!("./out-{:02}.ppm", index)).unwrap(),
    );
    file.write(format!("P6\n{} {}\n255\n", WIDTH, HEIGHT).as_bytes())
        .unwrap();
    for i in 0..HEIGHT * WIDTH {
        for j in 0..3 {
            file.write_all(&[max(0, min(255, (255.0 * framebuffer[i].0[j]) as i32)) as u8])
                .unwrap();
        }
    }
    println!("Write time: {:?}", write_timer.elapsed().unwrap());
}

fn main() {
    for i in 0..400 {
        write_frame(i, SPHERE_RADIUS, NOISE_AMPLITUDE - ((i as f32) * 0.001))
    }
}
