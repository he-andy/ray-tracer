use ray_tracer::*;

fn main() {
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 500;
    const _BVH: bool = false;

    let (camera, world) = scenes::random_scene();
    camera.render(IMAGE_HEIGHT, &world, SAMPLES_PER_PIXEL);
}
