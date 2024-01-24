use image::RgbImage;
use lighting::get_rayleigh_color;
use ndarray::Array3;
use std::time::Instant;
use std::time::Duration;

use core::f32::consts::PI;
use rand::prelude::*;
use rayon::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::f32::consts::E;
use std::sync::{Arc, Mutex};

use std::io::BufReader;
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::{SineWave, Source};

mod vec;
use crate::camera::get_camera;
use crate::vec::{Vec2, RayData};
use crate::vec::Vec3;

mod lighting;
use crate::lighting::get_direct_lighting;
use crate::lighting::get_indirect_lighting;

mod operations;
use crate::operations::Op;
// use crate::Op::Move;
// use crate::Op::Scale;
// use crate::Op::SinDistortHeight;
// use crate::Op::Sphere;
// use crate::Op::Cube;
// use crate::Op::CappedCone;
// use crate::Op::Line;
// use crate::Op::RotateY;

mod scenes;
use crate::scenes::scene;

mod camera;

//use num;

#[derive(Debug, Clone, Copy)]
pub struct Settings {
    pub num_of_samples: i32,
    pub eps1: f32, //hit
    pub eps2: f32, //normal
    pub eps3: f32, //lightning
    pub max_bounce_depth: u8,
    pub max_bounce_color: Vec3,
    pub nan_color: Vec3,
    pub max_distance: f32,
    pub initial_splits: i8,
    pub frames: i32,
    pub depth_of_field: bool,
    pub depth_of_field_const: f32,
    pub focal_depth_distance: f32,
    pub square_dof: bool,
    pub rgb_rays: bool,
    pub fog: bool,
    pub fog_lambda: f32,
    pub num_bin_height: usize,
    pub canvas_height: f32, 
    pub num_bin_width: usize, 
    pub canvas_width: f32,
    pub step_length_multiplier: f32,
    pub sun_light_method: i8,
    pub sun_multiplier: f32,
    pub sun_color: Vec3,
    pub sun_position: Vec3,
    pub sun_radius: f32,
    pub start_refractive_index: f32,
    pub max_steps: u32,
    pub max_steps_color: Vec3,
    pub time_approx: bool,
    pub time_approx_num: u32,
    pub write_options: bool,
    pub fog_color: Vec3,
    pub background_color_1: Vec3,
    pub background_color_2: Vec3,
    pub frame_start: i32,
}

impl Settings {
    pub fn basic() -> Settings {
        Settings {
            num_of_samples: 50,
            eps1: 0.00001, //hit
            eps2: 0.00001, //normal
            eps3: 0.00001, //lightning
            max_bounce_depth: 10,
            max_bounce_color: Vec3::zeros(),
            nan_color: Vec3::zeros(),
            max_distance: 200.0,
            initial_splits: 1,
            frames: 1,
            depth_of_field: false,
            depth_of_field_const: 0.2,
            focal_depth_distance: 2.0,
            square_dof: false,
            rgb_rays: false,  //TODO wavelength ist
            fog: false,
            fog_lambda: 1.0/15.0,
            num_bin_height: 1080,
            canvas_height: 2.0*1.125,
            num_bin_width: 1920, 
            canvas_width:  2.0*2.0,
            step_length_multiplier: 1.0,
            sun_light_method: 1,
            sun_multiplier: 1.0,
            sun_color: Vec3{x:0.95, y: 0.99, z: 0.9},
            sun_position: Vec3{x:-100.0, y:-100.0, z:-100.0},
            sun_radius: 0.08,
            start_refractive_index: 1.0,
            max_steps: 1000,
            max_steps_color: Vec3::zeros(),
            time_approx: true,
            time_approx_num: 10,
            write_options: true,
            fog_color: Vec3::ones(),
            background_color_1: Vec3{x: 0.05, y: 0.05, z: 0.6},
            background_color_2: Vec3{x: 0.53, y: 0.81, z: 0.92},
            //background_color_1: Vec3{x: 0.0, y: 0.0, z: 0.0},
            //background_color_2: Vec3{x: 0.0, y: 0.0, z: 0.0},
            frame_start: 0,
        }
    }
}



// fn lerp(a: f32, b: f32, h: f32) -> f32 {
//     let h = h.max(0.0).min(1.0);
//     return a*h+b*(1.0f32-h)
// } 

