use image;
use image::GenericImageView;
use image::DynamicImage;

use crate::Op::{Union, SmoothUnion, Cut, Intersection, Move, RotateX, RotateY, RotateZ, Scale, Round, Sphere, Cube, Torus, Plane, Prism, CappedCone, Ellipsoid, Ellipsoid2, Line, InfRep, SinDistortHeight, MirrorZ, SwirlY};

use crate::lerp;
use crate::vec::ObjectData;
use crate::vec::RayData;
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
    Prism(Vec2, ObjectData),
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
    Frac2(ObjectData),
    Frac3(ObjectData),
    Frac4(ObjectData, f32),
    Frac5(ObjectData),
    Frac6(ObjectData),
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
            Self::Prism(a, prism_attributes) => {
                let q = Vec3::abs(&ray_pos);
                return Surfacepoint{dist: (q.z-a.y).max((q.x*0.866025+ray_pos.y*0.5).max(-ray_pos.y)-a.x*0.5), attributes: *prism_attributes};
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

            Self::Frac2 (frac2_attributes) => {
                let mut z = ray_pos;
                let mut dr = 1.0;
                let mut r = 0.0;
                let bailout = 10.0;
                let power = 6.0;
                let iterations = 100;
                for i in 0..iterations {
                    r = Vec3::len(&z);
                    if r > bailout {break};
                    
                    // convert to polar coordinates
                    let mut theta = (z.z/r).acos();
                    let mut phi = (z.y).atan2(z.x);
                    dr = r.powf(power-1.0)*power*dr + 1.0;
                    
                    // scale and rotate the point
                    let zr = r.powf(power);
                    let theta = theta*power;
                    let phi = phi*power;
                    
                    // convert back to cartesian coordinates
                    z = Vec3{x:theta.sin()*phi.cos(), y:phi.sin()*theta.sin(), z:theta.cos()}*zr;
                    z = z + ray_pos;
                }
                //println!("{:?}",r);
                // let frac2_attributes = ObjectData{
                //     color: Vec3::rainbow_colors(r.log10() as f32 / 7.0 as f32),
                //     reflectance: 1.0,
                //     surface_model: 1,
                //     emission_rate: 0.0,
                //     refractive_index: 1.0,
                // };
                //let frac2_attributes.color = Vec3::rainbow_colors(r as f32 / bailout as f32);
                return Surfacepoint{dist: 0.5*r.log(10.0)*r/dr, attributes: *frac2_attributes};
            }
            Self::Frac3 (frac3_attributes) => {
                
                let iterations = 30;
                let mut x = ray_pos.x;
                let mut y = ray_pos.y;
                let mut z = ray_pos.z;
                let mut defactor = 1.0;

                for i in 0..iterations {
                    //inside iteration loop:
                
                    let fixed_radius = 1.0;
                    let f_r2 = fixed_radius * fixed_radius;
                    let min_radius = 0.5;
                    let m_r2 = min_radius * min_radius;

                    if x > 1.0 {
                        x = 2.0 - x;
                    } else if x < -1.0 {
                        x = -2.0 - x
                    };

                    if y > 1.0 {
                        y = 2.0 - y;
                    } else if y < -1.0 {
                        y = -2.0 - y};

                    if z > 1.0 {
                        z = 2.0 - z;
                    } else if z < -1.0 {
                        z = -2.0 - z
                    };

                    let r2 = x*x + y*y + z*z;

                    if r2 < m_r2 {
                    x = x * f_r2 / m_r2;
                    y = y * f_r2 / m_r2;
                    z = z * f_r2 / m_r2;
                    defactor = defactor * f_r2 / m_r2;
                    } else if r2 < f_r2 {
                    x = x * f_r2 / r2;
                    y = y * f_r2 / r2;
                    z = z * f_r2 / r2;
                    defactor *= f_r2 / r2;
                    }

                    //x = x * scale + cx;
                    //y = y * scale + cy;
                    //z = z * scale + cz;
                    //defactor *= scale;
                }
                let distance = (x*x+y*y+z*z).sqrt()/(defactor).abs();
                return Surfacepoint{dist: distance, attributes: *frac3_attributes};
            }
            // Self::Frac4 (frac4_attributes) => {
            //     //function mandelbox_distance_estimator(x, y, z, max_iter, scale, fold_scale, bailout):
            //     let mut real = ray_pos.x;
            //     let mut imag = ray_pos.y;
            //     let mut comp = ray_pos.z;
            //     let mut r2 = 0.0;
            //     let mut r = 0.0;
            //     let mut theta = 0.0;
            //     let mut phi = 0.0;

            //     let scale = 1.0;
            //     let max_iter = 100;
            //     let fold_scale = 1.5;
            //     let bailout =  10.0;
            //     let scale_sq = scale * scale;
                
                
            //     for _ in 0..max_iter {
            //         r2 = real * real + imag * imag + comp * comp;
            //         if r2 > bailout {
            //             let distance = 0.5 * (r2.sqrt()).ln() / r2.sqrt();
            //             println!("{:?}", distance);
            //             return Surfacepoint{dist: distance, attributes: *frac4_attributes};
            //         }
            //         // Scale and fold
            //         real = (real * fold_scale) - scale;
            //         imag = (imag * fold_scale) - scale;
            //         comp = (comp * fold_scale) - scale;

            //         // Scale and rotate
            //         r = (real * real + imag * imag + comp * comp).sqrt();
            //         theta = (imag).atan2(real);
            //         phi = (comp).atan2((real * real + imag * imag).sqrt());
                    
            //         real = r * (theta).sin() * scale_sq + ray_pos.x;
            //         imag = r * (theta).cos() * (phi).sin() * scale_sq + ray_pos.y;
            //         comp = r * (theta).cos() * (phi).cos() * scale_sq + ray_pos.z;
            //     }
            //     //let distance = 0.5 * (r2).ln() * r / (r2).sqrt();
            //     let distance = 0.0;
            //     return Surfacepoint{dist: distance, attributes: *frac4_attributes};
            // }
            Self::Frac4 (frac4_attributes, scale) => {

                //let scale: f32 = 2.0;    //2.0
                let max_iter = 30;    //todo, change number of iterations needed
                //let fold_scale = 1.5;
                //let bailout =  10.0;
                //let scale_sq = scale * scale;
                
                let offset = ray_pos;
                let mut z = ray_pos;
                let mut dr = 1.0;
                for _ in 0..max_iter {
                    (z,dr) = box_fold(z,dr);       // Reflect
                    (z,dr) = sphere_fold(z,dr);    // Sphere Inversion
                     
                    z = z**scale + offset;  // Scale & Translate
                    dr = dr*(scale.abs())+1.0;
                }
                let r = Vec3::len(&z);
                return Surfacepoint{dist: r/(dr.abs()), attributes: *frac4_attributes};
            }
            Self::Frac5(frac5_attributes) => {

                let d0=DE0(ray_pos, Vec3{x:0.0, y:0.0, z:0.0});   
                let d2=DE2(ray_pos);

                return Surfacepoint{dist: (d0).max(d2), attributes: *frac5_attributes};
            }
            Self::Frac6(frac6_attributes) => {
                let ones = Vec3::ones();
                let mut z = ray_pos;
                let scale = 2.0;
                let mut de = 1.0;
                let m_r2 = 0.5;
                let f_r2 = 1.0;
                let mbf = 0.5;

                for _ in 0..20 { 
                    
                    z = Vec3::abs(&(z + ones)) - Vec3::abs(&(z - ones)) - z;
                    let r2 = z.dot(&z);
                    
                    if r2 < m_r2 {
                        z = z * mbf;
                        de = de * mbf;
                    } else if r2 < f_r2 {
                        let tglad_factor2 = f_r2 / r2;
                        z = z * tglad_factor2;
                        de = de * tglad_factor2;
                    }
                    z = z * scale + ray_pos;
                    de = de * (scale).abs() + 1.;
                }
                //if fractal.mandelbox.mainRotationEnabled:
                //    z = fractal.mandelbox.mainRot.rotate_vector(z)

                
                return Surfacepoint{dist: de, attributes: *frac6_attributes}
            }
        }
    }
}

