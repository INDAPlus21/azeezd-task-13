mod utils;
mod tracer;

fn main() {
    //let mut win = tracer::Window::new();
    let mut world = tracer::World::new_empty();

    let material_ground = tracer::Material::new(utils::Colour::new(0.8, 0.8, 0.0), tracer::MaterialType::Lambertian);
    let material_center = tracer::Material::new(utils::Colour::new(0.1, 0.2, 0.8), tracer::MaterialType::Lambertian);
    let material_left = tracer::Material::new(utils::Colour::new(0.8, 0.8, 0.8), tracer::MaterialType::Dielectric(1.2));
    let material_right = tracer::Material::new(utils::Colour::new(0.8, 0.6, 0.2), tracer::MaterialType::Metal(0.5));
    
    world.objects.push(tracer::Sphere::new(utils::Vector3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new( 0.0, 0.0, -1.0), 0.5, material_center));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new(-1.0, 0.0, -1.0), -0.5, material_left));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new( 1.0, 0.0, -1.0), 0.5, material_right));


    let mut cam : tracer::Camera = tracer::Camera::new(utils::ORIGIN, utils::VIEWPORT_WIDTH, utils::VIEWPORT_HEIGHT, utils::FOCAL_LENGTH);

    //cam.render(&mut world, "outputx.png".to_string());

    cam.render(&mut world, "outputz.png".to_string());
}
