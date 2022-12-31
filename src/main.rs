use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::process;
use ray_tracer::*;

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
    const VIEWPORT_WIDTH : f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH : f64 = 1.0;

    let camera = Camera::new(VIEWPORT_HEIGHT, VIEWPORT_WIDTH, FOCAL_LENGTH);
  
    //Render
    
    if let Err(e) = render(&camera, IMAGE_WIDTH, IMAGE_HEIGHT){
        eprintln!("Application Error: {e}");
        process::exit(1);
    }

}

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.dir.unit();
    let t = 0.5 *(unit_dir.y + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn gradient(width: i32, height: i32) -> Result<(), Box<dyn Error>>{
    let mut file = File::create("out.ppm")?;
    
    writeln!(&mut file, "P3\n{width} {height}\n255")?;

    for j in (0..height).rev(){
        eprint!("\rScanlines remaining: {j}");
        for i in 0..width{
            let r = i as f64 / (width - 1) as f64 ;
            let g = j as f64 / (height - 1) as f64;
            let b = 0.25;

            let color = Color::new(r, g, b);
            write_color(&mut file, color).expect("failed to print");
        }
        writeln!(&mut file, "")?;
    }
    eprintln!("\ndone!");
    Ok(())
}

fn render(camera : &Camera, width: i32, height: i32) -> Result<(), Box<dyn Error>>{
    let mut file = File::create("out.ppm")?;
    
    writeln!(&mut file, "P3\n{width} {height}\n255")?;
    for j in (0..height).rev(){
        eprint!("\rScanlines remaining: {j}");
        for i in 0..width{
            let u = i as f64 / (width - 1) as f64;
            let v = j as f64 / (height - 1) as f64;

            let r = Ray::new(camera.origin, camera.lower_left_corner + u * camera.horizontal + v * camera.vertical - camera.origin);
            let pixel_color = ray_color(&r);
            write_color(&mut file, pixel_color)?;
        }
    }
    Ok(())
}


struct Camera{
    viewport_height: f64,
    viewport_width: f64,
    focal_length : f64,

    origin: Point,
    horizontal: Vec3, 
    vertical: Vec3,
    lower_left_corner: Vec3
}

impl Camera{
    fn new(viewport_height: f64, viewport_width: f64, focal_length: f64) -> Self{
        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        Self{
            viewport_height, 
            viewport_width, 
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_length)
        }
    }
}