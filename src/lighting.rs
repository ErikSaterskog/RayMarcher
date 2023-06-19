use crate::vec::Vec3;
use crate::Op;
use crate::ray;
use core::f32::consts::PI;
use crate::EPSILON;


pub fn get_indirect_lighting(
    start_pos: Vec3,    //struct som beskriver ray:ens state
    u_vec: Vec3,
    objects: &Op,
    current_refractive_index: f32,
    bounce_depth: u8,
    normal: Vec3,
    reflectance: f32,
    surface_model: i8,
    new_refractive_index: f32,
) -> Vec3 {

    //mixed diffuse and mirror
    //u_vector_rot = getRay(&normal, &u_vec);

    let mut u_vector_rot = Vec3{x:0.0, y:0.0, z:0.0};
    let mut p = 1.0;
    let mut brdf = 1.0;
    let mut cos_theta = 1.0;
    let mut indirect_incoming = Vec3{x:0.0, y:0.0, z:0.0};

    if surface_model == 1 {
        //Ray bouncing  Lambertian
        u_vector_rot = Vec3::hemisphere_bounce(&normal, &u_vec);
        //probability of new ray
        p = 1.0 / (2.0*PI);
        brdf = reflectance / PI;
        cos_theta = u_vector_rot.dot(&normal);

        (indirect_incoming,_) = ray(
            start_pos + normal*EPSILON*10.,
            Vec3::normalize(&u_vector_rot),
            &objects,
            bounce_depth,
            current_refractive_index,
        ); 
    }
    
    if surface_model == 2 {
        //Ray bouncing mirrror
        u_vector_rot = -Vec3::rot_vector180(&normal, &u_vec);
        p = 1.0;
        brdf = reflectance;
        cos_theta =  1.0;

        (indirect_incoming,_) = ray(
            start_pos + normal*EPSILON*10.,
            Vec3::normalize(&u_vector_rot),
            &objects,
            bounce_depth,
            current_refractive_index,
        ); 
    }

    if surface_model == 3 {
        //Refraction/Reflection
        let normal_dot_u=-Vec3::dot(&normal, &u_vec);
        let theta1 = (normal_dot_u/(Vec3::len(&u_vec))).acos();
        let num = theta1.sin()*current_refractive_index/new_refractive_index;     //Todo: Get surroundings refractive_index
        if num > 1.0 || (rand::random::<f32>()>0.7 && current_refractive_index < new_refractive_index) {
            //Total reflection
            u_vector_rot = -Vec3::rot_vector180(&normal, &u_vec);
        } else {
            let theta2 = num.asin();
            let tangent = Vec3::normalize(&Vec3::cross(&normal, &u_vec));
            u_vector_rot = -Vec3::rot_vector(&tangent, &normal, -theta2);
        }
        p = 1.0;
        brdf = reflectance;
        cos_theta =  1.0;

        (indirect_incoming,_) = ray(
            start_pos - normal*EPSILON*10.,
            u_vector_rot,
            &objects,
            bounce_depth,
            new_refractive_index,
        );
    }

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
        1.0f32,
    );

    if light_ray_hit == true {   //TODO can göras bättre......
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