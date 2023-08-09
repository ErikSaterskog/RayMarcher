use image;
use image::GenericImageView;
use image::DynamicImage;

use crate::Op::{Union, SmoothUnion, Cut, Move, RotateY, RotateZ, Scale, Sphere, Cube, Plane, CappedCone, Ellipsoid, Line, InfRep, SinDistortHeight, MirrorZ, SwirlY};
use crate::lerp;
use crate::vec::Vec2;
use crate::vec::Vec3;


pub struct Surfacepoint {
    pub dist: f32,
    pub color: Vec3,
    pub reflectance: f32,
    pub surface_model: i8,
    pub emission_rate: f32,
    pub refractive_index: f32,
}

#[derive(Debug, Clone)]
pub enum Op{
    Union(Box<Op>, Box<Op>),
    SmoothUnion(Box<Op>, Box<Op>, f32),
    Cut(Box<Op>, Box<Op>),
    Sphere(Vec3, f32, i8, f32, f32),
    Cube(Vec3, Vec3, f32, i8, f32, f32),
    //Cylinder(Vec3, f32, f32),
    CappedCone(f32, f32, f32, Vec3, f32, i8, f32, f32),
    Ellipsoid(Vec3, Vec3, f32, i8, f32, f32),
    Line(Vec3, Vec3, f32, Vec3, f32, i8, f32, f32),
    Plane(f32, Vec3, f32, i8, f32, f32),
    Move(Box<Op>, Vec3),
    RotateY(Box<Op>, f32),
    RotateZ(Box<Op>, f32),
    Scale(Box<Op>, f32),
    InfRep(Box<Op>, Vec3),
    SinDistortHeight(Box<Op>, f32, f32),
    MirrorZ(Box<Op>),
    SwirlY(Box<Op>, f32),
    Texturize(Box<Op>, DynamicImage, Vec3, Vec3)    //TODO tes with "&static str"  / String
}

