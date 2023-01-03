use std::process;
use ray_tracer::*;
use std::rc::Rc;
fn main() {
    //Image 
    const ASPECT_RATIO : f64 = 16.0/9.0;
    const IMAGE_WIDTH : i32 = 400;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }

    //Camera

    const VIEWPORT_HEIGHT : f64 = 2.0;
    const FOCAL_LENGTH : f64 = 1.0;

    let camera = Camera::new(VIEWPORT_HEIGHT, ASPECT_RATIO, FOCAL_LENGTH);
    //Environment
    let material_ground: Rc<dyn materials::Mat> = Rc::new(
        materials::Lambertian::new(Vec3::new(0.8, 0.8, 0.0)
    ));
    let material_center: Rc<dyn materials::Mat> = Rc::new(
        materials::Lambertian::new(Vec3::new(0.7, 0.3, 0.3)
    ));
    let material_left: Rc<dyn materials::Mat> = Rc::new(
        materials::Metal::new(Vec3::new(0.8, 0.8, 0.8)
    ));
    let material_right: Rc<dyn materials::Mat> = Rc::new(
        materials::Metal::new(Vec3::new(0.8, 0.6, 0.2)
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
    drop(world);
}