pub fn sphere_fold(mut z: Vec3, mut dz: f32)  -> (Vec3, f32) {
	let r2 = Vec3::dot(&z,&z);
    let m_r2 = 0.5;  //0.5
    let f_r2 = 1.0; //1.0
	if r2 < m_r2 { 
		// linear inner scaling
		let temp = f_r2/m_r2;
		z = z*temp;
		dz = dz*temp;
	} else if r2 < f_r2 { 
		// this is the actual sphere inversion
		let temp =f_r2/r2;
		z  = z*temp;
		dz = dz*temp;
	}
    return (z, dz)
}

pub fn sphere_fold_2(mut z: Vec4) -> Vec4 {
    let r2 = Vec3::dot(&Vec3{x:z.x, y:z.y, z:z.z},&Vec3{x:z.x, y:z.y, z:z.z});
    if r2 < 2.0 {
        z = z * (1.0/r2);
    } else {
        z = z * 0.5;
    }
    return z
}


pub fn box_fold(mut z: Vec3, dz: f32) -> (Vec3, f32) {
    let f_lim = 1.0;  //1.0
	z = Vec3::clamp(z, -f_lim, f_lim) * 2.0 - z;
    return (z, dz)
}

pub fn box_fold_2(mut z: Vec3) -> Vec3 {
    let f_lim = 1.0;  //1.0
	z = Vec3::clamp(z, -f_lim, f_lim) * 2.0 - z;
    return z
}

