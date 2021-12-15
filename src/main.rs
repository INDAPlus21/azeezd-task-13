mod utils;
mod tracer;

fn main() {

    for i in 0..10 {
        let mut cam = tracer::Camera::new(utils::Vector3::new(0.0, 0.0, 0.0), utils::VIEWPORT_WIDTH, utils::VIEWPORT_HEIGHT, utils::FOCAL_LENGTH);
        let mut world = tracer::World::new_empty();
        world.objects.push(tracer::Sphere::new(utils::Vector3::new(0.0, 0.0, -1.0), 0.5));
        world.objects.push(tracer::Sphere::new(utils::Vector3::new(0.0, -100.5, -1.0), 100.0));
        world.objects.push(tracer::Sphere::new(utils::Vector3::new(i as f32, 5.0, -5.0), 0.5));
        cam.render(&mut world, format!("output{}.png", i).to_string());
    }
    
}
