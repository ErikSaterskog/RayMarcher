
use std::f32::consts::PI;
use crate::{vec::Vec3};

pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
}

pub struct BezierPath {
    pub start_point: Vec3,
    pub control_point1: Vec3,
    pub control_point2: Vec3,
    pub end_point: Vec3,
}

pub fn evaluate_cubic_bezier(path: &BezierPath, t: f32) -> Vec3 {
    let u = 1.0 - t;
    let t2 = t * t;
    let u2 = u * u;
    let t3 = t2 * t;
    let u3 = u2 * u;

    let x = u3 * path.start_point.x + 3.0 * u2 * t * path.control_point1.x + 3.0 * u * t2 * path.control_point2.x + t3 * path.end_point.x;
    let y = u3 * path.start_point.y + 3.0 * u2 * t * path.control_point1.y + 3.0 * u * t2 * path.control_point2.y + t3 * path.end_point.y;
    let z = u3 * path.start_point.z + 3.0 * u2 * t * path.control_point1.z + 3.0 * u * t2 * path.control_point2.z + t3 * path.end_point.z;

    return Vec3{x:x, y:y, z:z}  
}

pub fn look_at_point(cam_pos: Vec3, point_pos: Vec3) -> (f32, f32) {
    let dist = point_pos - cam_pos;
    let len = Vec3::len(&dist);
    let phi = (dist.z / dist.x).atan(); //spin
    let theta = (dist.y / len).acos();  //up/down
    return (phi, theta)
}