pub fn DE0(pos: Vec3, from: Vec3) -> f32 {
    let z = pos - from;
    let r = Vec3::dot(&z,&z)*Vec3::len(&z).powf(2.0);
    return (1.0-smoothstep(0.0,0.01,r))*0.01
}

pub fn DE2(pos: Vec3) -> f32 {

    let scale = -20.0*0.272321;
    let mut p = Vec4{x: pos.x, y: pos.y, z: pos.z, q:1.0};
    let p0 = p;  
    let c = Vec4{x: 0.0, y: 0.0, z: 0.0, q: 0.0}; // params = -0.5..0.5

    for _ in 0..10 { 
        let temp = box_fold_2(Vec3{x:p.x, y:p.y, z:p.z});
        p = Vec4{x:temp.x ,y:temp.y ,z:temp.z ,q:p.q};
        p = sphere_fold_2(p);
        p = p*scale+c;
    }

   return Vec3::len(&Vec3{x:p.x, y:p.y, z:p.z})/p.q;
}

pub fn smoothstep (edge0: f32, edge1: f32, mut x: f32) -> f32 {
    // Scale, and clamp x to 0..1 range
    x = ((x - edge0) / (edge1 - edge0)).clamp(0.0, 1.0);
    return x * x * (3.0 - 2.0 * x);
 }

// vec4 sphere(vec4 z)
// {
//    float r2 = dot(z.xyz,z.xyz);
//    if(r2<2.0)
//       z*=(1.0/r2);
//    else   z*=0.5;

//    return z;

// }
// vec3 box(vec3 z)
// {
//    return clamp(z, -1.0, 1.0) * 2.0 - z;
// }

// float DE0(vec3 pos)
// {
//    vec3 z=pos-from;
//    float r=dot(pos-from,pos-from)*pow(length(z),2.0);
//    return (1.0-smoothstep(0.0,0.01,r))*0.01;
// }

// float DE2(vec3 pos)
// {

//    vec4 scale = -20*0.272321;
//      vec4 p = vec4(pos,1.0), p0 = p;  
//    vec4 c=vec4(param[31].w,param[32].w,param[33].w,0.5)-0.5; // param = 0..1

//      for (float i=0;i<10; i++)
//    {
//       p.xyz=box(p.xyz);
//       p=sphere(p);
//       p=p*scale+c;
//      }

//    return length(p.xyz)/p.w;
// }

// float DE(vec3 pos)
// {

//    float d0=DE0(pos);   
//    float d2=DE2(pos);

//    return max(d0,d2);
// }