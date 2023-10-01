use image::RgbImage;
use ndarray::Array3;
use std::time::Instant;

use core::f32::consts::PI;
use rand::prelude::*;
use rayon::prelude::*;

use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::cmp;
use std::f32::consts::E;
use std::sync::{Arc, Mutex};

mod vec;
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

//use num;


fn lerp(a: f32, b: f32, h: f32) -> f32 {
    return a*h+b*(1.0f32-h)
} 

fn vec_to_image(img: Vec<Vec<Vec3>>, filename: &str) -> () {
    let sizey = img.len() as u32;
    let sizex = img[0].len() as u32;
    let mut imgbuf = image::ImageBuffer::new(sizex, sizey);

    for (y, row) in img.iter().enumerate() {
        for (x, pixel) in row.iter().enumerate() {
            imgbuf.put_pixel(x as u32, y as u32, image::Rgb([pixel.x as u8, pixel.y as u8, pixel.z as u8]));
        }
    }

    imgbuf.save(filename).unwrap();
}


fn ray(
    objects: &Op,
    mut ray_data: RayData,
    //start_pos: Vec3,
    //u_vec: Vec3,
    //bounce_depth: u8,
    //refractive_index: f32,
    //fog_collision_check: bool,
    //initial: bool,
) -> (Vec3, bool) {


    //let mut ray_pos = start_pos.clone();
    let mut total_color = Vec3::zeros();
    let mut hit: bool = false;
    let mut fog_length = -(1.0-rand::random::<f32>()).ln()/FOG_LAMBDA;
    let mut i=1;

    while hit == false {
        i += 1;
        if i>MAX_STEPS {
            //println!("Warning! #1");
            return (MAX_STEPS_COLOR, true)
        }

        //find the step length
        let point = objects.get_nearest_point(ray_data.ray_pos);
        let sdf_val = point.dist;
        let material_color = point.attributes.color;
        let reflectance = point.attributes.reflectance;
        let mut surface_model = point.attributes.surface_model;
        let emission_rate = point.attributes.emission_rate;
        let mut new_refractive_index = point.attributes.refractive_index;
        let step_length = sdf_val.abs()*STEP_LENGTH_MULTIPLIER;
        let mut fog_hit = false;
        let mut normal = Vec3::zeros();
        let mut cum_indirect_color = Vec3::zeros();
        let mut initial_splits_var = INITIAL_SPLITS;
        //let mut new_ray_data = ray_data.clone();
        if !ray_data.initial {
            initial_splits_var = 1
        }
        //Check if fog scatter
        if fog_length < step_length && FOG && ray_data.fog_collision_check {
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
        if Vec3::len(&ray_data.ray_pos) > MAX_DISTANCE {

            hit = false;
            let h = (0.0f32).max(ray_data.u_vec.dot(&Vec3{x:0.0, y:1.0, z:0.0}));

            let r = lerp(BACKGROUND_COLOR_1.x, BACKGROUND_COLOR_2.x, h);
            let g = lerp(BACKGROUND_COLOR_1.y, BACKGROUND_COLOR_2.y, h);
            let b = lerp(BACKGROUND_COLOR_1.z, BACKGROUND_COLOR_2.z, h);
            
            return (Vec3{x:r, y:g, z:b}, hit);
        }
        
        

        //check if hit
        if sdf_val.abs() < EPSILON1 || fog_hit {
            if ray_data.bounce_depth >= MAX_BOUNCE_DEPTH {
                return (MAX_BOUNCE_COLOR, true)
            }
            
            hit = true;
            if !fog_hit {        //find normal
                let distc = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z}).dist;        
                let distx = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x+EPSILON2, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z}).dist;                 
                let disty = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y+EPSILON2, z:ray_data.ray_pos.z}).dist;                  
                let distz = objects.get_nearest_point(Vec3{x:ray_data.ray_pos.x, y:ray_data.ray_pos.y, z:ray_data.ray_pos.z+EPSILON2}).dist;
                normal = Vec3::normalize(&Vec3{x:(distx-distc)/EPSILON2, y:(disty-distc)/EPSILON2, z:(distz-distc)/EPSILON2})*sdf_val.signum();
                
                if normal.x.is_nan() || normal.y.is_nan() || normal.z.is_nan() {
                    return (NAN_COLOR, true);
                    //println!("x{:?}",distx);
                    //println!("c{:?}",distc);
                }
            }
            if SUN_LIGHT_METHOD == 1 {
                if ray_data.refractive_index == new_refractive_index {  //TODO, BAD WAY OF DOINGS THIS
                    new_refractive_index = START_REFRACTIVE_INDEX;
                }
                ray_data.bounce_depth += 1;
                for _ in 0..initial_splits_var {
                    let indirect_color = lighting::get_indirect_lighting(
                        ray_data,
                        &objects,
                        normal,
                        reflectance,
                        surface_model,
                        new_refractive_index,
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
                    );
                    total_color = (material_color).vec_mult(&(direct_color + cum_indirect_color));
                    return (total_color, hit)
                }
                if surface_model == 2 {
                    total_color = cum_indirect_color * reflectance + material_color * (1.0 - reflectance); 
                    return (total_color, hit)
                }
                if surface_model == 3 {
                    //refreaction and reflection
                    total_color = cum_indirect_color;
                    return (total_color, hit)
                }
                if surface_model == 4 {
                    //Fog diffuse
                    let direct_color = get_direct_lighting(
                        ray_data.ray_pos,
                        ray_data.u_vec,
                        &objects,
                        normal,
                        fog_hit,
                    );
                    total_color = (FOG_COLOR).vec_mult(&(direct_color*SUN_MULTIPLIER + cum_indirect_color));
                    return (total_color, hit)
                }//tomato reflection to be implemetned...
            }

            if SUN_LIGHT_METHOD == 2 { 
                if emission_rate < 0.001 {
                    ray_data.bounce_depth += 1;
                    for _ in 0..initial_splits_var {
                        let indirect_color = lighting::get_indirect_lighting(
                            ray_data,
                            &objects,
                            normal,
                            reflectance,
                            surface_model,
                            new_refractive_index,
                        );
                        cum_indirect_color = cum_indirect_color + indirect_color/(initial_splits_var as f32);
                    }
                    if surface_model == 1 {
                        total_color = (material_color).vec_mult(&(cum_indirect_color));
                        return (total_color, hit)
                    }
                    if surface_model == 2 {
                        total_color = cum_indirect_color * reflectance + material_color * (1.0 - reflectance);
                        return (total_color, hit) 
                    }
                    if surface_model == 3 {
                        //refraction and reflection
                        total_color = cum_indirect_color;
                        return (total_color, hit)
                    }
                    if surface_model == 4 {
                        //Fog diffuse
                        let direct_color = get_direct_lighting(
                            ray_data.ray_pos,
                            ray_data.u_vec,
                            &objects,
                            normal,
                            fog_hit,
                        );
                        total_color = (FOG_COLOR).vec_mult(&(direct_color*SUN_MULTIPLIER + cum_indirect_color));
                        return (total_color, hit)
                    }
                } else {
                    total_color = material_color*emission_rate;
                    return (total_color, hit)
                }
            }
        }
    };  
    return (total_color, hit)  
}


