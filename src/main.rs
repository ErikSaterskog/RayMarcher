//extern crate image;
//use image::DynamicImage::ImageRgb8;
use image::RgbImage;
use ndarray::{Array3};
use std::time::Instant;
use std::ops::{Add, Div, Mul, Sub, Neg};
use std::cmp;
use crate::Op::{Union, SmoothUnion, Cut, Move, Scale, Sphere, Cube, InfRep};
use core::f32::consts::PI;
use rand::prelude::*;
use rayon::prelude::*;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vec3 {
    pub fn zeros() -> Vec3 {
        Vec3 {
            x: 0.0f32,
            y: 0.0f32,
            z: 0.0f32,
        }
    }

    pub fn ones() -> Vec3 {
        Vec3 {
            x: 1.0f32,
            y: 1.0f32,
            z: 1.0f32,
        }
    }

    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.z * other.y - self.y * other.x,
        }
    }

    pub fn vec_mult(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn dist(&self, other: &Vec3) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2) + (self.z - other.z).powi(2))
            .sqrt()
    }

    pub fn len(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let length = Vec3::len(self);
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn scale(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    pub fn modulo(k: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            x: k.x % v.x,
            y: k.y % v.y,
            z: k.z % v.z,
        }
    }

    pub fn abs(v: &Vec3) -> Vec3 {
        Vec3 {
            x: v.x.abs(),
            y: v.y.abs(),
            z: v.z.abs(),
        }

    }

    pub fn rot_vector180(n: &Vec3, v: &Vec3) -> Vec3 {
        n.scale(2.0 * (n.dot(v))) - *v
    }

    pub fn rot_vector(k: &Vec3, v: &Vec3, angle: f32) -> Vec3 {
        return *v*angle.cos()+(k.cross(v))*angle.sin()+*k*(k.dot(v))*(1.0-angle.cos())
    }


    pub fn hemisphere_bounce(n: &Vec3, v:&Vec3) -> Vec3 {
        let cross = Vec3::normalize(&v.cross(n));
        let angle1 = rand::random::<f32>()*PI/2.0;
        let angle2 = rand::random::<f32>()*2.0*PI;
        let v_rot1 = Vec3::rot_vector(&cross, &n, angle1);
        let ray_out = Vec3::rot_vector(&n, &v_rot1, angle2);
        return ray_out
    }

    // def glossy(ray, normal, max_angle):
    // v_rot0 = multiply(rot_vector180(normal, ray), -1)
    // cross = normalize(cross_product(ray, normal))
    // v_rot1 = rot_vector(cross, v_rot0, random.uniform(0,max_angle))
    // ray_out = rot_vector(v_rot0, v_rot1, random.uniform(0, 2*np.pi))
    // return ray_out

}


impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

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

fn get_indirect_lightning(
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    bounce_depth: u8,
    normal: Vec3,
    reflectance: f32,
) -> Vec3 {

    //mixed diffuse and mirror
    //u_vector_rot = getRay(&normal, &u_vec);
    //Ray bouncing  Lambertian
    let u_vector_rot = Vec3::hemisphere_bounce(&normal, &u_vec);
    //Ray bouncing mirrror
    //let u_vector_rot = -Vec3::rot_vector180(&normal, &u_vec);

    let (indirect_incoming,_) = ray(
        start_pos + normal*EPSILON*10.,
        u_vector_rot,
        &objects,
        bounce_depth + 1u8,
    );

    //probability of new ray
    let p = 1.0 / (2.0*PI);

    let cos_theta = u_vector_rot.dot(&normal);
    let brdf = reflectance / PI;

    let indirect_color = indirect_incoming * brdf * cos_theta / p;

    return indirect_color
}

fn get_shadow(
    start_pos: Vec3,
    light_u_vec: Vec3,
    objects: &Op,
) -> f32 {
    let (light_ray_color, light_ray_hit) = ray(    
        start_pos,
        light_u_vec,
        objects,
        100u8,
    );

    if light_ray_hit == true {
        return 0.0f32
    } else {
        return 1.0f32
    }
}

fn get_sun_point() -> Vec3 {
    return Vec3{x:-100.0, y:-100.0, z:-100.0}
}