fn lerp(start: f32, end: f32, t: f32) -> f32 {
    let t = t.max(0.0).min(1.0); // Ensure t is between 0.0 and 1.0
    start + (end - start) * t
}

fn vec_to_image(img: Vec<Vec<Vec3>>, filename: &String) -> () {
    let sizey = img.len() as u32;
    let sizex = img[0].len() as u32;
    let mut imgbuf = image::ImageBuffer::new(sizex, sizey);

    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    imgbuf = image::imageops::rotate270(&imgbuf);

    imgbuf.save(filename).unwrap();
}


fn ray(
    objects: &Op,
    mut ray_data: RayData,
    mut settings: Settings,
) -> (Vec3, bool, f32) {

    //let mut ray_pos = start_pos.clone();
    let mut total_color = Vec3::zeros();
    let mut hit: bool = false;
    let mut fog_length = -(1.0-rand::random::<f32>()).ln()/settings.fog_lambda;
    let mut i=1;

    while hit == false {
        i += 1;
        if i > settings.max_steps {
            //println!("Warning! #1");
            return (settings.max_steps_color, true, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
        }

        //find the step length
        let point = objects.get_nearest_point(ray_data.ray_pos);
        let sdf_val = point.dist;
        let material_color = point.attributes.color;
        let reflectance = point.attributes.reflectance;
        let mut surface_model = point.attributes.surface_model;
        let emission_rate = point.attributes.emission_rate;
        let mut new_refractive_index = point.attributes.refractive_index;
        let step_length = sdf_val.abs()*settings.step_length_multiplier;
        let mut fog_hit = false;
        let mut normal = Vec3::zeros();
        let mut cum_indirect_color = Vec3::zeros();
        let mut initial_splits_var = settings.initial_splits;

        //let mut new_ray_data = ray_data.clone();
        if !ray_data.initial {
            initial_splits_var = 1
        }
        //Check if fog scatter
        if fog_length < step_length && settings.fog && ray_data.fog_collision_check {
            //Take the step, but encounter a particle at a random 
            ray_data.ray_pos = ray_data.ray_pos + ray_data.u_vec * fog_length;
            surface_model = 4i8;
            fog_hit = true;
        } else {
            //take the step
            ray_data.ray_pos = ray_data.ray_pos + ray_data.u_vec*step_length;
            fog_length -= step_length;
        }  

        //check if outside scene
        if Vec3::len(&ray_data.ray_pos) > settings.max_distance {

            //let rayleigh_color = get_rayleigh_color(ray_data.origin, ray_data.ray_pos);
            //return (rayleigh_color, false); 

            let h = (0.0f32).max(ray_data.u_vec.dot(&Vec3{x:0.0, y:1.0, z:0.0}));

            let r = lerp(settings.background_color_2.x, settings.background_color_1.x, h);
            let g = lerp(settings.background_color_2.y, settings.background_color_1.y, h);
            let b = lerp(settings.background_color_2.z, settings.background_color_1.z, h);
            
            return (Vec3{x:r, y:g, z:b}, false, Vec3::len(&(ray_data.ray_pos - ray_data.origin)));
        }
        

        //check if hit
        if sdf_val.abs() < settings.eps1 || fog_hit {
            if ray_data.bounce_depth >= settings.max_bounce_depth {
                return (settings.max_bounce_color, true, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
            }
            
            
            //let rayleigh_color = get_rayleigh_color(ray_data.origin, ray_data.ray_pos);
            //return (rayleigh_color, true);  


            hit = true;
            ray_data.bounce_depth += 1;
            ray_data.origin = ray_data.ray_pos;

            if !fog_hit {        //find normal
                let distc = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z}).dist;        
                let distx = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x+settings.eps2, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z}).dist;                 
                let disty = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y+settings.eps2, z:ray_data.ray_pos.z}).dist;                  
                let distz = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z+settings.eps2}).dist;
                normal = Vec3::normalize(&Vec3{x:(distx-distc)/settings.eps2, y:(disty-distc)/settings.eps2, z:(distz-distc)/settings.eps2})*sdf_val.signum();
                
                if normal.x.is_nan() || normal.y.is_nan() || normal.z.is_nan() {
                    return (settings.nan_color, true, Vec3::len(&(ray_data.ray_pos - ray_data.origin)));
                    //println!("x{:?}",distx);
                    //println!("c{:?}",distc);
                }
            }
            if settings.sun_light_method == 1 {
                if ray_data.refractive_index == new_refractive_index {  //TODO, BAD WAY OF DOINGS THIS
                    new_refractive_index = settings.start_refractive_index;
                }
                for _ in 0..initial_splits_var {
                    let indirect_color = lighting::get_indirect_lighting(
                        ray_data,
                        &objects,
                        normal,
                        reflectance,
                        surface_model,
                        new_refractive_index,
                        settings,
                    );
                    cum_indirect_color = cum_indirect_color + indirect_color/(initial_splits_var as f32);
                }
                if surface_model == 1 {  //TODO detta kanns fel...
                    let direct_color = get_direct_lighting(
                        ray_data.ray_pos,
                        ray_data.u_vec,
                        &objects,
                        normal,
                        fog_hit,
                        settings,
                    );
                    total_color = (material_color).vec_mult(&(direct_color + cum_indirect_color));
                    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                }
                if surface_model == 2 {
                    total_color = cum_indirect_color * reflectance + material_color * (1.0 - reflectance); 
                    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                }
                if surface_model == 3 {
                    //refreaction and reflection
                    total_color = cum_indirect_color.vec_mult(&material_color);
                    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                }
                if surface_model == 4 {
                    //Fog diffuse
                    //let rayleigh_color = get_rayleigh_color(ray_data.origin, ray_data.ray_pos);
                    //return (rayleigh_color, true);   
                    //return (rayleigh_color, true)

                    let mut direct_color = get_direct_lighting(
                        ray_data.ray_pos,
                        ray_data.u_vec,
                        &objects,
                        normal,
                        fog_hit,
                        settings,
                    );

                    total_color = (settings.fog_color).vec_mult(&(direct_color*settings.sun_multiplier + cum_indirect_color));
                    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                    
                }//tomato reflection to be implemetned...
            }

            if settings.sun_light_method == 2 { 
                if emission_rate < 0.001 {
                    for _ in 0..initial_splits_var {
                        let indirect_color = lighting::get_indirect_lighting(
                            ray_data,
                            &objects,
                            normal,
                            reflectance,
                            surface_model,
                            new_refractive_index,
                            settings,
                        );
                        cum_indirect_color = cum_indirect_color + indirect_color/(initial_splits_var as f32);
                    }
                    if surface_model == 1 {
                        total_color = (material_color).vec_mult(&(cum_indirect_color));
                        return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                    }
                    if surface_model == 2 {
                        total_color = cum_indirect_color * reflectance + material_color * (1.0 - reflectance);
                        return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin))) 
                    }
                    if surface_model == 3 {
                        //refraction and reflection
                        total_color = cum_indirect_color.vec_mult(&material_color);
                        return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                    }
                    if surface_model == 4 {
                        //Fog diffuse
                        let direct_color = get_direct_lighting(
                            ray_data.ray_pos,
                            ray_data.u_vec,
                            &objects,
                            normal,
                            fog_hit,
                            settings,
                        );
                        total_color = (settings.fog_color).vec_mult(&(direct_color*settings.sun_multiplier + cum_indirect_color));
                        return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                        //return (rayleigh_color, true)
                    }
                } else {
                    total_color = material_color*emission_rate;
                    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))
                }
            }
        }
    };  
    return (total_color, hit, Vec3::len(&(ray_data.ray_pos - ray_data.origin)))  
}


