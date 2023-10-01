use image;
use image::GenericImageView;
use image::DynamicImage;

use crate::Op::{Union, SmoothUnion, Cut, Intersection, Move, RotateX, RotateY, RotateZ, Scale, Round, Sphere, Cube, Torus, Plane, CappedCone, Ellipsoid, Ellipsoid2, Line, InfRep, SinDistortHeight, MirrorZ, SwirlY};

use crate::lerp;
use crate::vec::ObjectData;
use crate::vec::Vec2;
use crate::vec::Vec3;
use crate::vec::Vec4;

use std::cmp::min;

// pub struct Surfacepoint {
//     pub dist: f32,
//     pub color: Vec3,
//     pub reflectance: f32,
//     pub surface_model: i8,
//     pub emission_rate: f32,
//     pub refractive_index: f32,
// }

pub struct Surfacepoint {
    pub dist: f32,
    pub attributes: ObjectData,
}

#[derive(Debug, Clone)]
pub enum Op{
    Union(Box<Op>, Box<Op>),
    SmoothUnion(Box<Op>, Box<Op>, f32),
    Cut(Box<Op>, Box<Op>),
    Intersection(Box<Op>, Box<Op>),
    Sphere(ObjectData),
    Cube(Vec3, ObjectData),
    Torus(Vec2, ObjectData),
    //Cylinder(Vec3, f32, f32),
    CappedCone(f32, f32, f32, ObjectData),
    Ellipsoid(Vec3, ObjectData),
    Ellipsoid2(Vec3, ObjectData),
    Line(Vec3, Vec3, f32, ObjectData),
    Plane(f32, ObjectData),
    Move(Box<Op>, Vec3),
    RotateX(Box<Op>, f32),
    RotateY(Box<Op>, f32),
    RotateZ(Box<Op>, f32),
    Scale(Box<Op>, f32),
    InfRep(Box<Op>, Vec3),
    SinDistortHeight(Box<Op>, f32, f32),
    MirrorZ(Box<Op>),
    SwirlY(Box<Op>, f32),
    Round(Box<Op>, f32),
    Texturize(Box<Op>, DynamicImage, Vec3, Vec3),    //TODO tes with "&static str"  / String
    Frac(ObjectData),
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
                let ca = point_a.attributes.color;
                let cb = point_b.attributes.color;
                let ra = point_a.attributes.reflectance;
                let rb = point_b.attributes.reflectance;
                let sm_a = point_a.attributes.surface_model;
                let sm_b = point_b.attributes.surface_model;
                let er_a = point_a.attributes.emission_rate;
                let er_b = point_b.attributes.emission_rate;
                let re_a = point_a.attributes.refractive_index;
                let re_b = point_b.attributes.refractive_index;

                let mut sm = 1;
                let h = (0.5+0.5*(db-da)/k).min(1.0).max(0.0);
                
                if rand::random::<f32>() > h {
                    sm = sm_b;
                } else {
                    sm = sm_a;
                }

                let smooth_union_attributes = ObjectData{
                    color: Vec3{x: lerp(ca.x,cb.x,h),y: lerp(ca.y,cb.y,h),z: lerp(ca.z,cb.z,h)},
                    reflectance: lerp(ra, rb, h),
                    surface_model: sm,
                    emission_rate: lerp(er_a, er_b, h),
                    refractive_index: lerp(re_a, re_b, h),
                };
                    
                return Surfacepoint{dist: (1.0-h)*db+h*da-k*h*(1.0-h), attributes: smooth_union_attributes}
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
            Self::Intersection(a, b) => {
                let point_a = a.get_nearest_point(ray_pos);
                let point_b = b.get_nearest_point(ray_pos);

                if point_a.dist > point_b.dist {
                    return point_a
                } else {
                    return point_b
                }
            }

            Self::Sphere(sphere_attributes) => {
                return Surfacepoint{dist: Vec3::len(&ray_pos)-1.0, attributes: *sphere_attributes}
            }

            Self::Cube(size, cube_attributes) => {
                let q = Vec3::abs(&ray_pos) - *size;
                return Surfacepoint{dist: Vec3::len(&Vec3{x:q.x.max(0.0), y:q.y.max(0.0), z:q.z.max(0.0)}) + ((q.y.max(q.z)).max(q.x)).min(0.0), attributes: *cube_attributes};
            }

