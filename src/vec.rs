use std::{ops::{Add, Div, Mul, Sub, Neg}, f32::consts::PI};

use crate::lerp;

#[derive(Debug, Clone, Copy)]
pub struct RayData {
    pub ray_pos: Vec3,
    pub origin: Vec3,
    pub u_vec: Vec3,
    pub color: Vec3,
    pub pol_angle: f32,
    pub bounce_depth: u8,
    pub refractive_index: f32,
    pub fog_collision_check: bool,
    pub initial: bool,
}

impl RayData {
    pub fn basic() -> RayData {
        RayData {
            ray_pos: Vec3::zeros(),
            origin: Vec3::zeros(),
            u_vec: Vec3::zeros(),
            color: Vec3::zeros(),
            pol_angle: 0.0,
            bounce_depth: 0u8,
            refractive_index: 1.0f32,
            fog_collision_check: true,
            initial: true,  
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ObjectData {
    pub color: Vec3,
    pub reflectance: f32,
    pub surface_model: i8,
    pub emission_rate: f32,
    pub refractive_index: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub q: f32,
}

impl Vec4 {
    pub fn q_length2(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2) + (self.z).powi(2) + (self.q).powi(2)).sqrt()
    }

    pub fn vec_mult(&self, other: &Vec4) -> Vec4 {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            q: self.q * other.q,
        }
    }

    pub fn q_square(&self) -> Vec4 {
        Vec4 {
            x: self.x * self.x - self.y * self.y - self.z * self.z - self.q * self.q,
            y: 2.0*self.x*self.y,
            z: 2.0*self.x*self.z,
            q: 2.0*self.x*self.q
        }
    }

    pub fn q_cube(&self) -> Vec4 {
        
        let q2 = self.vec_mult(self);
        Vec4 {
            x: self.x  *(    q2.x - 3.0*q2.y - 3.0*q2.z - 3.0*q2.q),
            y: self.y*(3.0*q2.x -     q2.y -     q2.z -     q2.q),
            z: self.z*(3.0*q2.x -     q2.y -     q2.z -     q2.q),
            q: self.q*(3.0*q2.x -     q2.y -     q2.z -     q2.q)
        }
    }

    pub fn dot(&self, other: &Vec4) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.q * other.q
    }

    pub fn ones() -> Vec4 {
        Vec4 {
            x: 1.0f32,
            y: 1.0f32,
            z: 1.0f32,
            q: 1.0f32,
        }
    }
}

impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            q: self.q + other.q,
        }
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, other: f32) -> Vec4 {
        Vec4 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            q: self.q / other,
        }
    }
}