// const EPSILON1: f32 = 0.000007;  //hit 0.0000004
// const EPSILON2: f32 = 0.000007;  //normal
// const EPSILON3: f32 = 0.000007;  //lightning
// const MAX_BOUNCE_DEPTH: u8 = 8;
// const MAX_BOUNCE_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:0.0};
// const NAN_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:1.0};
// const MAX_DISTANCE: f32 = 200.0;
// //const NUM_OF_SAMPLES: i32 = 1;
// const INITIAL_SPLITS: i8 = 1;
// const FRAMES: i32 = 1;

// const DEPTH_OF_FIELD: bool = false;
// const DEPTH_OF_FIELD_CONST: f32 = 0.02;
// const FOCAL_DEPTH_DISTANCE: f32 = 3.0;
// const SQUARE_DOF: bool = false;

// const RGB_RAYS: bool = false;
// //const RGB_STEPS: i8 = 3;

// const FOG: bool = false;
// const FOG_LAMBDA: f32 = 1.0/15.0;

// //const NUM_BIN_HEIGHT: usize = 1080/2;
// const NUM_BIN_HEIGHT: usize = 720;
// const CANVAS_HEIGHT: f32 = FOCAL_DEPTH_DISTANCE*1.125;

// //const NUM_BIN_WIDTH: usize = 1920/2;
// const NUM_BIN_WIDTH: usize = 1280;
// const CANVAS_WIDTH: f32 = FOCAL_DEPTH_DISTANCE*2.0;
// //const CANVAS_WIDTH: f32 = FOCAL_DEPTH_DISTANCE*1.125;