//TODO: rays start from an area, to create depth of field

const EPSILON1: f32 = 0.004;  //hit 0.0000004
const EPSILON2: f32 = 0.001;  //normal
const EPSILON3: f32 = 0.001;  //lightning
const MAX_BOUNCE_DEPTH: u8 = 5;
const MAX_BOUNCE_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:0.0};
const NAN_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:1.0};
const MAX_DISTANCE: f32 = 100.0;
const NUM_OF_SAMPLES: i32 = 4;
const INITIAL_SPLITS: i8 = 5;

const DEPTH_OF_FIELD: bool = true;
const DEPTH_OF_FIELD_CONST: f32 = 0.2;
const FOCAL_DEPTH_DISTANCE: f32 = 4.0;
const SQUARE_DOF: bool = false;

const FOG: bool = false;
const FOG_LAMBDA: f32 = 1.0/8.0;

const NUM_BIN_WIDTH: usize = 1080/2;
//const NUM_BIN_WIDTH: usize = 720/2;
const CANVAS_WIDTH: f32 = FOCAL_DEPTH_DISTANCE*1.123;

const NUM_BIN_HEIGHT: usize = 1920/2;
//const NUM_BIN_HEIGHT: usize = 1080;
//const NUM_BIN_HEIGHT: usize = 1280/2;
const CANVAS_HEIGHT: f32 = FOCAL_DEPTH_DISTANCE*2.0;
//const CANVAS_HEIGHT: f32 = FOCAL_DEPTH_DISTANCE*1.123;

