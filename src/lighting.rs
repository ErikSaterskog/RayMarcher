use crate::vec::Vec3;
use crate::Op;
use crate::ray;
use core::f32::consts::PI;
use crate::EPSILON;


pub fn get_indirect_lighting(
    start_pos: Vec3,
    u_vec: Vec3,
    objects: &Op,
    bounce_depth: u8,
    normal: Vec3,
    reflectance: f32,
    surface_model: i8,
) -> Vec3 {

    //mixed diffuse and mirror
    //u_vector_rot = getRay(&normal, &u_vec);

    let mut u_vector_rot = Vec3{x:0.0, y:0.0, z:0.0};
    let mut p = 1.0;
    let mut brdf = 1.0;
    let mut cos_theta = 1.0;

    if surface_model == 1 {
        //Ray bouncing  Lambertian
        u_vector_rot = Vec3::hemisphere_bounce(&normal, &u_vec);
        //probability of new ray
        p = 1.0 / (2.0*PI);
        brdf = reflectance / PI;
        cos_theta = u_vector_rot.dot(&normal);
    }
    
    if surface_model == 2 {
        //Ray bouncing mirrror
        u_vector_rot = -Vec3::rot_vector180(&normal, &u_vec);
        p = 1.0;
        brdf = reflectance;
        cos_theta =  1.0;
    }

    let (indirect_incoming,_) = ray(
        start_pos + normal*EPSILON*10.,
        Vec3::normalize(&u_vector_rot),
        &objects,
        bounce_depth + 1u8,
    );
    
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
    let sun_radius = 10.0;
    return Vec3{x:-100.0, y:-100.0, z:-100.0} + Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*sun_radius-Vec3{x:1.0, y:1.0, z:1.0}*sun_radius/2.0;
}

pub fn get_direct_lighting(
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