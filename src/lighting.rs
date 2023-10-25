use crate::ATMOSPHERE_HEIGHT;
use crate::DENSITY_FALLOFF;

use crate::SCATTER_COEFF;
use crate::Settings;
//use crate::SUN_POSITION;
use crate::vec::RayData;
use crate::vec::Vec3;
use crate::Op;
use crate::ray;
use core::f32::consts::PI;
use std::f32::consts::E;
use std::ops::Add;



pub fn get_indirect_lighting(
    mut ray_data: RayData,    
    objects: &Op,
    normal: Vec3,
    reflectance: f32,
    surface_model: i8,
    new_refractive_index: f32,
    settings: Settings,
) -> Vec3 {

    //mixed diffuse and mirror
    //u_vector_rot = getRay(&normal, &u_vec);

    let mut u_vector_rot = Vec3{x:0.0, y:0.0, z:0.0};
    let mut p = 1.0;
    let mut brdf = 1.0;
    let mut cos_theta = 1.0;
    let mut indirect_incoming = Vec3::zeros();

    if surface_model == 1 {
        //Ray bouncing  Lambertian
        //u_vector_rot = Vec3::hemisphere_bounce(&normal);                                //Original
        //u_vector_rot = Vec3::cosine_weighted_hemisphere_bounce(&normal);                //chatgpts cosine weighted
        u_vector_rot = Vec3::normalize(&(Vec3::sphere_bounce() + normal));          //Cosine weighted
        
        //probability of new ray
        //p = 1.0 / (2.0*PI);                   //Original
        p = 1.0 / (PI);                         //cosine weighted
        //p = u_vector_rot.dot(&normal)/PI;     //???

        brdf = reflectance / PI;
        
        //cos_theta = u_vector_rot.dot(&normal);   //Original
        cos_theta = 1.0;                           //cosine weighted

        // if cos_theta.is_nan() {
        //     println!("u_vector_rot: {:?}",u_vector_rot);
        //     println!("normal: {:?}",normal);
        // }

        ray_data.ray_pos = ray_data.ray_pos + normal*settings.eps3*10.;
        ray_data.origin = ray_data.ray_pos;
        ray_data.u_vec = Vec3::normalize(&u_vector_rot);
        ray_data.fog_collision_check = true;
        ray_data.initial = false;

        (indirect_incoming,_,_) = ray(
            &objects,
            ray_data,
            settings,
        ); 
        //indirect_incoming = indirect_incoming.vec_mult(&ray_data.color);
    }
    
    if surface_model == 2 {
        //Ray bouncing mirrror
        u_vector_rot = -Vec3::rot_vector180(&normal, &ray_data.u_vec);
        p = 1.0;
        brdf = reflectance;
        cos_theta =  1.0;

        ray_data.ray_pos = ray_data.ray_pos + normal*settings.eps3*10.;
        ray_data.origin = ray_data.ray_pos;
        ray_data.u_vec = Vec3::normalize(&u_vector_rot);
        ray_data.fog_collision_check = true;
        ray_data.initial = true;
        
        (indirect_incoming,_,_) = ray(
            &objects,
            ray_data,
            settings,
        ); 
    }

    if surface_model == 3 {

        //new_refractive_index depends on the color of the ray
        //if RGB_RAYS {
        //let color_val = Vec3::rainbow_colors_inverse(ray_data.color);
        //let new_refractive_index = color_val+1.0;
            //println!("{:?}",new_refractive_index)
        //}

        //Refraction/Reflection
        let normal_dot_u=-Vec3::dot(&normal, &ray_data.u_vec);
        let theta1 = (normal_dot_u/(Vec3::len(&ray_data.u_vec))).acos();
        let sin_theta2 = theta1.sin()*ray_data.refractive_index/new_refractive_index;
        // println!("{:?}", sin_theta2);
        
        //Schlicks approximation
        let r_0 = ((ray_data.refractive_index-new_refractive_index)/(ray_data.refractive_index+new_refractive_index)).powi(2);
        let r = r_0 + (1.0 - r_0)*(1.0-theta1.cos()).powi(5);
        //println!("{:?}",r);
        
        if sin_theta2 > 1.0 || rand::random::<f32>() < r {// || (rand::random::<f32>() < (1.0 - normal_dot_u) && ray_data.refractive_index < new_refractive_index) {
            //Total reflection
            u_vector_rot = -Vec3::rot_vector180(&normal, &ray_data.u_vec);
            ray_data.ray_pos = ray_data.ray_pos + normal*settings.eps3*10.;
        } else {
            let theta2 = sin_theta2.asin();
            let tangent = Vec3::normalize(&Vec3::cross(&normal, &ray_data.u_vec));
            u_vector_rot = -Vec3::rot_vector(&tangent, &normal, -theta2);
            ray_data.refractive_index = new_refractive_index;
            ray_data.ray_pos = ray_data.ray_pos - normal*settings.eps3*10.;
        }
        p = 1.0;
        brdf = reflectance;
        cos_theta =  1.0;

        //ray_data.ray_pos = ray_data.ray_pos - normal*EPSILON3*10.;
        
        ray_data.origin = ray_data.ray_pos;
        ray_data.u_vec = u_vector_rot;
        ray_data.fog_collision_check = true;
        ray_data.initial = false;
        
        (indirect_incoming,_,_) = ray(
            &objects,
            ray_data,
            settings,
        ); 
    }

    if surface_model == 4 {

        //Scatter hit in fog
        u_vector_rot = Vec3::sphere_bounce();
        //probability of new ray
        //p = 1.0 / (2.0*PI);
        //brdf = reflectance / PI;
        //cos_theta = u_vector_rot.dot(&normal);

        //ray_data.ray_pos = ray_data.ray_pos;
        ray_data.origin = ray_data.ray_pos;
        ray_data.u_vec = Vec3::normalize(&u_vector_rot);
        ray_data.fog_collision_check = true;
        ray_data.initial = false;
        
        (indirect_incoming,_,_) = ray(
            &objects,
            ray_data,
            settings,
        ); 
    }

    let indirect_color = indirect_incoming * brdf * cos_theta / p;

    return indirect_color
}


