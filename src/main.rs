use image::RgbImage;
use ndarray::{Array3};
use std::time::Instant;
use std::ops::{Add, Div, Mul, Sub, Neg};
use std::cmp;
use crate::Op::{Union, Move, Scale, Sphere};


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

    pub fn rot_vector(n: &Vec3, v: &Vec3) -> Vec3 {
               n.scale(2.0 * (n.dot(v))) - *v
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

// #[derive(Debug, Clone, Copy)]
// struct Sphere {
//     pos: Vec3,
//     radius: f32,
//     color: Vec3,
//     light_prop: Vec3,
//     alpha: f32,
// }

// impl Sphere {
//     fn sdf(&self, ray_pos: Vec3) -> f32 {
//         Vec3::len(&(ray_pos - self.pos)) - self.radius
//     }
// }

// #[derive(Debug, Clone, Copy)]
// struct SkySphere {
//     pos: Vec3,
//     radius: f32,
//     color: Vec3,
//     light_prop: Vec3,
//     alpha: f32,
// }

// impl SkySphere {
//     fn sdf(&self, ray_pos: Vec3) -> f32 {
//         self.radius - Vec3::len(&(ray_pos - self.pos))
//     }
// }

// #[derive(Debug, Clone, Copy)]
// struct Torus {
//     pos: Vec3,
//     ra: f32,
//     rb: f32,
//     color: Vec3,
//     light_prop: Vec3,
//     alpha: f32,
// }

// impl Torus {
//     fn sdf(&self, ray_pos: Vec3) -> f32 {
//         let h = ((ray_pos.x-self.pos.x).powi(2)+(ray_pos.z-self.pos.z).powi(2)).sqrt();
//         return ((h-self.ra).powi(2)+(ray_pos.y-self.pos.y).powi(2)).sqrt()-self.rb
//     }
// }

// #[derive(Debug, Clone, Copy)]
// enum Shape {
//     Sphere(Sphere),
//     Torus(Torus),
//     SkySphere(SkySphere),
// }

fn lerp(a: f32, b: f32, h: f32) -> f32 {
    return a*h+b*(1.0f32-h)
} 


#[derive(Debug, Clone)]
enum Op{
    Union(Box<Op>, Box<Op>),
    Sphere(),
    Move(Box<Op>, Vec3),
    Scale(Box<Op>, f32),
}

impl Op { 
    fn sdf(&self, ray_pos: Vec3) -> f32 { 
        match &self { 
            Self::Union(a,b) => a.sdf(ray_pos).min(b.sdf(ray_pos)),
            Self::Sphere() => Vec3::len(&ray_pos)-1.,
            Self::Move(a,vec) => a.sdf(ray_pos - *vec),
            Self::Scale(a,scale) => a.sdf(ray_pos/ *scale),
        }
    }
}



fn array_to_image(arr: Array3<u8>) -> RgbImage {
    assert!(arr.is_standard_layout());

    let (height, width, _) = arr.dim();
    let raw = arr.into_raw_vec();

    RgbImage::from_raw(width as u32, height as u32, raw)
        .expect("container should have the right size for the image dimensions")
}

fn ray(
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    bounce_depth: u8,
) -> Vec3 {

    const EPSILON: f32 = 0.0001;
    const MAX_BOUNCE_DEPTH: u8 = 5;
    const MAX_DISTANCE: f32 = 128.0;
    
    let background_color_1 = Vec3{x: 10.0f32, y: 10.0f32, z:155.0f32};
    let background_color_2 = Vec3{x: 132.0f32, y: 206.0f32, z:235.0f32};

    let mut ray_pos = start_pos.clone();
    let mut impact_normal = Vec3::zeros();
    let mut color = Vec3::zeros();
    let mut intensity = 1.0f32;
    let mut sdf_val: f32 = 128.0; //background distance
    let mut hit: bool = false;
    let mut closest_object_color = Vec3::zeros();
    let mut closest_object_light_prop = Vec3::zeros();
    let mut closest_object_alpha = 0.0f32;


    while hit == false {
        
        //find the step length
        let sdf_val = objects.sdf(ray_pos);
        let color = Vec3{x:200.0f32, y:200.0f32, z:200.0f32};

        //take the step
        ray_pos = ray_pos + u_vec*sdf_val;
        
        //check if outside scene
        if Vec3::len(&ray_pos) > MAX_DISTANCE {
            //let r = lerp(background_color_1.x, background_color_2.x, ray_pos.y/MAX_DISTANCE);
            //let g = lerp(background_color_1.y, background_color_2.y, ray_pos.y/MAX_DISTANCE);
            //let b = lerp(background_color_1.z, background_color_2.z, ray_pos.y/MAX_DISTANCE);
            return Vec3{x:0.0f32, y:0.0f32, z:0.0f32};
        }

        //check if hit
        if sdf_val < EPSILON {
            hit = true;

            //closest object found
            if bounce_depth < MAX_BOUNCE_DEPTH {
                //find normal
                
                let distc = objects.sdf(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z});        
                let distx = objects.sdf(Vec3{x:ray_pos.x+EPSILON, y:ray_pos.y, z:ray_pos.z});                 
                let disty = objects.sdf(Vec3{x:ray_pos.x, y:ray_pos.y+EPSILON, z:ray_pos.z});                  
                let distz = objects.sdf(Vec3{x:ray_pos.x, y:ray_pos.y, z:ray_pos.z+EPSILON});
                let normal = Vec3::normalize(&Vec3{x:(distx-distc)/EPSILON, y:(disty-distc)/EPSILON, z:(distz-distc)/EPSILON});
                

                //Ray bouncing
                // let u_vector_rot = -Vec3::hemisphere_bounce(&impact_normal, &u_vector);
                // let (bounce_color, _, _) = ray(
                //     position_of_hit,
                //     u_vector_rot,
                //     &objects
                //     bounce_depth + 1u8,
                // );

                

                //Light calcs
                // let (_, _light_hit, light_closest_hit_distance) = ray(
                //     light_source.pos,
                //     position_of_hit,
                //     &spheres,
                //     &triangles,
                //     &light_sources,
                //     MAX_BOUNCE_DEPTH,
                // );
                // if light_closest_hit_distance < (Vec3::dist(&position_of_hit, &light_source.pos) - 0.01f32) {
                //     //Shadow
                //     intensity = closest_object_k_a * light_source.i_a;
                // } else {
                    //PHONG
                let light_u_vector = Vec3::normalize(&(Vec3{x:0.0, y:-2.0, z:-2.0} - ray_pos)); //direction unit vector from point on surface towards light
                let light_u_vector_rot = Vec3::rot_vector(&normal, &light_u_vector); //direction which a perfectly reflected ray of light has
                let a = 0.5f32;
                let d = 0.5f32* 0.0f32.max(Vec3::dot(&light_u_vector,&normal));
                let s = 0.5f32* 0.0f32.max(Vec3::dot(&light_u_vector_rot,&-u_vec)).powf(20.0f32)* 0.5f32;
                let intensity = a + d + s;

                return color * intensity;
            }
        }
    }
    return color * intensity
}



