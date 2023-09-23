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
use crate::vec::Vec2;
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
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    bounce_depth: u8,
    refractive_index: f32,
    background_color_1: Vec3,
    background_color_2: Vec3,
    fog_color: Vec3,
    fog_collision_check: bool,
) -> (Vec3, bool) {

    

    let mut ray_pos = start_pos.clone();
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
        let point = objects.get_nearest_point(ray_pos);
        let sdf_val = point.dist;
        let material_color = point.color;
        let reflectance = point.reflectance;
        let surface_model = point.surface_model;
        let emission_rate = point.emission_rate;
        let mut new_refractive_index = point.refractive_index;
        let step_length = sdf_val.abs()*STEP_LENGTH_MULTIPLIER;
        

        //Check if fog scatter
        if FOG && fog_collision_check {
            if fog_length < step_length  {
                if bounce_depth >= MAX_BOUNCE_DEPTH {
                    return (Vec3{x:0.0, y:0.0, z:0.0}, true)
                }
                
                //Take the step, but encounter a particle at a random distance
                ray_pos = ray_pos + u_vec * fog_length;

                if SUN_LIGHT_METHOD == 1{
                    let indirect_color = lighting::get_indirect_lighting(
                        ray_pos,
                        u_vec,
                        &objects,
                        refractive_index,
                        bounce_depth + 1u8,
                        Vec3{x: 1.0, y: 1.0, z: 1.0},  //WRONG TODO
                        1.0f32,
                        4i8,
                        point.refractive_index,
                        background_color_1,
                        background_color_2,
                        fog_color,
                    );
                    
                    let direct_color = get_direct_lighting(
                        ray_pos,
                        u_vec,
                        &objects,
                        Vec3{x: 0.0, y: 0.0, z: 0.0},
                        true,
                    );
                    
                    
                    total_color = (fog_color/255.).vec_mult(&(direct_color*SUN_MULTIPLIER + indirect_color));
                    return (total_color, true) 
                }

                if SUN_LIGHT_METHOD == 2{
                    let indirect_color = lighting::get_indirect_lighting(
                        ray_pos,
                        u_vec,
                        &objects,
                        refractive_index,
                        bounce_depth + 1u8,
                        Vec3{x: 1.0, y: 1.0, z: 1.0},  //WRONG TODO
                        1.0f32,
                        4i8,
                        point.refractive_index,
                        background_color_1,
                        background_color_2,
                        fog_color,
                    );
                    total_color = (fog_color/255.).vec_mult(&(indirect_color));  
                    return (total_color, true) 
                }

                if SUN_LIGHT_METHOD == 3{
                    let indirect_color = lighting::get_indirect_lighting_split(
                        ray_pos,
                        u_vec,
                        &objects,
                        refractive_index,
                        bounce_depth + 1u8,
                        Vec3{x: 1.0, y: 1.0, z: 1.0},  //WRONG TODO
                        1.0f32,
                        4i8,
                        point.refractive_index,
                        background_color_1,
                        background_color_2,
                        fog_color,
                        INITIAL_SPLITS,
                    );
                    
                    let direct_color = get_direct_lighting(
                        ray_pos,
                        u_vec,
                        &objects,
                        Vec3{x: 0.0, y: 0.0, z: 0.0},
                        true,
                    );
                    
                    total_color = (fog_color/255.).vec_mult(&(direct_color*3.0 + indirect_color));
                    return (total_color, true) 
                }
            }   
        }

        //take the step
        ray_pos = ray_pos + u_vec*step_length;
        fog_length -= step_length;

        //check if outside scene
        if Vec3::len(&ray_pos) > MAX_DISTANCE {
            hit = false;
            let h = (0.0f32).max(u_vec.dot(&Vec3{x:0.0, y:1.0, z:0.0}));

            let r = lerp(background_color_1.x, background_color_2.x, h);
            let g = lerp(background_color_1.y, background_color_2.y, h);
            let b = lerp(background_color_1.z, background_color_2.z, h);
            return (Vec3{x:r, y:g, z:b}, hit);
            //return (Vec3{x:0.0f32, y:0.0f32, z:0.0f32}, hit);
        }

        //check if hit
        if sdf_val.abs() < EPSILON1 {
            hit = true;

            if bounce_depth >= MAX_BOUNCE_DEPTH {
                return (MAX_BOUNCE_COLOR, true)
            }

            //find normal
            let distc = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z}).dist;        
            let distx = objects.get_nearest_point(Vec3{x:ray_pos.x+EPSILON2, y:ray_pos.y, z:ray_pos.z}).dist;                 
            let disty = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y+EPSILON2, z:ray_pos.z}).dist;                  
            let distz = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z+EPSILON2}).dist;
            let normal = Vec3::normalize(&Vec3{x:(distx-distc)/EPSILON2, y:(disty-distc)/EPSILON2, z:(distz-distc)/EPSILON2})*sdf_val.signum();
            
            if SUN_LIGHT_METHOD == 1 {
                if refractive_index == new_refractive_index {  //TODO, BAD WAY OF DOINGS THIS
                    new_refractive_index = START_REFRACTIVE_INDEX;
                }
                let indirect_color = lighting::get_indirect_lighting(
                    ray_pos,
                    u_vec,
                    &objects,
                    refractive_index,
                    bounce_depth + 1u8,
                    normal,
                    reflectance,
                    surface_model,
                    new_refractive_index,
                    background_color_1,
                    background_color_2,
                    fog_color,
                );

                let direct_color = get_direct_lighting(
                    ray_pos,
                    u_vec,
                    &objects,
                    normal,
                    false
                );

                if surface_model == 1 {  //TODO detta kanns fel...
                    total_color = (material_color/255.).vec_mult(&(direct_color + indirect_color));  
                }
                if surface_model == 2 {
                    total_color = indirect_color; 
                }
                if surface_model == 3 {
                    //refreaction and reflection
                    total_color = indirect_color;
                }
            }

            if SUN_LIGHT_METHOD == 2 { 
                if emission_rate < 0.001 {
                    let indirect_color = lighting::get_indirect_lighting(
                        ray_pos,
                        u_vec,
                        &objects,
                        refractive_index,
                        bounce_depth + 1u8,
                        normal,
                        reflectance,
                        surface_model,
                        point.refractive_index,
                        background_color_1,
                        background_color_2,
                        fog_color,
                    );

                    if surface_model == 1 {
                        total_color = (material_color/255.).vec_mult(&(indirect_color));  
                    }
                    if surface_model == 2 {
                        total_color = indirect_color; 
                    }
                    if surface_model == 3 {
                        //refraction and reflection
                        total_color = indirect_color;
                    }
                } else {
                    total_color = material_color*emission_rate
                }
            }

            if SUN_LIGHT_METHOD == 3 {
                //if refractive_index == new_refractive_index {  //TODO, BAD WAY OF DOINGS THIS
                //    new_refractive_index = START_REFRACTIVE_INDEX;
                //}

                let indirect_color = lighting::get_indirect_lighting_split(
                    ray_pos,
                    u_vec,
                    &objects,
                    refractive_index,
                    bounce_depth + 1u8,
                    normal,
                    reflectance,
                    surface_model,
                    new_refractive_index,
                    background_color_1,
                    background_color_2,
                    fog_color,
                    INITIAL_SPLITS
                );

                let direct_color = get_direct_lighting(
                    ray_pos,
                    u_vec,
                    &objects,
                    normal,
                    false
                );

                if surface_model == 1 {  //TODO detta kanns fel...
                    total_color = (material_color/255.).vec_mult(&(direct_color + indirect_color));  
                }
                if surface_model == 2 {
                    total_color = indirect_color; 
                }
                if surface_model == 3 {
                    //refreaction and reflection
                    total_color = indirect_color;
                }
            }
            return (total_color, hit);
        }
    }
    return (total_color, hit);
}