// const STEP_LENGTH_MULTIPLIER: f32 = 1.0;
// const SUN_LIGHT_METHOD: i8 = 1; 
// const SUN_MULTIPLIER: f32 = 1.0;
// //const SUN_COLOR: Vec3 = Vec3{x:0.95, y: 0.99, z: 0.9};
// const SUN_COLOR: Vec3 = Vec3{x:0.95, y: 0.99, z: 0.9};
// const START_REFRACTIVE_INDEX: f32 = 1.0;

// const MAX_STEPS: u32 = 1200;
// const MAX_STEPS_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:1.0};
// const TIME_APPROX: bool = true;
// const TIME_APPROX_NUM: u32 = 10;
// const WRITE_OPTIONS: bool = true;

// const FOG_COLOR: Vec3 = Vec3{x: 0.9, y: 0.9, z: 0.9};
// //const SUN_POSITION: Vec3 = Vec3{x:-100.0, y:-100.0, z:100.0};
// //const BACKGROUND_COLOR_1: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
// //const BACKGROUND_COLOR_2: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
// const BACKGROUND_COLOR_1: Vec3 = Vec3 {x: 0.05, y: 0.05, z: 0.6};
// const BACKGROUND_COLOR_2: Vec3 = Vec3 {x: 0.53, y: 0.81, z: 0.92};

const RAYLEIGH: bool = false;
const ATMOSPHERE_HEIGHT: f32 = 100.0;
const DENSITY_FALLOFF: f32 = 10.0;
const SCATTERING_STRENGTH: f32 = 0.01;
const WAVELENGTHS: Vec3 = Vec3{
    x:700.0,
    y:530.0,
    z:440.0,
};
const SCATTER_COEFF: Vec3 = Vec3{
    x: 0.1 * SCATTERING_STRENGTH,
    y: 0.32 * SCATTERING_STRENGTH,
    z: 0.68 * SCATTERING_STRENGTH,
};


