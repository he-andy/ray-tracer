use ray_tracer::*;
use timed::timed;
#[macro_use] extern crate log;

#[timed::timed(tracing(enabled = true), duration(disabled = true))]
fn main() {
    let trace = timed::Trace::new("Main");
    //Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: i32 = 90;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 1;

    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }

    //Camera
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let v_up = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        20.0,
        ASPECT_RATIO,
        &look_from,
        &look_at,
        &v_up,
        dist_to_focus,
        aperture,
    );

    let world = BVH::from_hittable_list(random_scene());  
    eprintln!("#balls {}", random_scene().list.len());
    eprintln!("height of tree: {}", world.height());
    let world = random_scene();
    
    //Render

    camera.render(IMAGE_HEIGHT, &world, SAMPLES_PER_PIXEL);

    println!("{}", trace.statistics());
    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_mat = materials::Lambertian::from_vec(Vec3::new(0.5, 0.5, 0.5));

    world.add(Sphere::new(
        Point::new(0.0, -1000.0, -1.0),
        1000.0,
        ground_mat,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = crate::rand();
            let center = Point::new(
                a as f64 + 0.9 * crate::rand(),
                0.2,
                b as f64 + 0.9 * crate::rand(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let mat = materials::Lambertian::from_vec(Vec3::rand(0.0, 1.0));
                    world.add(Sphere::new(center, 0.2, mat));
                } else if choose_mat < 0.95 {
                    let mat =
                        materials::Metal::from_vec(Vec3::rand(0.0, 1.0), rand_range(0.0, 0.5));
                    world.add(Sphere::new(center, 0.2, mat));
                } else {
                    let mat = materials::Dielectric::new(1.5);
                    world.add(Sphere::new(center, 0.2, mat));
                }
            }
        }
    }

    let mat1 = materials::Dielectric::new(1.5);
    world.add(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1));

    let mat2 = materials::Lambertian::new(0.4, 0.2, 0.1);
    world.add(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));
    
    let mat3 = materials::Metal::new(0.7, 0.6, 0.5, 0.0);
    world.add(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3));
    world
}