fn get_direct_lightning(
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    normal: Vec3,
) -> Vec3 {

    let sun_pos = get_sun_point();
    let light_u_vec = Vec3::normalize(&(sun_pos - start_pos));
    let normal_dot_light = (0.0f32).max(light_u_vec.dot(&normal));
    let shadow = get_shadow(start_pos+normal*EPSILON*10., light_u_vec, objects);
    let direct_color = Vec3{x:253.0, y: 251.0, z: 211.0} * normal_dot_light * shadow;
    return direct_color
}

fn ray(
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    bounce_depth: u8,
) -> (Vec3, bool) {
    
    let background_color_1 = Vec3{x: 10.0f32, y: 10.0f32, z:155.0f32};
    let background_color_2 = Vec3{x: 132.0f32, y: 206.0f32, z:235.0f32};

    let mut ray_pos = start_pos.clone();
    let mut total_color = Vec3::zeros();
    let mut hit: bool = false;

    let mut i=1.0f32;

    while hit == false {
        i += 1.0f32;
        if i>10000.0 {
            println!("Warning! #1");
            hit = true;
            return (Vec3{x:0.0, y:0.0, z:0.0}, hit)
        }

        //find the step length
        let point = objects.get_nearest_point(ray_pos);
        let sdf_val = point.dist;
        let material_color = point.color;
        let reflectance = point.reflectance;

        //take the step
        ray_pos = ray_pos + u_vec*sdf_val;
        
        //check if outside scene
        if Vec3::len(&ray_pos) > MAX_DISTANCE {
            hit = false;
            let h = (0.0f32).max(u_vec.dot(&Vec3{x:0.0, y:1.0, z:0.0}));

            let r = lerp(background_color_1.x, background_color_2.x, h);
            let g = lerp(background_color_1.y, background_color_2.y, h);
            let b = lerp(background_color_1.z, background_color_2.z, h);
            return (Vec3{x:r, y:g, z:b}, hit);
            //return Vec3{x:0.0f32, y:0.0f32, z:0.0f32};
        }

        //check if hit
        if sdf_val < EPSILON {
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
            let normal = Vec3::normalize(&Vec3{x:(distx-distc)/EPSILON, y:(disty-distc)/EPSILON, z:(distz-distc)/EPSILON});
            
            
            let indirect_color = get_indirect_lightning(
                ray_pos,
                u_vec,
                &objects,
                bounce_depth + 1u8,
                normal,
                reflectance,
            );

            let direct_color = get_direct_lightning(
                ray_pos,
                u_vec,
                &objects,
                normal,
            );

            
            //let direct_color = Vec3{x: 0.0, y:0.0, z:0.0};
            //total_color = material_color.vec_mult(&(direct_color + indirect_color)); 
            total_color = direct_color + indirect_color; 
            return (total_color, hit);
        }
    }
    return (total_color, hit);
}

struct Surfacepoint {
    dist: f32,
    color: Vec3,
    reflectance: f32,
}

#[derive(Debug, Clone)]
enum Op{
    Union(Box<Op>, Box<Op>),
    SmoothUnion(Box<Op>, Box<Op>, f32),
    Cut(Box<Op>, Box<Op>),
    Sphere(Vec3, f32),
    Cube(Vec3, Vec3, f32),
    Move(Box<Op>, Vec3),
    Scale(Box<Op>, f32),
    InfRep(Box<Op>, Vec3)
}