impl Add<f32> for Vec4 {
    type Output = Vec4;
    fn add(self, other: f32) -> Vec4 {
        Vec4 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
            q: self.q + other,
        }
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, other: f32) -> Vec4 {
        Vec4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            q: self.q * other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
//#[derive(PartialEq, Eq)]
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
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn vec_mult(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn vec_div(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
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

    pub fn exp(a: &Vec3) -> Vec3 {
        Vec3 {
            x: a.x.exp(),
            y: a.y.exp(),
            z: a.z.exp(),
        }
    }

    pub fn rot_vector180(n: &Vec3, v: &Vec3) -> Vec3 {
        n.scale(2.0 * (n.dot(v))) - *v
    }

    pub fn rot_vector(k: &Vec3, v: &Vec3, angle: f32) -> Vec3 { //angle cos can eb calced before
        return *v*angle.cos() + k.cross(v)*angle.sin() + *k*(k.dot(v))*(1.0-angle.cos())
    }

    pub fn hemisphere_bounce(normal: &Vec3) -> Vec3 {
        let mut ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        while ray_out.dot(&ray_out) > 1.0 || normal.dot(&ray_out) < 0.0 {
            ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        } 
        Vec3::normalize(&ray_out)
    }

    // pub fn cosine_weighted_hemisphere_bounce(normal: &Vec3) -> Vec3 {   chatgpt
    //     // Calculate spherical coordinates
    //     let theta = 2.0 * PI * rand::random::<f32>();
    //     let phi = (1.0 - rand::random::<f32>()).acos();

    //     // Convert spherical coordinates to Cartesian coordinates
    //     let x = phi.sin() * theta.cos();
    //     let y = phi.sin() * theta.sin();
    //     let z = phi.cos();

    //     // Create the local coordinate system aligned with the normal vector
    //     let w = normal;
    //     let up = if w.x.abs() > 0.1 {
    //         Vec3{x:0.0, y:1.0, z:0.0}
    //     } else {
    //         Vec3{x:1.0, y:0.0, z:0.0}
    //     };
    //     let u = up.cross(&w).normalize();
    //     let v = w.cross(&u);

    //     // Transform the sampled direction to world coordinates
    //     u * x + v * y + *w * z
    // }


    pub fn sphere_bounce() -> Vec3 {
        let mut ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        while ray_out.dot(&ray_out) > 1.0 {
            ray_out = Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*2.0-Vec3{x:1.0, y:1.0, z:1.0};
        } 
        Vec3::normalize(&ray_out)
    }

    // pub fn hemisphere_bounce(n: &Vec3, v: &Vec3) -> Vec3 {
    //     let v1 = Vec3::normalize(v);
    //     let cross = Vec3::normalize(&v1.cross(n));
    //     let angle1 = rand::random::<f32>().acos();
    //     let angle2 = rand::random::<f32>()*2.0*PI;
    //     let mut v_rot1 = Vec3::rot_vector(&cross, &n, angle1);
    //     v_rot1 = Vec3::normalize(&v_rot1);
    //     let mut ray_out = Vec3::rot_vector(&n, &v_rot1, angle2);
    //     ray_out = Vec3::normalize(&ray_out);
    //     //println!("{:?}", Vec3::len(&ray_out)); 
    //     return ray_out
    // }
    
    pub fn rotate_x(v: &Vec3, angle1: f32) -> Vec3 {
        Vec3 {
            x: v.x,
            y: v.y*angle1.cos() - v.z*angle1.sin(),
            z: v.y*angle1.sin() + v.z*angle1.cos(),
        }
    }

    pub fn rotate_y(v: &Vec3, angle1: f32) -> Vec3 {
        Vec3 {
            x: v.x*angle1.cos() + v.z*angle1.sin(),
            y: v.y,
            z: -v.x*angle1.sin() + v.z*angle1.cos(),
        }
    }
    
    pub fn rotate_z(v: &Vec3, angle1: f32) -> Vec3 {
        Vec3 {
            x: v.x*angle1.cos() + v.y*angle1.sin(),
            y: -v.x*angle1.sin() + v.y*angle1.cos(),
            z: v.z,
        }
    }

    pub fn rainbow_colors(value: f32) -> Vec3 {
        let value = value.min(1.0).max(0.0);

        if value < 0.25 {
            let h = value*4.0;
            return Vec3{x:h, y:0.0, z:0.0}
        } else if value < 0.5 {
            let h = (value-0.25)*4.0;
            return Vec3{x:1.0-h, y:h, z:0.0}
        } else if value < 0.75 {
            let h = (value-0.5)*4.0;
            return Vec3{x:0.0, y:1.0-h, z:h}
        } else {
            let h = (value-0.75)*4.0;
            return Vec3{x:0.0, y:0.0, z:1.0-h}
        }
    }

    pub fn rainbow_colors_inverse(color: Vec3) -> f32 {
        if color.x >= 0.0 && color.y == 0.0 {
            return color.x/4.0
        } else if color.x >= 0.0 && color.y >= 0.0 && color.z == 0.0 {
            return 0.25+color.y/4.0
        } else if color.x == 0.0 && color.y >= 0.0 && color.z >= 0.0 {
            return 0.5+color.z/4.0
        } else if color.x == 0.0 && color.y == 0.0 && color.z >= 0.0 {
            return 0.75+(1.0-color.z)/4.0
        } else {
            println!("{:?}","ERROR in rainbow_color_inverse");
            return 1.0
        }
    }

    pub fn clamp(vec: Vec3, min: f32, max:f32) -> Vec3 {
        Vec3 { 
            x: num::clamp(vec.x, min, max),
            y: num::clamp(vec.y, min, max),
            z: num::clamp(vec.z, min, max),
        }
    }

    // pub fn rainbow_colors(value: f32) -> Vec3 {
    //     let value = value.min(1.0).max(0.0);
    
    //     if value < 0.25 {
    //         let h = value * 4.0; // Scale value to [0, 1] range
    //         return Vec3 { x:h, y:0.0, z: 0.0 };
    //     } else if value < 0.5 {
    //         let h = (value - 0.25)* 2.0; // Scale value to [0, 1] range
    //         return Vec3 { x: 1.0 - h, y: h, z: 0.0 };
    //     } else if value < 0.75 {
    //         let h = (value - 0.5) * 2.0; // Scale value to [0, 1] range
    //         return Vec3 { x: 0.0, y: 1.0 - h, z: h };
    //     } else {
    //         let h = (value - 0.75) * 2.0; // Scale value to [0, 1] range
    //         return Vec3 { x: h, y: 0.0, z: 1.0 - h };
    //     }
    // }

    // def glossy(ray, normal, max_angle):
    // v_rot0 = multiply(rot_vector180(normal, ray), -1)
    // cross = normalize(cross_product(ray, normal))
    // v_rot1 = rot_vector(cross, v_rot0, random.uniform(0,max_angle))
    // ray_out = rot_vector(v_rot0, v_rot1, random.uniform(0, 2*np.pi))
    // return ray_out

}

// //TODO may be removed
// impl PartialEq for Vec3 {
//     fn eq(&self, other: &Self) -> bool {
//         return (self.x == other.x) && (self.y == other.y) && (self.z == other.z);
//     }
// }
// //TODO may be removed
// impl Eq for Vec3 {}



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


#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}
impl Vec2 {
    pub fn dot(&self, other: &Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }
    pub fn len(&self) -> f32 {
        ((self.x).powi(2) + (self.y).powi(2)).sqrt()
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, other: f32) -> Vec2 {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}