//TODO: rays start from an area, to create depth of field

const EPSILON1: f32 = 0.0000003;  //hit
const EPSILON2: f32 = 0.0001;  //normal
const EPSILON3: f32 = 0.0001;  //lightning
const MAX_BOUNCE_DEPTH: u8 = 5;
const MAX_BOUNCE_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:0.0};
const MAX_DISTANCE: f32 = 100.0;
const NUM_OF_SAMPLES: i32 = 5;  
const DEPTH_OF_FIELD: bool = false;
const DEPTH_OF_FIELD_CONST: f32 = 0.01;
const FOG: bool = false;
const FOG_LAMBDA: f32 = 1.0/15.0;
const INITIAL_SPLITS: i8 = 1;

//const NUM_BIN_WIDTH: usize = 1080;
const NUM_BIN_WIDTH: usize = 720/2;
const CANVAS_WIDTH: f32 = 1.123;

//const NUM_BIN_HEIGHT: usize = 1920;
const NUM_BIN_HEIGHT: usize = 1280/2;
const CANVAS_HEIGHT: f32 = 2.0;

const STEP_LENGTH_MULTIPLIER: f32 = 0.5;
const SUN_LIGHT_METHOD: i8 = 1;             //TODO method 3,4. one initial ray, which then splits into multiple. 
const SUN_MULTIPLIER: f32 = 3.;
const START_REFRACTIVE_INDEX: f32 = 1.0;

