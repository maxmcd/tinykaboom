const sphere_radius: f32 = 1.5;
use std::cmp::max;
use std::cmp::min;
use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

fn main() {
    let width = 640;
    let height = 480;
    let fov = PI / 3.;

    let mut framebuffer: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]; width * height];

    // actual rendering loop
    for j in 0..height {
        for i in 0..width {
            let _dir_x = ((i as f32) + 0.5) - (width as f32) / 2.;
            // this flips the image at the same time
            let _dir_y = (-(i as f32) + 0.5) + (height as f32) / 2.;
            let _dir_z = -(height as f32) / (2. * ((fov / 2.).tan() as f32));
            framebuffer[i + j * width] = [0.2, 0.7, 0.8];
        }
    }
    let mut file = File::create("./out.ppm").unwrap();
    file.write(format!("P6\n{} {}\n255\n", width, height).as_bytes())
        .unwrap();
    for i in 0..height * width {
        for j in 0..3 {
            file.write(&[max(0, min(255, (255.0 * framebuffer[i][j]) as i32)) as u8])
                .unwrap();
        }
    }
}