fn get_shadow(
    start_pos: Vec3,
    light_u_vec: Vec3,
    objects: &Op,
    settings: Settings,
) -> f32 {

    let mut ray_data = RayData::basic();
    ray_data.ray_pos = start_pos;
    ray_data.origin = ray_data.ray_pos;
    ray_data.u_vec = light_u_vec;
    ray_data.bounce_depth = 100u8;
    ray_data.fog_collision_check = false;
    ray_data.initial = false;
    let (_, _, distance) = ray(
        &objects,
        ray_data,
        settings,
    ); 

    if distance < Vec3::len(&(ray_data.origin - settings.sun_position)) {
        return 0.0f32
    } else {
        return 1.0f32
    }
}

fn get_sun_point(settings: Settings) -> Vec3 {
    return settings.sun_position + Vec3{x:rand::random::<f32>(), y:rand::random::<f32>(), z:rand::random::<f32>()}*settings.sun_radius-Vec3{x:1.0, y:1.0, z:1.0}*settings.sun_radius/2.0;
}

pub fn get_direct_lighting(
    start_pos: Vec3,
    _u_vec: Vec3,
    objects: &Op,
    normal: Vec3,
    fog: bool,
    settings: Settings,
) -> Vec3 {

    if fog {
        let sun_pos = get_sun_point(settings);
        let light_u_vec = Vec3::normalize(&(sun_pos - start_pos));
        let shadow = get_shadow(start_pos, light_u_vec, objects, settings);
        let direct_color = settings.sun_color * shadow;        
        return direct_color
    } else {
        let sun_pos = get_sun_point(settings);
        let light_u_vec = Vec3::normalize(&(sun_pos - start_pos));
        let normal_dot_light = (0.0f32).max(light_u_vec.dot(&normal));
        let shadow = get_shadow(start_pos+normal*settings.eps3*10., light_u_vec, objects, settings);
        let direct_color = settings.sun_color * normal_dot_light * shadow;
        return direct_color
    }
}

pub fn get_rayleigh_color(
    start_pos: Vec3,
    end_pos: Vec3,
    sun_position: Vec3
    ) -> Vec3 {
    let ray_length = Vec3::len(&(start_pos-end_pos));
    let ray_dir = Vec3::normalize(&(end_pos - start_pos));

    let num_scattering_points = 100;
    let step_length = ray_length / (num_scattering_points - 1) as f32;
    let mut sun_scatter_direction = Vec3::normalize(&(sun_position - start_pos));
    let mut scatter_point = start_pos;
    let mut density = 0.0;
    let mut transmittance = Vec3::zeros();
    let mut in_scattered_light = Vec3::zeros();
    let mut sun_ray_length = 0.0;
    let mut sun_ray_optical_depth = 0.0;
    let mut view_ray_optical_depth = 0.0;

    for i in 0..num_scattering_points {
        
        density = atm_density_at_point(scatter_point);
        sun_scatter_direction = Vec3::normalize(&(sun_position - scatter_point));
        sun_ray_length = Vec3::len(&(sun_position-scatter_point));

        sun_ray_optical_depth = optical_depth(start_pos, sun_scatter_direction, sun_ray_length);
        view_ray_optical_depth = optical_depth(start_pos, ray_dir, step_length * i as f32);

        transmittance = Vec3::exp(&(SCATTER_COEFF * -(sun_ray_optical_depth + view_ray_optical_depth)));
        //println!("{:?}",sun_ray_optical_depth);
        in_scattered_light = in_scattered_light + (transmittance.vec_mult(&SCATTER_COEFF) * step_length * density);
        scatter_point = scatter_point + ray_dir * step_length;
    }
   
    return in_scattered_light

}

pub fn atm_density_at_point(position: Vec3) -> f32 {
    let height01 = -position.y / ATMOSPHERE_HEIGHT;
    let local_density = (-height01 * DENSITY_FALLOFF).exp() * (1.0 - height01);
    //println!("{:?}", local_density.max(0.0));
    return local_density.max(0.0)
}

pub fn optical_depth(start_pos: Vec3, ray_dir: Vec3, ray_length: f32) -> f32 {
    let num_optical_depth_points = 10;
    let mut density_sample_point = start_pos;
    let step_size = ray_length / (num_optical_depth_points - 1) as f32;
    let mut optical_depth = 0.0;
    let mut density = 0.0;
    for i in 0..num_optical_depth_points {
        density = atm_density_at_point(density_sample_point);
        optical_depth += density * step_size;
        density_sample_point = density_sample_point + ray_dir * step_size;
    }
    //println!("{:?}", optical_depth);
    return optical_depth
}