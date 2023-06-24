use std::sync::{Arc, Mutex};
use image::RgbImage;
use ndarray::{Array3};
use std::time::Instant;
use std::cmp;

use core::f32::consts::PI;
use rand::prelude::*;
use rayon::prelude::*;
use std::path::Path;

mod vec;
use crate::vec::Vec2;
use crate::vec::Vec3;

mod lighting;
use crate::lighting::get_direct_lighting;
use crate::lighting::get_indirect_lighting;

mod operations;
use crate::operations::Op;
use crate::Op::Move;
use crate::Op::Scale;
use crate::Op::SinDistortHeight;
use crate::Op::Sphere;
use crate::Op::Cube;
use crate::Op::CappedCone;
use crate::Op::Line;
use crate::Op::RotateY;

mod scenes;
use crate::scenes::scene;


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
) -> (Vec3, bool) {

    
    let background_color_1 = Vec3{x: 10.0f32, y: 10.0f32, z:155.0f32};
    let background_color_2 = Vec3{x: 132.0f32, y: 206.0f32, z:235.0f32};

    //let background_color_1 = Vec3{x: 0.0f32, y: 0.0f32, z:0.0f32};
    //let background_color_2 = Vec3{x: 0.0f32, y: 0.0f32, z:0.0f32};


    let mut ray_pos = start_pos.clone();
    let mut total_color = Vec3::zeros();
    let mut hit: bool = false;

    let mut i=1;

    while hit == false {
        i += 1;
        if i>1000 {
            //println!("Warning! #1");
            hit = true;
            return (Vec3{x:255.0, y:0.0, z:0.0}, hit)
        }

        //find the step length
        let point = objects.get_nearest_point(ray_pos);
        let sdf_val = point.dist;
        let material_color = point.color;
        let reflectance = point.reflectance;
        let surface_model = point.surface_model;
        let emission_rate = point.emission_rate;
        let mut new_refractive_index = point.refractive_index;

        //take the step   //TODO sdf abs new variable
        ray_pos = ray_pos + u_vec*sdf_val.abs()*STEP_LENGTH_MULTIPLIER;
        
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
        if sdf_val.abs() < EPSILON {
            hit = true;

            //Check if max bounces has been reached
            if bounce_depth >= MAX_BOUNCE_DEPTH {
                hit = true;
                return (Vec3{x:0.0, y:0.0, z:0.0}, hit)
            }
    
            //find normal
            let distc = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z}).dist;        
            let distx = objects.get_nearest_point(Vec3{x:ray_pos.x+EPSILON, y:ray_pos.y, z:ray_pos.z}).dist;                 
            let disty = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y+EPSILON, z:ray_pos.z}).dist;                  
            let distz = objects.get_nearest_point(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z+EPSILON}).dist;
            let normal = Vec3::normalize(&Vec3{x:(distx-distc)/EPSILON, y:(disty-distc)/EPSILON, z:(distz-distc)/EPSILON})*sdf_val.signum();
            
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
                    new_refractive_index
                );

                let direct_color = get_direct_lighting(
                    ray_pos,
                    u_vec,
                    &objects,
                    normal,
                );

                if surface_model == 1 {
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

            if SUN_LIGHT_METHOD == 2 {  //TODO vid sista studsen så gör man en ray mot ljuskällan
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
            return (total_color, hit);
        }
    }
    return (total_color, hit);
}


//TODO: rays start from an area, to create depth of field

const EPSILON: f32 = 0.0001;
const MAX_BOUNCE_DEPTH: u8 = 4;
const MAX_DISTANCE: f32 = 20.0;
const NUM_OF_SAMPLES: i32 = 20;  
const DEPTH_OF_FIELD: bool = false;
const DEPTH_OF_FIELD_CONST: f32 = 0.05;

// const NUM_BIN_WIDTH: usize = 1000;
// const CANVAS_WIDTH: f32 = 1.0;

// const NUM_BIN_HEIGHT: usize = 1000;
// const CANVAS_HEIGHT: f32 = 1.0;

const NUM_BIN_WIDTH: usize = 1080;
//const NUM_BIN_WIDTH: usize = 720;
const CANVAS_WIDTH: f32 = 1.123;

const NUM_BIN_HEIGHT: usize = 1920;
//const NUM_BIN_HEIGHT: usize = 1280;
const CANVAS_HEIGHT: f32 = 2.0;

const STEP_LENGTH_MULTIPLIER: f32 = 1.0;
const SUN_LIGHT_METHOD: i8 = 1;

const START_REFRACTIVE_INDEX: f32 = 1.0;


fn main() {
    let progress = Arc::new(Mutex::new(0.0));
    let now = Instant::now();

    
    let bin_width = CANVAS_WIDTH / (NUM_BIN_WIDTH as f32);
    let bin_height = CANVAS_HEIGHT / (NUM_BIN_HEIGHT as f32);

    let mut bin_pos_array: Array3<f32> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //x,y,z
    let image_array: Array3<u8> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //R,G,B

    let eye_pos = Vec3::zeros();
    let canvas_pos = Vec3 {
        x: 1.0f32,
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
                if DEPTH_OF_FIELD == true {
                    vector = end_pos - Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*DEPTH_OF_FIELD_CONST, z:(rand::random::<f32>()-0.5)*DEPTH_OF_FIELD_CONST} + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};
                } else {
                    vector = end_pos - eye_pos + Vec3{x:0.0, y:(rand::random::<f32>()-0.5)*bin_width, z:(rand::random::<f32>()-0.5)*bin_height};
                }
                let u_vector = Vec3::normalize(&vector);
                (color, _) = ray(
                    eye_pos,
                    u_vector,
                    &objects,
                    0u8,
                    START_REFRACTIVE_INDEX,
                );
                tcolor = tcolor + color
            }
            tcolor = tcolor/NUM_OF_SAMPLES as f32;
            tcolor
        }).collect();
        let mut progress = progress.lock().unwrap();
        *progress += 1.0;
        if i%20==0 {
            println!("{:?}", *progress/NUM_BIN_WIDTH as f32);    
        }
        row
    }).collect();

    vec_to_image(image_array, "picture1.png");
        
    let elapsed = now.elapsed();
    println!("Total time: {:?}", elapsed);
}