            Self::Torus(t, torus_attributes) => {
                let mut q = Vec2{x: Vec2{x:ray_pos.x, y:ray_pos.z}.len()-t.x,y: ray_pos.y};
                return Surfacepoint{dist: q.len()-t.y, attributes: *torus_attributes};
            }
            // Self::Cylinder() => {
            //     //d = Vec3::len()
            //     //vec2 d = abs(vec2(length(p.xz),p.y)) - vec2(h,r);
            //     //return min(max(d.x,d.y),0.0) + length(max(d,0.0));
            // }
            Self::CappedCone(h, r1, r2, cc_attributes) => {
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
                return Surfacepoint{dist: s*(ca.dot(&ca).min(cb.dot(&cb))).sqrt(), attributes: *cc_attributes};
            }
            Self::Ellipsoid(r, ellipsoid_attributes) => {
                let k0 = Vec3::len(&Vec3::vec_div(&ray_pos,&r));
                let k1 = Vec3::len(&Vec3::vec_div(&ray_pos,&Vec3::vec_mult(r,r)));
                return Surfacepoint{dist: k0*(k0-1.0)/k1, attributes: *ellipsoid_attributes};
            }
            Self::Ellipsoid2(r, ellipsoid2_attributes) => {
                return Surfacepoint{dist: (Vec3::len(&Vec3::vec_div(&ray_pos,&r))-1.0) * (((r.x).min(r.y)).min(r.z)), attributes: *ellipsoid2_attributes};
            }
            Self::Line(a, b, r, line_attributes) => {
              let pa = ray_pos - *a;
              let ba = *b - *a;
              let h = (Vec3::dot(&pa,&ba) / Vec3::dot(&ba,&ba)).max(0.0).min(1.0);
              return Surfacepoint{dist: Vec3::len(&(pa - ba*h)) - r, attributes: *line_attributes};
            }
            Self::Plane(h, plane_attributes) => {
                return Surfacepoint{dist: h-ray_pos.y, attributes: *plane_attributes};
            }
            Self::Move(a,vec) => {
                return a.get_nearest_point(ray_pos - *vec)
            }
            Self::RotateX(a, angle1) => {
                return a.get_nearest_point(Vec3::rotate_x(&ray_pos, *angle1))
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
                let r = point.attributes.reflectance;
                let sm = point.attributes.surface_model;
                let er = point.attributes.emission_rate;
                let ri = point.attributes.refractive_index;

                let ray_pos_abs = Vec3::abs(&ray_pos);
                let dim = tex.dimensions();

                let tex_coord_1 = Vec3::dot(&ray_pos_abs, v1) as u32 % dim.0;
                let tex_coord_2 = Vec3::dot(&ray_pos_abs, v2) as u32 % dim.1;

                let pixel = tex.get_pixel(tex_coord_1, tex_coord_2);
                let c = Vec3{x:pixel[0] as f32 / 255.0, y:pixel[1] as f32 / 255.0, z:pixel[2] as f32 / 255.0};

                let tex_attributes = ObjectData{
                    color: c,
                    reflectance: r,
                    surface_model: sm,
                    emission_rate: er,
                    refractive_index:ri
                };
                return Surfacepoint{dist: d, attributes: tex_attributes}
            }

            Self::Round(a, b) => {

                let mut point_a = a.get_nearest_point(ray_pos);
                point_a.dist -= *b;
                return point_a
                
                //return Surfacepoint{dist: d, color: c, reflectance: r, surface_model: sm, emission_rate: er, refractive_index: ri}
            }

            Self::Frac (frac_attributes) => {
                //https://www.shadertoy.com/view/3tsyzl
                let mut z = Vec4{x: ray_pos.x, y: ray_pos.y, z: ray_pos.z, q:0.0};
                let mut dz2 = 1.0;
                let mut m2  = 0.0;
                let mut n = 0.0;
                //#ifdef TRAPS
                let mut o  = 10000000000.0;
                //#endif
                let k_num_ite = 200;
                let k_c = Vec4{x:-2.0, y:6.0, z:15.0, q:-6.0}/22.0;  // /22.0
                

                for _ in 0..k_num_ite {
                    
                    // z' = 3z² -> |z'|² = 9|z²|²
                    dz2 *= 9.0*(z.q_square()).q_length2();

                    // z = z³ + c		
                    z = z.q_cube() + k_c;
                    
                    // stop under divergence
                    m2 = z.q_length2();
                    
                    // orbit trapping : https://iquilezles.org/articles/orbittraps3d
                    //#ifdef TRAPS
                    // let temp2 = (Vec2{x: z.x, y:z.z}-Vec2{x:0.45, y: 0.55}).len()-0.1;
                    // if o > temp2 {
                    //     o = temp2;
                    // }
                    //#endif
                    
                    // exit condition
                    if m2 > 256.0 {
                        break;			
                    }	 
                    n += 1.0;
                }
            
                // sdf(z) = log|z|·|z|/|dz| : https://iquilezles.org/articles/distancefractals
                let d = 0.25*m2.ln()*((m2/dz2).sqrt());
                
                //#ifdef TRAPS
                // if o < d {
                //    d = o;
                // }
                //#endif
                
                //#ifdef CUT
                // if ray_pos.y > d {
                //     d = ray_pos.y;
                // }
                //#endif
                  
                return Surfacepoint{dist: d, attributes: *frac_attributes}      
            }
        }
    }
}