impl Op { 
    fn get_nearest_point(&self, ray_pos: Vec3) -> Surfacepoint { 
        match &self { 
            Self::Union(a, b) => {
                let point_a = a.get_nearest_point(ray_pos);
                let point_b = b.get_nearest_point(ray_pos);

                if point_a.dist < point_b.dist {
                    return point_a
                } else {
                    return point_b
                }
            }
            Self::SmoothUnion(a, b, k) => {
                let point_a = a.get_nearest_point(ray_pos);
                let point_b = b.get_nearest_point(ray_pos);
                let da = point_a.dist;
                let db = point_b.dist;
                let ca = point_a.color;
                let cb = point_b.color;
                let ra = point_a.reflectance;
                let rb = point_b.reflectance;

                let h = (0.5+0.5*(db-da)/k).min(1.0).max(0.0);
                return Surfacepoint{dist: (1.0-h)*db+h*da-k*h*(1.0-h), color: Vec3{x: lerp(ca.x,cb.x,h),y: lerp(ca.y,cb.y,h),z: lerp(ca.z,cb.z,h)}, reflectance: lerp(ra, rb, h)}
            }
            Self::Cut(b, a) => {
                let mut point_a = a.get_nearest_point(ray_pos);
                let point_b = b.get_nearest_point(ray_pos);
                point_a.dist *= -1.0;
                if point_a.dist > point_b.dist { 
                    return point_a
                } else {
                    return point_b
                }
            }
            Self::Sphere(color, reflectance) => {
                return Surfacepoint{dist: Vec3::len(&ray_pos)-1.0, color: *color, reflectance: *reflectance}
            }
            Self::Cube(size, color, reflectance) => {
                let q = Vec3::abs(&ray_pos) - *size;
                return Surfacepoint{dist: Vec3::len(&Vec3{x:q.x.max(0.0), y:q.y.max(0.0), z:q.z.max(0.0)}) + ((q.y.max(q.z)).max(q.x)).min(0.0), color: *color, reflectance: *reflectance};
            }
            Self::Move(a,vec) => {
                return a.get_nearest_point(ray_pos - *vec)
            }
            Self::Scale(a,scale) => {
                let mut point_a = a.get_nearest_point(ray_pos/ *scale);
                point_a.dist *= *scale;
                return point_a
            }
            Self::InfRep(a, vec) => {
                let q = Vec3::modulo(&(ray_pos + *vec*0.5), vec) - *vec*0.5;
                //let q = Vec3::modulo(&ray_pos, &vec);
                return a.get_nearest_point(q)
            }
        }
    }
}

const EPSILON: f32 = 0.0001;
const MAX_BOUNCE_DEPTH: u8 = 3;
const MAX_DISTANCE: f32 = 63.0;

