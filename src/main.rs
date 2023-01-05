use std::process;
use ray_tracer::*;
use std::rc::Rc;
fn main() {
    //Image 
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : i32 = 720;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }

    //Camera
    let look_from = Point::new(3.0, 3.0, 2.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let v_up = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;

    let camera = Camera::new(20.0, ASPECT_RATIO, &look_from, &look_at, &v_up, dist_to_focus, aperture);

    //Environment
    let material_ground: Rc<dyn materials::Mat> = Rc::new(
        materials::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)
    ));
    let material_center: Rc<dyn materials::Mat> = Rc::new(
        materials::Lambertian::new(Vec3::new(0.7, 0.3, 0.3)
    ));
    let material_left: Rc<dyn materials::Mat> = Rc::new(
        materials::Dielectric::new(1.5)
    );
    let material_right: Rc<dyn materials::Mat> = Rc::new(
        materials::Metal::new(Vec3::new(0.8, 0.6, 0.2),
        1.0
    ));

    let mut world = HittableList::default();
    world.add(
        Box::new(
            Sphere::new(
                Point::new(0.0, 0.0, -1.0),
                0.5,
                Rc::clone(&material_center)
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Point::new(-1.0, 0.0, -1.0),
                0.5,
                Rc::clone(&material_left)
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Point::new(-1.0, 0.0, -1.0),
                -0.4,
                Rc::clone(&material_left)
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Point::new(1.0, 0.0, -1.0),
                0.5,
                Rc::clone(&material_right)
            )
        )
    );

    world.add(
        Box::new(
            Sphere::new(
                Point::new(0.0, -100.5, -1.0),
                100.0,
                Rc::clone(&material_ground)
            )
        )
    );
    
    //Render
    
    if let Err(e) = camera.render(IMAGE_HEIGHT, &world, 100){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }
}
