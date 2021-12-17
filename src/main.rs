mod utils;
mod tracer;

fn main() {
    let mut win = tracer::Window::new();
    let mut world = tracer::World::new_empty();
    world.objects.push(tracer::Sphere::new(-utils::Z_UNIT, 0.5));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new(0.0, -100.5, -1.0), 100.0));
    world.objects.push(tracer::Sphere::new(utils::Vector3::new(1.0, 5.0, -5.0), 0.5));

    let mut cam : tracer::Camera = tracer::Camera::new(utils::ORIGIN, utils::VIEWPORT_WIDTH, utils::VIEWPORT_HEIGHT, utils::FOCAL_LENGTH);

    //cam.render(&mut world, "outputx.png".to_string());

    win.update(&mut world);
}