fn main() {
    let now = Instant::now();

    const NUM_OF_SAMPLES: i32 = 1;

    const NUM_BIN_WIDTH: usize = 640*2;
    const CANVAS_WIDTH: f32 = 2.0;
    let bin_width = CANVAS_WIDTH / (NUM_BIN_WIDTH as f32);

    const NUM_BIN_HEIGHT: usize = 640*2;
    const CANVAS_HEIGHT: f32 = 2.0;
    let bin_height = CANVAS_HEIGHT / (NUM_BIN_HEIGHT as f32);

    let mut bin_pos_array: Array3<f32> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //x,y,z
    let mut image_array: Array3<u8> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //R,G,B

    let eye_pos = Vec3::zeros();
    let canvas_pos = Vec3 {
        x: 1.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };

    //let mut objects = Box::new(Sphere(Vec3{x:255.0, y:255.0, z:255.0},1.0));

    if false { //room scene
        let mut room = Cut(
            Box::new(Cube(Vec3{x:10.0, y:1.0, z:1.0}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0)),
            Box::new(Move(Box::new(Cube(Vec3{x:4.0, y:0.9, z:0.9}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0)), Vec3{x:-1.1, y:0.0, z:0.0}))
        );
        
        room = Cut(
            Box::new(room.clone()),
            Box::new(Move(Box::new(Cube(Vec3{x:0.3, y:0.3, z:0.3}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0)), Vec3{x:0.0, y:-0.3, z:-0.9}))
        );

        room = Union(
            Box::new(room.clone()),
            Box::new(Move(Box::new(Scale(Box::new(Sphere(Vec3{x:255.0, y:255.0, z:255.0}, 1.0)),0.2)), Vec3{x:0.0, y:0.3, z:0.0})),
        );

        let objects = Box::new(Move(Box::new(room), Vec3{x:3.0, y:0.0, z:0.1}));
    }

    if false { //Infballs scene
        let mut objects = Box::new(Sphere(Vec3{x:255.0, y:0.0, z:0.0}, 1.0));
        objects = Box::new(InfRep(Box::new(*objects), Vec3{x:5.0, y:5.0, z:5.0}));
        objects = Box::new(Move(Box::new(*objects), Vec3{x:5., y:-102.5, z:-102.5}));
    }

    //if false { //Menger Sponge
    let mut cross_0 = Union(
        Box::new(Union(
            Box::new(Cube(Vec3{x:10000.0,y:1.,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0)),
            Box::new(Cube(Vec3{x:1.,y:10000.0,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0))
        )),
        Box::new(Cube(Vec3{x:1.,y:1.,z:10000.0}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0))
    );
    let layer_0 = Box::new(Move(Box::new(Cube(Vec3{x:1.,y:1.,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0)),Vec3{x:0.0,y:2.0,z:2.0}));
    let cross_1 = Box::new(Scale(Box::new(cross_0.clone()),0.333));
    let layer_1 = Cut(
        Box::new(*layer_0.clone()),
        //Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0/(3.*i as f32), y:2.0/(3.*i as f32), z:2.0/(3.*i as f32)}))
        Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:-2.0, y:2.0, z:2.0}))
    );
    let cross_2 = Box::new(Scale(Box::new(*cross_1.clone()),0.333));
    let mut layer_2 = Cut(
        Box::new(layer_1.clone()),
        Box::new(InfRep(Box::new(*cross_2.clone()), Vec3{x:-0.666, y:0.666, z:0.666}))
    );
    let cross_3 = Box::new(Scale(Box::new(*cross_2.clone()),0.333));
    let mut layer_3 = Cut(
        Box::new(layer_2.clone()),
        Box::new(InfRep(Box::new(*cross_3.clone()), Vec3{x:-0.222, y:0.222, z:0.222}))
    );
    let cross_4 = Box::new(Scale(Box::new(*cross_3.clone()),0.333));
    let mut layer_4 = Cut(
        Box::new(layer_3.clone()),
        Box::new(InfRep(Box::new(*cross_4.clone()), Vec3{x:-0.074, y:0.074, z:0.074}))
    );
    let cross_5 = Box::new(Scale(Box::new(*cross_4.clone()),0.333));
    let mut layer_5 = Cut(
        Box::new(layer_4.clone()),
        Box::new(InfRep(Box::new(*cross_5.clone()), Vec3{x:-0.074/3., y:0.074/3., z:0.074/3.}))
    );
    let objects = Box::new(Move(Box::new(layer_5), Vec3{x:1.0, y:-2.3, z:-2.0}));
    //}


    //loop to find bin positions
    for ((i, j, c), v) in bin_pos_array.indexed_iter_mut() {
        *v = match c {
            0 => canvas_pos.x,                                                       //x
            1 => canvas_pos.y - CANVAS_WIDTH / 2.0 + (i as f32 + 0.5) * bin_width,   //y
            2 => canvas_pos.z - CANVAS_HEIGHT / 2.0 + (j as f32 + 0.5) * bin_height, //z
            _ => unreachable!(),
        };
    }

    //loop to shoot rays parallell
    let image_array: Vec<Vec<Vec3>> = (0..NUM_BIN_WIDTH).into_par_iter().map(|i| {
        let row: Vec<Vec3> = (0..NUM_BIN_HEIGHT).into_par_iter().map(|j| {

            let x = bin_pos_array[[i, j, 0]];
            let y = bin_pos_array[[i, j, 1]];
            let z = bin_pos_array[[i, j, 2]];
            let end_pos = Vec3{x:x, y:y, z:z};
            let vector = end_pos - eye_pos;
            let u_vector = Vec3::normalize(&vector);

            let mut color = Vec3{x:0.0, y:0.0, z:0.0};
            let mut tcolor = Vec3{x:0.0, y:0.0, z:0.0};
            
            for _k in 0..NUM_OF_SAMPLES {
                (color, _) = ray(
                    eye_pos,
                    u_vector,
                    &objects,
                    0u8,
                );
                tcolor = tcolor + color
            }
            tcolor = tcolor/NUM_OF_SAMPLES as f32;
            tcolor
        }).collect();
        if i % 10 == 0 {
           println!("{}", (i as f32) / (NUM_BIN_WIDTH as f32))
        }
        row
    }).collect();

    vec_to_image(image_array, "picture2.png");
        
    let elapsed = now.elapsed();
    println!("Total time: {:?}", elapsed);
}