impl Op { 
    pub fn get_nearest_point(&self, ray_pos: Vec3) -> Surfacepoint { 
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
                let sm_a = point_a.surface_model;
                let sm_b = point_b.surface_model;
                let er_a = point_a.emission_rate;
                let er_b = point_b.emission_rate;
                let re_a = point_a.refractive_index;
                let re_b = point_b.refractive_index;

                let mut sm = 1;
                let h = (0.5+0.5*(db-da)/k).min(1.0).max(0.0);
                
                if rand::random::<f32>() > h {
                    sm = sm_b;
                } else {
                    sm = sm_a;
                }
                    
                return Surfacepoint{dist: (1.0-h)*db+h*da-k*h*(1.0-h), color: Vec3{x: lerp(ca.x,cb.x,h),y: lerp(ca.y,cb.y,h),z: lerp(ca.z,cb.z,h)}, reflectance: lerp(ra, rb, h), surface_model: sm, emission_rate: lerp(er_a, er_b, h), refractive_index: lerp(re_a, re_b, h)}
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
            Self::Sphere(color, reflectance, surface_model, emission_rate, refractive_index) => {
                return Surfacepoint{dist: Vec3::len(&ray_pos)-1.0, color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index}
            }
            Self::Cube(size, color, reflectance, surface_model, emission_rate, refractive_index) => {
                let q = Vec3::abs(&ray_pos) - *size;
                return Surfacepoint{dist: Vec3::len(&Vec3{x:q.x.max(0.0), y:q.y.max(0.0), z:q.z.max(0.0)}) + ((q.y.max(q.z)).max(q.x)).min(0.0), color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index};
            }
            // Self::Cylinder() => {
            //     //d = Vec3::len()
            //     //vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(h,r);
            //     //return min(max(d.x,d.y),0.0) + length(max(d,0.0));
            // }
            Self::CappedCone(h, r1, r2, color, reflectance, surface_model, emission_rate, refractive_index) => {
                let q = Vec2{x:Vec2::len(&Vec2{x: ray_pos.x, y: ray_pos.z}), y: ray_pos.y};
                let k1 = Vec2{x:*r2, y:*h};
                let k2 = Vec2{x:r2-r1, y:2.0*h};
                let mut ca = Vec2{x:0.0, y:0.0};
                if q.y < 0.0 {
                    ca = Vec2{x:q.x-q.x.min(*r1), y: q.y.abs()-h};
                } else {
                    ca = Vec2{x:q.x-q.x.min(*r2), y: q.y.abs()-h};
                }
                let cb = q - k1 + k2*((k1-q).dot(&k2) / k2.dot(&k2)).max(0.0).min(1.0);
                let mut s = 0.0;
                if cb.x < 0.0 && ca.y < 0.0 {
                    s = -1.0;
                } else {
                    s = 1.0;
                }
                return Surfacepoint{dist: s*(ca.dot(&ca).min(cb.dot(&cb))).sqrt(), color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index};
            }
            Self::Ellipsoid(r, color, reflectance, surface_model, emission_rate, refractive_index) => {
                let k0 = Vec3::len(&Vec3::vec_div(&ray_pos,&r));
                let k1 = Vec3::len(&Vec3::vec_div(&ray_pos,&Vec3::vec_mult(r,r)));
                return Surfacepoint{dist: k0*(k0-1.0)/k1, color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index};
            }
            Self::Line(a, b, r, color, reflectance, surface_model, emission_rate, refractive_index) => {
              let pa = ray_pos - *a;
              let ba = *b - *a;
              let h = (Vec3::dot(&pa,&ba) / Vec3::dot(&ba,&ba)).max(0.0).min(1.0);
              return Surfacepoint{dist: Vec3::len(&(pa - ba*h)) - r, color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index};
            }
            Self::Plane(h, color, reflectance, surface_model, emission_rate, refractive_index) => {
                return Surfacepoint{dist: h-ray_pos.y, color: *color, reflectance: *reflectance, surface_model: *surface_model, emission_rate: *emission_rate, refractive_index: *refractive_index};
            }
            Self::Move(a,vec) => {
                return a.get_nearest_point(ray_pos - *vec)
            }
            Self::RotateY(a, angle1) => {
                return a.get_nearest_point(Vec3::rotate_y(&ray_pos, *angle1))
            }
            Self::RotateZ(a, angle1) => {
                return a.get_nearest_point(Vec3::rotate_z(&ray_pos, *angle1))
            }
            Self::Scale(a,scale) => {
                let mut point_a = a.get_nearest_point(ray_pos/ *scale);
                point_a.dist *= *scale;
                return point_a
            }
            Self::InfRep(a, vec) => {
                let q = Vec3::modulo(&(ray_pos + *vec*0.5), vec) - *vec*0.5;
                return a.get_nearest_point(q)
            }
            Self::SinDistortHeight(a, amp_y, freq_y) => {
                let mut point_a = a.get_nearest_point(ray_pos);
                let displacement = (ray_pos.x*freq_y).sin()*(ray_pos.z*freq_y).sin();
                point_a.dist += displacement*amp_y;
                return point_a
            }
            Self::MirrorZ(a) => {
                return a.get_nearest_point(Vec3{x: ray_pos.x, y: ray_pos.y, z: (ray_pos.z).abs()});
            }
            Self::SwirlY(a, k) => {
                let c = (k*ray_pos.y).cos();
                let s = (k*ray_pos.y).sin();
                let q = Vec3{x: c*ray_pos.x+s*ray_pos.z, y: ray_pos.y , z: -s*ray_pos.x+c*ray_pos.z};
                return a.get_nearest_point(q)
            }
            Self::Texturize(a, tex, v1, v2) => {
                let point = a.get_nearest_point(ray_pos);
                let d = point.dist;
                let r = point.reflectance;
                let sm = point.surface_model;
                let er = point.emission_rate;
                let ri = point.refractive_index;

                let ray_pos_abs = Vec3::abs(&ray_pos);
                let dim = tex.dimensions();

                let tex_coord_1 = Vec3::dot(&ray_pos_abs, v1) as u32 % dim.0;
                let tex_coord_2 = Vec3::dot(&ray_pos_abs, v2) as u32 % dim.1;

                let pixel = tex.get_pixel(tex_coord_1, tex_coord_2);
                let c = Vec3{x:pixel[0] as f32, y:pixel[1] as f32, z:pixel[2] as f32};
                return Surfacepoint{dist: d, color: c, reflectance: r, surface_model: sm, emission_rate: er, refractive_index: ri}
            }
        }
    }
}