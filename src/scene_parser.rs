use crate::tracer::{Axis, Camera, Material, MaterialType, Rectangle, Sphere, World};
use crate::utils::{Colour, Vector3, ASPECT_RATIO, ORIGIN, Z_UNIT};
use rand::Rng;
use std::{collections::HashMap, fs};

pub struct Parser {
    pub file_path: String,
    pub world: World,
    pub camera: Camera
}

impl Parser {
    pub fn new(file_path: &String) -> Parser {
        Parser {
            file_path: file_path.to_string(),
            world: World::new_empty(),
            camera: Camera::new(ORIGIN, Z_UNIT, 90.0, ASPECT_RATIO)
        }
    }
    pub fn parse(&mut self) {
        self.world = World::new_empty();
        let mut materials: HashMap<String, Material> = HashMap::new();
        match fs::read_to_string(&self.file_path) {
            Ok(_text) => {
                let lines: Vec<String> = _text
                    .lines()
                    .map(|_line| _line.trim().to_string())
                    .filter(|_line| _line.len() > 0)
                    .collect();

                let mut big_loop_idx: usize = 0;
                let mut in_loop_idx: usize = 0;
                while big_loop_idx < lines.len() {
                    let _line = lines.get(big_loop_idx).unwrap();
                    let data: Vec<String> = _line
                        .split_whitespace()
                        .map(|_token| _token.to_string())
                        .collect();

                    match data.get(0).unwrap().as_str() {
                        "CAM" => {
                            self.camera = Self::parse_cam(&data);
                        }
                        "MAT" => {
                            let mat = Self::parse_mat(&data);

                            materials.insert(mat.0, mat.1);
                        }
                        "OBJ" => match data[1].as_str() {
                            "sphere" => {
                                Self::parse_sphere(&data, &mut self.world, &mut materials);
                            }
                            "rect" => {
                                Self::parse_rect(&data, &mut self.world, &mut materials);
                            }
                            _ => {
                                panic!("No or unknown object type given")
                            }
                        },
                        "~" => {
                            in_loop_idx = data[1].parse::<usize>().unwrap();
                            big_loop_idx += 1;
                        }
                        _ => {}
                    }
                    if in_loop_idx > 0 {
                        in_loop_idx -= 1;
                    } else {
                        big_loop_idx += 1;
                    }
                }
            }

            Err(_error) => {
                panic!("Error encountered while reading {}", self.file_path)
            }
        }
    }

    fn parse_cam(data: &Vec<String>) -> Camera {
        let data: Vec<f32> = data[1..]
            .iter()
            .map(|_val| Self::get_val(_val.to_string()))
            .collect::<Vec<f32>>();
        let from = Vector3::new(data[0], data[1], data[2]);
        let to = Vector3::new(data[3], data[4], data[5]);
        let fov = data[6];
        Camera::new(from, to, fov, ASPECT_RATIO)
    }

    fn parse_sphere(
        data: &Vec<String>,
        world: &mut World,
        materials: &mut HashMap<String, Material>,
    ) {
        let mat_name = data[2].to_string();
        let center = Vector3::from_vec(
            data[3..6]
                .iter()
                .map(|_val| Self::get_val(_val.to_string()))
                .collect::<Vec<f32>>(),
        )
        .unwrap();
        let radius = Self::get_val(data[6].to_string());

        world.objects.push(Sphere::new(
            center,
            radius,
            *materials.get(&mat_name).unwrap(),
        ));
    }

    fn parse_rect(
        data: &Vec<String>,
        world: &mut World,
        materials: &mut HashMap<String, Material>,
    ) {
        let mat_name = data[2].to_string();
        let axis = match data[3].as_str() {
            "xy" => Axis::XY,
            "xz" => Axis::XZ,
            "yz" => Axis::YZ,
            _ => {
                panic!("No or unknown axis given")
            }
        };

        let coord1 = data[4..6]
            .iter()
            .map(|_val| Self::get_val(_val.to_string()))
            .collect::<Vec<f32>>();

        let coord2 = data[6..8]
            .iter()
            .map(|_val| Self::get_val(_val.to_string()))
            .collect::<Vec<f32>>();

        let depth = data[8].parse::<f32>().unwrap();

        world.objects.push(Rectangle::new(
            axis,
            (coord1[0], coord1[1]),
            (coord2[0], coord2[1]),
            depth,
            *materials.get(&mat_name).unwrap(),
        ));
    }

    fn parse_mat(data: &Vec<String>) -> (String, Material) {
        let colour: Colour = Colour::from_vec(
            data[3..6]
                .iter()
                .map(|_val| Self::get_val(_val.to_string()))
                .collect::<Vec<f32>>(),
        )
        .unwrap();

        let extra_val = match data.get(6) {
            Some(e) => Self::get_val(e.to_string()),
            _ => 1.0,
        };

        let name = data.get(1).unwrap().to_string();
        let mat_type = match data[2].as_str() {
            "metal" => MaterialType::Metal(extra_val),
            "lambertian" => MaterialType::Lambertian,
            "dielectric" => MaterialType::Dielectric(extra_val),
            "light" => MaterialType::DiffuseLight,
            _ => {
                panic!("No or unknown material given")
            }
        };

        (name, Material::new(colour, mat_type))
    }

    fn get_val(value: String) -> f32 {
        let vals: Vec<f32> = value
            .split('_')
            .map(|_val| _val.parse::<f32>().unwrap())
            .collect();
        let mut rng = rand::thread_rng();

        match vals.len() {
            1 => vals[0],
            2 => {rng.gen_range(vals[0]..=vals[1])},
            _ => panic!("Error parsing range"),
        }
    }
}