const STEP_LENGTH_MULTIPLIER: f32 = 1.0;
const SUN_LIGHT_METHOD: i8 = 1;             //TODO method 3,4. one initial ray, which then splits into multiple. 
const SUN_MULTIPLIER: f32 = 2.5;
const SUN_COLOR: Vec3 = Vec3{x:0.95, y: 0.99, z: 0.9};
const START_REFRACTIVE_INDEX: f32 = 1.0;

const MAX_STEPS: u32 = 1000;
const MAX_STEPS_COLOR: Vec3 = Vec3{x:1.0, y:0.0, z:0.0};
const TIME_APPROX: bool = true;
const TIME_APPROX_NUM: u32 = 100;
const WRITE_OPTIONS: bool = true;

const FOG_COLOR: Vec3 = Vec3{x: 0.9, y: 0.9, z: 0.9};
//const BACKGROUND_COLOR_1: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
//const BACKGROUND_COLOR_2: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
 const BACKGROUND_COLOR_1: Vec3 = Vec3 {x: 0.05, y: 0.05, z: 0.6};
 const BACKGROUND_COLOR_2: Vec3 = Vec3 {x: 0.53, y: 0.81, z: 0.92};


fn main() {
        
    let progress = Arc::new(Mutex::new(0.0));
    let now = Instant::now();

    let bin_width = CANVAS_WIDTH / (NUM_BIN_WIDTH as f32);
    let bin_height = CANVAS_HEIGHT / (NUM_BIN_HEIGHT as f32);

    let mut bin_pos_array: Array3<f32> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //x,y,z
    //let image_array: Array3<u8> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //R,G,B

    let eye_pos = Vec3::zeros();
    let canvas_pos = Vec3 {
        x: FOCAL_DEPTH_DISTANCE,
        y: 0.0f32,
        z: 0.0f32,
    };


    let objects = scene();

    //loop to find bin positions
    for ((i, j, c), v) in bin_pos_array.indexed_iter_mut() {
        *v = match c {
            0 => canvas_pos.x,                                                       //x
            1 => canvas_pos.y - CANVAS_WIDTH / 2.0 + (i as f32 + 0.5) * bin_width,   //y
            2 => canvas_pos.z - CANVAS_HEIGHT / 2.0 + (j as f32 + 0.5) * bin_height, //z
            _ => unreachable!(),
        };
    }

    if WRITE_OPTIONS {
        // Create a file
        let mut data_file = File::create("Options1.txt").expect("creation failed");
        // Write contents to the file
        let eps11 = "EPSILON1: ";
        let eps12 = EPSILON1.to_string();
        let eps13 = format!("{}{}\n", eps11, eps12);

        let eps21 = "EPSILON2: ";
        let eps22 = EPSILON2.to_string();
        let eps23 = format!("{}{}\n", eps21, eps22);

        let eps31 = "EPSILON3: ";
        let eps32 = EPSILON2.to_string();
        let eps33 = format!("{}{}\n", eps31, eps32);
        
        let ms1 = "MAX STEPS: ";
        let ms2 = MAX_STEPS.to_string();
        let ms3 = format!("{}{}\n", ms1, ms2);

        let slm1 = "STEP LENGTH MULTIPLIER: ";
        let slm2 = STEP_LENGTH_MULTIPLIER.to_string();
        let slm3 = format!("{}{}\n", slm1, slm2);
        
        data_file.write((eps13).as_bytes()).expect("write failed");
        data_file.write((eps23).as_bytes()).expect("write failed");
        data_file.write((eps33).as_bytes()).expect("write failed");
        data_file.write((ms3).as_bytes()).expect("write failed");
        data_file.write((slm3).as_bytes()).expect("write failed");

        println!("Created file Options1.txt");
    }


    //Give aproximation of time
    if TIME_APPROX {
        let now_approx = Instant::now();
        for _ in 0..TIME_APPROX_NUM {
            let i = (rand::random::<f32>()*NUM_BIN_WIDTH as f32) as usize;
            let j = (rand::random::<f32>()*NUM_BIN_HEIGHT as f32) as usize;

            let x = bin_pos_array[[i, j, 0]];
            let y = bin_pos_array[[i, j, 1]];
            let z = bin_pos_array[[i, j, 2]];
            let end_pos = Vec3{x:x, y:y, z:z};
            
            let mut vector = Vec3::zeros();  //TODO remove this line
            
            vector = end_pos - eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};

            let u_vector = Vec3::normalize(&vector);
            // (_, _) = ray(
            //     eye_pos,
            //     u_vector,
            //     &objects,
            //     0u8,
            //     START_REFRACTIVE_INDEX,
            //     true,
            //     true,
            // );
            let mut ray_data = RayData::basic();
            ray_data.ray_pos = eye_pos;
            ray_data.u_vec = u_vector;
            ray_data.refractive_index = START_REFRACTIVE_INDEX;

            (_, _) = ray(
                &objects,
                ray_data,
            );
        }
        println!("ETA: {:?}",(now_approx.elapsed()/(TIME_APPROX_NUM*2))*NUM_BIN_WIDTH as u32*NUM_BIN_HEIGHT as u32*NUM_OF_SAMPLES as u32);
    }



    //loop to shoot rays parallell     /into_par_iter().map
    let image_array: Vec<Vec<Vec3>> = (0..NUM_BIN_WIDTH).into_par_iter().map(|i| {
        let row: Vec<Vec3> = (0..NUM_BIN_HEIGHT).into_par_iter().map(|j| {

            let x = bin_pos_array[[i, j, 0]];
            let y = bin_pos_array[[i, j, 1]];
            let z = bin_pos_array[[i, j, 2]];
            let end_pos = Vec3{x:x, y:y, z:z};
            //let vector = end_pos - eye_pos;
            //let u_vector = Vec3::normalize(&vector);

            let mut color = Vec3{x:0.0, y:0.0, z:0.0};
            let mut tcolor = Vec3{x:0.0, y:0.0, z:0.0};
            
            for _k in 0..NUM_OF_SAMPLES {
                let mut vector = Vec3::zeros();  //TODO remove this line
                let mut new_eye_pos = eye_pos;
                if DEPTH_OF_FIELD {
                    let a = rand::random::<f32>();
                    let b = rand::random::<f32>();
                    let mut dy = 0.0;
                    let mut dz = 0.0;
                    if SQUARE_DOF {
                        dy = (a-0.5)*DEPTH_OF_FIELD_CONST;
                        dz = (b-0.5)*DEPTH_OF_FIELD_CONST;
                    } else {
                        dy = 0.5*(a*DEPTH_OF_FIELD_CONST).sqrt()*(b*2.0*PI).cos();
                        dz = 0.5*(a*DEPTH_OF_FIELD_CONST).sqrt()*(b*2.0*PI).sin();
                    }
                    new_eye_pos = new_eye_pos + Vec3{x:0.0, y:dy, z:dz}
                }
                vector = end_pos - new_eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};
                let u_vector = Vec3::normalize(&vector);
                // (color, _) = ray(
                //     new_eye_pos,
                //     u_vector,
                //     &objects,
                //     0u8,
                //     START_REFRACTIVE_INDEX,
                //     true,
                //     true,
                // );
                let mut ray_data = RayData::basic();
                ray_data.ray_pos = new_eye_pos;
                ray_data.u_vec = u_vector;
                ray_data.refractive_index = START_REFRACTIVE_INDEX;


                (color,_) = ray(
                    &objects,
                    ray_data,
                );

                tcolor = tcolor + color
            }
            tcolor = tcolor/NUM_OF_SAMPLES as f32;
            tcolor*255.0
        }).collect();
        let mut progress = progress.lock().unwrap();
        *progress += 1.0;
        if i%1==0 {
            print!("\rProgress: {:.3}", *progress/NUM_BIN_WIDTH as f32); 
            std::io::stdout().flush();   
        }
        row
    }).collect();

    vec_to_image(image_array, "picture1.png");
        
    let elapsed = now.elapsed();
    println!("\nTotal time: {:?}", elapsed);
}
