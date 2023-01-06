use std::{process};
use ray_tracer::*;
use ray_tracer::materials::Mat;
use std::rc::Rc;
fn main() {
    //Image 
    const ASPECT_RATIO : f64 = 3.0/2.0;
    const IMAGE_WIDTH : i32 = 1500;
    const IMAGE_HEIGHT : i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL : i32 = 500;

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

    let camera = Camera::new(20.0, ASPECT_RATIO, &look_from, &look_at, &v_up, dist_to_focus, aperture);

    let world = random_scene();
    
    //Render
    
    if let Err(e) = camera.render(IMAGE_HEIGHT, &world, SAMPLES_PER_PIXEL){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

    // if let Err(e) = gradient(IMAGE_WIDTH, IMAGE_HEIGHT){
    //     eprintln!("Application Error: {e}");
    //     process::exit(1);
    // }
}

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_mat: Rc<dyn Mat> = Rc::new(
        materials::Lambertian::from_vec(Vec3::new(0.5, 0.5, 0.5))
    );

    world.add(
        Box::new(
            Sphere::new(
                Point::new(0.0, -1000.0, -1.0),
                1000.0,
                Rc::clone(&ground_mat)
            )
        )
    );

    for a in -11..11 {
        for b in -11..11{
            let choose_mat = crate::rand();
            let center = Point::new(a as f64 + 0.9 * crate::rand(), 0.2, b as f64 + 0.9 * crate::rand());

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9{
                let mut mat: Rc<dyn Mat> = Rc::new(materials::Dielectric::new(1.5));
                if choose_mat < 0.8 {
                    mat = Rc::new(materials::Lambertian::from_vec(Vec3::rand(0.0, 1.0)));
                } else if choose_mat < 0.95 {
                    mat = Rc::new(materials::Metal::from_vec(Vec3::rand(0.0, 1.0), rand_range(0.0, 0.5)));
                }  
                
                world.add(
                    Box::new(
                        Sphere::new(
                            center,
                            0.2,
                            Rc::clone(&mat)
                        )
                    )
                );
            }
            
           let mat1: Rc<dyn Mat> = Rc::new(materials::Dielectric::new(1.5));
           world.add_sphere(Point::new(0.0, 1.0, 0.0), 1.0, mat1.clone());

           let mat2: Rc<dyn Mat> = Rc::new(materials::Lambertian::new(0.4, 0.2, 0.1));
           world.add_sphere(Point::new(-4.0, 1.0, 0.0), 1.0, mat2.clone());

           let mat3: Rc<dyn Mat> = Rc::new(materials::Metal::new(0.7, 0.6, 0.5, 0.0));
           world.add_sphere(Point::new(4.0, 1.0, 0.0), 1.0, mat3.clone());
        }
    }
    world
}