pub fn get_camera(t: f32) -> Camera {

    // let bezier_path = BezierPath {
    //     start_point: Vec3 {x:-2.8, y:-4.0, z:5.7},
    //     control_point1: Vec3 {x:-1.5, y:-4.0, z:5.7},
    //     control_point2: Vec3 {x:-1.5, y:-4.15, z:7.0},
    //     end_point: Vec3 {x:-1.5, y:-4.15, z:7.0},
    // };

    // let bezier_path1 = BezierPath {
    //     start_point: Vec3 {x:-14.0, y:0.0, z:0.0},
    //     control_point1: Vec3 {x:-8.0, y:-3.88, z:-3.858},
    //     control_point2: Vec3 {x:-7.5, y:-3.88, z:-3.858},
    //     end_point: Vec3 {x:-7.0, y:-3.88, z:-3.858},
    // };

    // let bezier_path2 = BezierPath {
    //     start_point: Vec3 {x:-7.0, y:-3.88, z:-3.858},
    //     control_point1: Vec3 {x:-6.0, y:-3.88, z:-3.858},
    //     control_point2: Vec3 {x:-5.81, y:-3.88, z:-3.858},
    //     end_point: Vec3 {x:-5.62, y:-3.88, z:-3.858},
    // };

    // let bezier_path3 = BezierPath {
    //     start_point: Vec3 {x:-71.0, y:-51.0, z:51.0},
    //     control_point1: Vec3 {x:-71.0, y:-51.0, z:51.0}/1.3,
    //     control_point2: Vec3 {x:-71.0, y:-51.0, z:51.0}/1.3,
    //     end_point: Vec3 {x:-71.0, y:-51.0, z:51.0}/1.3,
    // };

    // let bezier_path4 = BezierPath {  //Frac4, scale 2
    //     start_point: Vec3 {x:-4.0, y:-0.0, z:0.0},
    //     control_point1: Vec3 {x:-2.0, y:-0.65, z:0.65},
    //     control_point2: Vec3 {x:-1.0, y:-0.55, z:0.595},
    //     end_point: Vec3 {x:-0.9, y:-0.55, z:0.595},
    // };

    // let bezier_path5 = BezierPath {  //Frac4, scale 2
    //     start_point: Vec3 {x:-0.9, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.89, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.88, y:-0.55, z:0.595},
    //     end_point: Vec3 {x:-0.87, y:-0.55, z:0.595},
    // };

    // let bezier_path6 = BezierPath {  //Frac4, scale 2
    //     start_point: Vec3 {x:-0.87, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.866, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.863, y:-0.55, z:0.595},
    //     end_point: Vec3 {x:-0.86, y:-0.55, z:0.595},
    // };

    // let bezier_path7 = BezierPath {  //path
    //     start_point: Vec3 {x:-0.86, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.8566, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.8533, y:-0.55, z:0.595},
    //     end_point: Vec3 {x:-0.85, y:-0.65, z:0.595},
    // };

    // let bezier_path_point = BezierPath {  //point
    //     start_point: Vec3 {x:-0.8, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.75, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.75, y:-0.6, z:0.6},
    //     end_point: Vec3 {x:-0.75, y:-1.0, z:0.65},
    // };

        // let bezier_path7 = BezierPath {  //path
    //     start_point: Vec3 {x:-0.86, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.8566, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.8533, y:-0.55, z:0.595},
    //     end_point: Vec3 {x:-0.85, y:-0.65, z:0.595},
    // };

    // let bezier_path_point = BezierPath {  //point
    //     start_point: Vec3 {x:-0.8, y:-0.55, z:0.595},
    //     control_point1: Vec3 {x:-0.75, y:-0.55, z:0.595},
    //     control_point2: Vec3 {x:-0.75, y:-0.6, z:0.6},
    //     end_point: Vec3 {x:-0.75, y:-1.0, z:0.65},
    // };

    // let bezier_path7 = BezierPath {  //path
    //     start_point: Vec3 {x:-0.85, y:-0.65, z:0.595},
    //     control_point1: Vec3 {x:-0.85, y:-0.8, z:0.595},
    //     control_point2: Vec3 {x:-0.85, y:-1.2, z:0.595},
    //     end_point: Vec3 {x:-0.85, y:-1.5, z:0.595},
    // };

    // let bezier_path_point = BezierPath {  //point
    //     start_point: Vec3 {x:-0.75, y:-1.0, z:0.65},
    //     control_point1: Vec3 {x:-0.75, y:-1.0, z:0.65},
    //     control_point2: Vec3 {x:-0.85, y:-1.0, z:0.6},
    //     end_point: Vec3 {x:-0.85, y:-1.1, z:0.7},
    // };

    // let bezier_path8 = BezierPath {  //path
    //     start_point: Vec3 {x:-0.85, y:-1.5, z:0.595},
    //     control_point1: Vec3 {x:-0.5, y:-1.5, z:0.595},
    //     control_point2: Vec3 {x:-0.3, y:-1.5, z:0.595},
    //     end_point: Vec3 {x:0.0, y:-1.5, z:0.595},
    // };

    // let bezier_path_point = BezierPath {  //point
    //     start_point: Vec3 {x:-0.85, y:-1.1, z:0.7},
    //     control_point1: Vec3 {x:-0.5, y:-1.1, z:0.7},
    //     control_point2: Vec3 {x:-0.3, y:-1.1, z:0.7},
    //     end_point: Vec3 {x:0.0, y:-1.1, z:0.7},
    // };
    
    //let t = 0.25;
    //let mut cam_pos = Vec3::zeros();
    // if t < 0.5 {
    //     cam_pos = evaluate_cubic_bezier(&bezier_path1, t*2.0);
    //     num_of_samples = 4;
    // } else {
    //     cam_pos = evaluate_cubic_bezier(&bezier_path2, (t-0.5)*2.0);
    //     num_of_samples = 10;
    // };

    //let cam_pos = Vec3 {x:-0.86, y:-0.55, z:0.595};
    //let point_pos = Vec3{x:-0.8, y:-0.55, z:0.54};
    
    //let cam_pos = evaluate_cubic_bezier(&bezier_path8, t);
    //let cam_pos = Vec3 {x:-0.85, y:-1.5, z:0.595};
    //let point_pos = evaluate_cubic_bezier(&bezier_path_point, t);

    //let cam_pos = Vec3{x:-6.1, y:-3.88, z:-3.858};
    //let cam_pos = Vec3{x:-0.9, y:-0.55, z:0.595};
    //let cam_pos = Vec3 { x: -0.7, y: -0.53, z: 0.6 };
    let cam_pos = Vec3{x:0.0, y:0.0, z:0.0};
    
    //let point_pos = Vec3{x:-3.0, y:-3.88, z:-3.858};
    //  let point_pos = Vec3{x:0.0, y:0.0, z:0.0};

    //let phi: f32 = - PI / 4.0;          //spin
    let phi: f32 = 0.0;
    let theta: f32 = PI / 2.0;  //up/down












    //Convert
    //let (phi, theta) = look_at_point(cam_pos, point_pos);
    let mut cam_dir = Vec3{
        x: theta.sin() * phi.cos(),
        y: theta.cos(),
        z: theta.sin() * phi.sin(), 
    };
    cam_dir = Vec3::normalize(&cam_dir);
    println!("Camera Dir: {:?}", cam_dir);
    println!("Camera Pos: {:?}", cam_pos);
    return Camera{position: cam_pos, direction: cam_dir};
}