const MAX_STEPS: u32 = 1000;
const MAX_STEPS_COLOR: Vec3 = Vec3{x:0.0, y:0.0, z:0.0};
const TIME_APPROX: bool = true;
const TIME_APPROX_NUM: u32 = 100;
const WRITE_OPTIONS: bool = true;

fn main() {
        
    let progress = Arc::new(Mutex::new(0.0));
    let now = Instant::now();

    let bin_width = CANVAS_WIDTH / (NUM_BIN_WIDTH as f32);
    let bin_height = CANVAS_HEIGHT / (NUM_BIN_HEIGHT as f32);

    let mut bin_pos_array: Array3<f32> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //x,y,z
    //let image_array: Array3<u8> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //R,G,B

    let eye_pos = Vec3::zeros();
    let canvas_pos = Vec3 {
        x: 1.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };

    let fog_color: Vec3 = Vec3{x: 255.0, y: 255.0, z: 255.0};
    // let background_color_1: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    // let background_color_2: Vec3 = Vec3 {x: 0.0, y: 0.0, z: 0.0};
    
    let background_color_1: Vec3 = Vec3{x: 10.0f32, y: 10.0f32, z:155.0f32};
    let background_color_2: Vec3 = Vec3{x: 132.0f32, y: 206.0f32, z:235.0f32};
    
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
            (_, _) = ray(
                eye_pos,
                u_vector,
                &objects,
                0u8,
                START_REFRACTIVE_INDEX,
                background_color_1,
                background_color_2,
                fog_color,
                true,
            );
        }
        println!("Time left: {:?}",(now_approx.elapsed()/(TIME_APPROX_NUM*2))*NUM_BIN_WIDTH as u32*NUM_BIN_HEIGHT as u32*NUM_OF_SAMPLES as u32);
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
                    new_eye_pos = new_eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*DEPTH_OF_FIELD_CONST, z:(rand::random::<f32>()-0.5)*DEPTH_OF_FIELD_CONST}
                }
                vector = end_pos - new_eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};
                let u_vector = Vec3::normalize(&vector);
                (color, _) = ray(
                    new_eye_pos,
                    u_vector,
                    &objects,
                    0u8,
                    START_REFRACTIVE_INDEX,
                    background_color_1,
                    background_color_2,
                    fog_color,
                    true,
                );
                tcolor = tcolor + color
            }
            tcolor = tcolor/NUM_OF_SAMPLES as f32;
            tcolor
        }).collect();
        let mut progress = progress.lock().unwrap();
        *progress += 1.0;
        if i%5==0 {
            print!("\rProgress: {:.3}", *progress/NUM_BIN_WIDTH as f32); 
            std::io::stdout().flush();   
        }
        row
    }).collect();

    vec_to_image(image_array, "picture1.png");
        
    let elapsed = now.elapsed();
    println!("\nTotal time: {:?}", elapsed);
}
