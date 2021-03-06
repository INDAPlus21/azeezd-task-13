mod utils;
mod tracer;
mod scene_parser;
use std::env;

fn main() {
    let mut args = env::args();

    // Fast preview rendering
    let mut fast = false;

    let mut samples = utils::SAMPLES;
    let mut max_bounce = utils::BOUNCE_AMOUNT;

    let scene_file_path = args.nth(1).unwrap().to_string();
    let mut output_path = "out".to_string();

    for arg in args {
        match arg.get(..2) {
            Some("-f") => {fast = true;},
            Some("-s") => {samples = arg.get(2..).unwrap().parse::<usize>().unwrap();}
            Some("-b") => {max_bounce = arg.get(2..).unwrap().parse::<usize>().unwrap();}
            Some("-@") => {output_path = arg.get(2..).unwrap().to_string()}
            _ => {println!("Unknown argument {}", arg)}
        }
    }

    let mut parser = scene_parser::Parser::new(&scene_file_path.to_string());
    parser.parse();

    if fast {
        println!("Fast render of {} started.", scene_file_path);
        parser.camera.fast_render(&mut parser.world, output_path.as_str())
            .expect("Error While Rendering");
    } else {
        println!("Render of {} started. {} Samples, {} Light Bounces.", scene_file_path, samples, max_bounce);
        parser.camera.render(&mut parser.world, output_path.as_str(), samples, max_bounce)
            .expect("Error While Rendering");
    }
}
