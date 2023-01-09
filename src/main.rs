use ray_tracer::*;

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 900;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const _BVH: bool = false;

    let (camera, world) = scenes::two_perlin_spheres();
    camera.render(IMAGE_HEIGHT, &world, SAMPLES_PER_PIXEL);
}