fn main() {
    let mut settings = Settings::basic();
    settings.frames = 1;

    for frame in 0..settings.frames {

        println!("Current frame: {:?}", frame);
        let t = frame as f32 / settings.frames as f32;
        let objects = scene(t);
        let camera = get_camera(t);
        //settings.sun_position = camera.position;
        //settings.sun_position = Vec3{x:lerp(camera.position.x, -10.0, t), y:lerp(camera.position.y, -10.0, t),z:lerp(camera.position.z, -10.0, t)};// + up_dir*0.05;
        // settings.sun_position = Vec3 {x:-10.0, y:-10.0, z:-10.0};
        // settings.eps1 = 0.0001;
        // settings.eps2 = 0.0001;
        // settings.eps3 = 0.0001;
        // settings.num_bin_height = 720;
        // settings.num_bin_width = 1280;
        // settings.num_of_samples = 20;
        // settings.depth_of_field = true;
        // settings.focal_depth_distance = 3.2;
        // settings.depth_of_field_const = 0.05;
        // settings.canvas_width = settings.focal_depth_distance * 2.0;
        // settings.canvas_height = settings.focal_depth_distance * 1.125;
        // //settings.frame_start = 0;
        // settings.frame_start = 0;
        // //settings.sun_radius = 

        let progress = Arc::new(Mutex::new(0.0));
        let now = Instant::now();

        let bin_width = settings.canvas_width / (settings.num_bin_width as f32);
        let bin_height = settings.canvas_height / (settings.num_bin_height as f32);

        let mut bin_pos_array: Array3<f32> = Array3::zeros((settings.num_bin_width, settings.num_bin_height, 3)); //x,y,z
        
        let right_dir = Vec3::normalize(&Vec3::cross(&camera.direction, &Vec3{x:0.0, y:-1.0, z:0.0}));
        let up_dir = Vec3::normalize(&Vec3::cross(&right_dir, &camera.direction));


        for ((i, j, c), v) in bin_pos_array.indexed_iter_mut() {
            *v = match c {
                0 => camera.position.x + camera.direction.x * settings.focal_depth_distance + right_dir.x * (-settings.canvas_width / 2.0 + (i as f32 + 0.5) * bin_width) + up_dir.x * (-settings.canvas_height / 2.0 + (j as f32 + 0.5) * bin_height),
                1 => camera.position.y + camera.direction.y * settings.focal_depth_distance + up_dir.y * (-settings.canvas_height / 2.0 + (j as f32 + 0.5) * bin_height),
                2 => camera.position.z + camera.direction.z * settings.focal_depth_distance + right_dir.z * (-settings.canvas_width / 2.0 + (i as f32 + 0.5) * bin_width) + up_dir.z * (-settings.canvas_height / 2.0 + (j as f32 + 0.5) * bin_height),
                _ => unreachable!(),
                
            };
        }
        
        //println!("{bin_pos_array}");
        //pixel_pos = camera.pos + view_dir * FOCAL_DEPTH_DISTANCE + right_dir * (CANVAS_WIDTH * (i / CANVAS_WIDTH) - 0.5) + up_dir * (CANVAS_HEIGHT * (j / CANVAS_HEIGHT) - 0.5);

        if settings.write_options {
            // Create a file
            let mut data_file = File::create("Options1.txt").expect("creation failed");
            // Write contents to the file
            let eps11 = "EPSILON1: ";
            let eps12 = settings.eps1.to_string();
            let eps13 = format!("{}{}\n", eps11, eps12);

            let eps21 = "EPSILON2: ";
            let eps22 = settings.eps2.to_string();
            let eps23 = format!("{}{}\n", eps21, eps22);

            let eps31 = "EPSILON3: ";
            let eps32 = settings.eps3.to_string();
            let eps33 = format!("{}{}\n", eps31, eps32);
            
            let ms1 = "MAX STEPS: ";
            let ms2 = settings.max_steps.to_string();
            let ms3 = format!("{}{}\n", ms1, ms2);

            let slm1 = "STEP LENGTH MULTIPLIER: ";
            let slm2 = settings.step_length_multiplier.to_string();
            let slm3 = format!("{}{}\n", slm1, slm2);
            
            data_file.write((eps13).as_bytes()).expect("write failed");
            data_file.write((eps23).as_bytes()).expect("write failed");
            data_file.write((eps33).as_bytes()).expect("write failed");
            data_file.write((ms3).as_bytes()).expect("write failed");
            data_file.write((slm3).as_bytes()).expect("write failed");

            println!("Created file Options1.txt");
        }


        //Give aproximation of time
        if settings.time_approx {
            let now_approx = Instant::now();
            for _ in 0..settings.time_approx_num {
                let i = (rand::random::<f32>()*settings.num_bin_width as f32) as usize;
                let j = (rand::random::<f32>()*settings.num_bin_height as f32) as usize;

                let x = bin_pos_array[[i, j, 0]];
                let y = bin_pos_array[[i, j, 1]];
                let z = bin_pos_array[[i, j, 2]];
                let end_pos = Vec3{x:x, y:y, z:z};
                
                let mut vector = Vec3::zeros();  //TODO remove this line
                
                vector = end_pos - camera.position + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};

                let u_vector = Vec3::normalize(&vector);

                let mut ray_data = RayData::basic();
                ray_data.ray_pos = camera.position;
                ray_data.u_vec = u_vector;
                ray_data.refractive_index = settings.start_refractive_index;

                (_, _, _) = ray(
                    &objects,
                    ray_data,
                    settings,
                );
            }
            println!("ETA: {:?}",(now_approx.elapsed()/(settings.time_approx_num*4))*settings.num_bin_width as u32*settings.num_bin_height as u32*settings.num_of_samples as u32);
        }



        //loop to shoot rays parallell
        let image_array: Vec<Vec<Vec3>> = (0..settings.num_bin_width).into_par_iter().map(|i| {
            let row: Vec<Vec3> = (0..settings.num_bin_height).into_iter().map(|j| {

                let x = bin_pos_array[[i, j, 0]];
                let y = bin_pos_array[[i, j, 1]];
                let z = bin_pos_array[[i, j, 2]];
                let end_pos = Vec3{x:x, y:y, z:z};
                //let vector = end_pos - eye_pos;
                //let u_vector = Vec3::normalize(&vector);

                let mut color = Vec3{x:0.0, y:0.0, z:0.0};
                let mut tcolor = Vec3{x:0.0, y:0.0, z:0.0};
                
                for _k in 0..settings.num_of_samples {
                    //let mut vector = Vec3::zeros();  //TODO remove this line
                    let mut new_eye_pos = camera.position;
                    if settings.depth_of_field {
                        let a = rand::random::<f32>();
                        let b = rand::random::<f32>();
                        let mut dy = 0.0;
                        let mut dz = 0.0;
                        if settings.square_dof {
                            dy = (a-0.5)*settings.depth_of_field_const;
                            dz = (b-0.5)*settings.depth_of_field_const;
                        } else {
                            dy = 0.5*(a*settings.depth_of_field_const).sqrt()*(b*2.0*PI).cos();
                            dz = 0.5*(a*settings.depth_of_field_const).sqrt()*(b*2.0*PI).sin();
                        }
                        new_eye_pos = new_eye_pos + right_dir*dz + up_dir*dy;
                    }
                    //let vector = end_pos - new_eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};  //TODO THIS IS WRONG
                    let vector = end_pos - new_eye_pos + right_dir*bin_width*(rand::random::<f32>()-0.5) + up_dir*bin_height*(rand::random::<f32>()-0.5);

                    let u_vector = Vec3::normalize(&vector);

                    if settings.rgb_rays {  //Shoot RGB-rays
                        //for i in 0..RGB_STEPS {
                        //    let value = i as f32 / RGB_STEPS as f32;
                        for i in [0.25, 0.5, 0.75] {
                            let mut ray_data = RayData::basic();
                            ray_data.color = Vec3::rainbow_colors(i);
                            ray_data.pol_angle = rand::random::<f32>()*2.0*PI;
                            ray_data.ray_pos = new_eye_pos;
                            ray_data.origin = new_eye_pos;
                            ray_data.u_vec = u_vector;
                            ray_data.refractive_index = settings.start_refractive_index;

                            (color,_,_) = ray(
                                &objects,
                                ray_data,
                                settings,
                            );
                            tcolor = tcolor + color.vec_mult(&ray_data.color)
                        }
                    } else {
                        let mut ray_data = RayData::basic();
                        ray_data.color = Vec3{x:1.0, y:1.0, z:1.0};
                        ray_data.ray_pos = new_eye_pos;
                        ray_data.u_vec = u_vector;
                        ray_data.refractive_index = settings.start_refractive_index;

                        (color,_,_) = ray(
                            &objects,
                            ray_data,
                            settings,
                        );
                        tcolor = tcolor + color
                    }
                }
                tcolor = tcolor/settings.num_of_samples as f32;
                tcolor*255.0
            }).collect();
            let mut progress = progress.lock().unwrap();
            *progress += 1.0;
            if i%1==0 {
                print!("\rProgress: {:.3}", *progress/settings.num_bin_width as f32); 
                std::io::stdout().flush();   
            }
            row
        }).collect();


        // //Play a finished sound
        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // let sink = Sink::try_new(&stream_handle).unwrap();

        // // Add a dummy source of the sake of the example.
        // let source = SineWave::new(1000.0).take_duration(Duration::from_secs_f32(0.25)).amplify(1.00);
        // sink.append(source);

        // // The sound plays in a separate thread. This call will block the current thread until the sink
        // // has finished playing all its queued sounds.
        // sink.sleep_until_end();

        
        let sn1 = (frame+settings.frame_start).to_string();
        let sn2 = ".png";
        let sn3 = format!("{}{}", sn1, sn2);
        vec_to_image(image_array, &sn3);
        //vec_to_image(image_array, "picture1.png");
            
        let elapsed = now.elapsed();
        println!("\nTotal time: {:?}", elapsed);
    }
}