fn main() {
    let now = Instant::now();

    const NUM_BIN_WIDTH: usize = 1280;
    const CANVAS_WIDTH: f32 = 1.0;
    let bin_width = CANVAS_WIDTH / (NUM_BIN_WIDTH as f32);

    const NUM_BIN_HEIGHT: usize = 1280;
    const CANVAS_HEIGHT: f32 = 1.0;
    let bin_height = CANVAS_HEIGHT / (NUM_BIN_HEIGHT as f32);

    let mut bin_pos_array: Array3<f32> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //x,y,z
    let mut image_array: Array3<u8> = Array3::zeros((NUM_BIN_WIDTH, NUM_BIN_HEIGHT, 3)); //R,G,B

    let eye_pos = Vec3::zeros();
    let canvas_pos = Vec3 {
        x: 1.0f32,
        y: 0.0f32,
        z: 0.0f32,
    };

    let snow_man = Union(
        Box::new(Sphere()),
        Box::new(Move(Box::new(Scale(Box::new(Sphere{}), 0.5)), Vec3{x:0., y:-1.2, z:0.0})));

    let objects = Union(
        Box::new(snow_man.clone()), //Mom snowman
        Box::new(Union(
            Box::new(Move(Box::new(snow_man.clone()), Vec3{x:0., y:0., z:2.})), //Dad snowman
            Box::new(Move(Box::new(Scale(Box::new(snow_man.clone()), 0.5)), Vec3{x:0., y:0., z:-1.5})) //Baby snowman
        ))
    );

    let objects = Box::new(Move(Box::new(objects), Vec3{x:5., y:0., z:0.}));

    //loop to find bin positions
    for ((i, j, c), v) in bin_pos_array.indexed_iter_mut() {
        *v = match c {
            0 => canvas_pos.x,                                                       //x
            1 => canvas_pos.y - CANVAS_WIDTH / 2.0 + (i as f32 + 0.5) * bin_width,   //y
            2 => canvas_pos.z - CANVAS_HEIGHT / 2.0 + (j as f32 + 0.5) * bin_height, //z
            _ => unreachable!(),
        };
    }

    //loop to shoot rays
    for i in 0..NUM_BIN_WIDTH {
        for j in 0..NUM_BIN_HEIGHT {
            let x = bin_pos_array[[i, j, 0]]; //TODO must be possible to do in a better way.....
            let y = bin_pos_array[[i, j, 1]];
            let z = bin_pos_array[[i, j, 2]];
            let end_pos = Vec3{x:x, y:y, z:z};

            let vector = end_pos - eye_pos;
            let u_vector = Vec3::normalize(&vector);


            let color = ray(
                eye_pos,
                u_vector,
                &objects,
                0u8,
            );

            image_array[[i, j, 0]] = 255.0f32.min(color.x) as u8;
            image_array[[i, j, 1]] = 255.0f32.min(color.y) as u8;
            image_array[[i, j, 2]] = 255.0f32.min(color.z) as u8;
        }
        if i % 100 == 0 {
           println!("{}", (i as f32) / (NUM_BIN_WIDTH as f32))
        }
    }
    
    array_to_image(image_array).save("picture.png");

    let elapsed = now.elapsed();
    println!("Total time: {:?}", elapsed);
}
