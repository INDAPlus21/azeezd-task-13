mod utils;
mod tracer;

use std::time::Instant;

fn main() {

    let start = Instant::now();
    //let mut win = tracer::Window::new();
    let mut world = tracer::World::new_empty();

    let green = tracer::Material::new(utils::Colour::new(0.15, 0.5, 0.15), tracer::MaterialType::Lambertian);
    let red = tracer::Material::new(utils::Colour::new(0.5, 0.15, 0.15), tracer::MaterialType::Lambertian);
    let white = tracer::Material::new(utils::Colour::new(0.73, 0.73, 0.73), tracer::MaterialType::Lambertian);
    let light = tracer::Material::new(utils::Colour::new(15.0, 15.0, 15.0), tracer::MaterialType::DiffuseLight);
    let metal = tracer::Material::new(utils::Colour::new(1.0, 1.0, 1.0), tracer::MaterialType::Metal(0.3));
    
    world.objects.push(tracer::Rectangle::new(tracer::Axis::YZ, (0.0, 555.0), (0.0, 555.0), 555.0, green));
    world.objects.push(tracer::Rectangle::new(tracer::Axis::YZ, (0.0, 555.0), (0.0, 555.0), 0.0, red));
    world.objects.push(tracer::Rectangle::new(tracer::Axis::XZ, (213.0, 343.0), (227.0, 332.0), 554.0, light));
    world.objects.push(tracer::Rectangle::new(tracer::Axis::XZ, (0.0, 555.0), (0.0, 555.0), 0.0, white));
    world.objects.push(tracer::Rectangle::new(tracer::Axis::XZ, (0.0, 555.0), (0.0, 555.0), 555.0, white));
    world.objects.push(tracer::Rectangle::new(tracer::Axis::XY, (0.0, 555.0), (0.0, 555.0), 555.0, white));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new(250.0, 70.0, 250.0), 70.0, metal));



    let mut cam : tracer::Camera = tracer::Camera::new(utils::Vector3::new(278.0, 278.0, -800.0), utils::Vector3::new(278.0, 278.0, 0.0), 40.0, utils::ASPECT_RATIO);

    cam.render(&mut world, "outputz2.png".to_string());

    println!("Render finished. Took: {:.3}s", start.elapsed().as_secs());
}
