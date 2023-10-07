
use crate::operations::Op;
use crate::Op::Move;
use crate::Op::Scale;
use crate::Op::Cut;
use crate::Op::Union;
use crate::Op::Intersection;
use crate::Op::SmoothUnion;
use crate::Op::SinDistortHeight;
use crate::Op::Sphere;
use crate::Op::Cube;
use crate::Op::Torus;
use crate::Op::CappedCone;
use crate::Op::Ellipsoid;
use crate::Op::Ellipsoid2;
use crate::Op::Line;
use crate::Op::Plane;
use crate::Op::Prism;
use crate::Op::RotateX;
use crate::Op::RotateY;
use crate::Op::RotateZ;
use crate::Op::InfRep;
use crate::Op::MirrorZ;
use crate::Op::SwirlY;
use crate::Op::Round;
use crate::Op::Texturize;
use crate::Op::Frac;
use crate::Op::Frac2;

use core::f32::consts::PI;

use crate::vec::ObjectData;
use crate::vec::Vec3;
use crate::vec::Vec2;


//Surface models:
//lambertian = 1
//mirror = 2

//room scene
// pub fn scene_1() -> Box<Op> {
//     let mut room = Cut(
//         Box::new(Cube(Vec3{x:10.0, y:1.0, z:1.0}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0)),
//         Box::new(Move(Box::new(Cube(Vec3{x:4.0, y:0.9, z:0.9}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:0.0, z:0.0}))
//     );

//     // let room_with_window = Cut(
//     //     Box::new(room.clone()),
//     //     Box::new(Move(Box::new(Cube(Vec3{x:0.3, y:0.3, z:0.3}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:-0.3, z:-0.9}))
//     // );

//     let cube_stack = Union(
//         Box::new(Move(Box::new(RotateY(Box::new(Cube(Vec3{x:0.2, y:0.2, z:0.2}, Vec3{x:1.0, y:0.0, z:0.0}, 1.0, 1, 0.0)),1.0)), Vec3{x:-0.5, y:0.7, z:-0.3})),
//         Box::new(Move(Box::new(RotateY(Box::new(Cube(Vec3{x:0.1, y:0.1, z:0.1}, Vec3{x:0.0, y:1.0, z:0.0}, 1.0, 2, 0.0)),0.5)), Vec3{x:-0.5, y:0.4, z:-0.3})), 
//     );

//     let sphere_mirror = Box::new(Move(Box::new(Scale(Box::new(Sphere(Vec3{x:0.0, y:0.0, z:1.0}, 1.0, 1, 0.0)),0.2)), Vec3{x:-0.4, y:0.7, z:0.4}));

//     let room_and_objs = Union(
//        Box::new(room.clone()),
//        Box::new(Union(
//             Box::new(cube_stack.clone()),
//             Box::new(*sphere_mirror.clone()),
//        ))
//     );

//     let room_and_sun = Union(
//         Box::new(room_and_objs.clone()),
//         Box::new(Move(Box::new(Scale(Box::new(Sphere(Vec3{x:253.0, y:251.0, z:211.0}, 1.0, 1, 20.0)),0.2)), Vec3{x:0.0, y:-0.9, z:0.0})),
//     );

//     let objects = Box::new(Move(Box::new(room_and_sun), Vec3{x:3.0, y:0.0, z:0.0}));
//     return objects
// }

// //Infballs scene
// pub fn scene_2() -> Box<Op> {
//     let mut objects = Box::new(Sphere(Vec3{x:1.0, y:1.0, z:1.0}, 0.9, 2, 0.0));
//     objects = Box::new(InfRep(Box::new(*objects), Vec3{x:5.0, y:5.0, z:5.0}));
//     objects = Box::new(Move(Box::new(*objects), Vec3{x:5., y:-102.5, z:-102.5}));
//     return objects
// }

// //Menger Sponge
// pub fn scene() -> Box<Op> {
//     let mut cross_0 = Union(
//         Box::new(Union(
//             Box::new(Cube(Vec3{x:10000.0,y:1.,z:1.}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0)),
//             Box::new(Cube(Vec3{x:1.,y:10000.0,z:1.}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0))
//         )),
//         Box::new(Cube(Vec3{x:1.,y:1.,z:10000.0}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0))
//     );

//     let layer_0 = Box::new(Move(Box::new(Cube(Vec3{x:1.,y:1.,z:1.}, Vec3{x:225.0, y:225.0, z:225.0}, 1.0, 1, 0.0, 1.0)),Vec3{x:2.0,y:2.0,z:2.0}));
//     let cross_1 = Box::new(Scale(Box::new(cross_0.clone()),0.333));
//     let layer_1 = Cut(
//         Box::new(*layer_0.clone()),
//         //Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0/(3.*i as f32), y:2.0/(3.*i as f32), z:2.0/(3.*i as f32)}))
//         Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0, y:2.0, z:2.0}))
//     );
//     let cross_2 = Box::new(Scale(Box::new(*cross_1.clone()),0.333));
//     let mut layer_2 = Cut(
//         Box::new(layer_1.clone()),
//         Box::new(InfRep(Box::new(*cross_2.clone()), Vec3{x:0.666, y:0.666, z:0.666}))
//     );
//     let cross_3 = Box::new(Scale(Box::new(*cross_2.clone()),0.333));
//     let mut layer_3 = Cut(
//         Box::new(layer_2.clone()),
//         Box::new(InfRep(Box::new(*cross_3.clone()), Vec3{x:0.222, y:0.222, z:0.222}))
//     );
//     let cross_4 = Box::new(Scale(Box::new(*cross_3.clone()),0.333));
//     let mut layer_4 = Cut(
//         Box::new(layer_3.clone()),
//         Box::new(InfRep(Box::new(*cross_4.clone()), Vec3{x:0.074, y:0.074, z:0.074}))
//     );
//     let cross_5 = Box::new(Scale(Box::new(*cross_4.clone()),0.333));
//     let mut layer_5 = Cut(
//         Box::new(layer_4.clone()),
//         Box::new(InfRep(Box::new(*cross_5.clone()), Vec3{x:0.074/3., y:0.074/3., z:0.074/3.}))
//     );


//     //let objects = Box::new(Move(Box::new(objects), Vec3{x:-1.0, y:-2.3, z:-2.0}));
//     //let objects = Box::new(Move(Box::new(layer_5), Vec3{x:-1.1, y:-1.35, z:-2.0}));
//     let objects = Box::new(Move(Box::new(layer_5), Vec3{x:-1.7, y:-2.66, z:-1.33}));
//     //let objects = Box::new(RotateZ(Box::new(*objects), -0.3));
//     //let objects = Box::new(RotateY(Box::new(*objects), 0.3));
//     return objects
// }

// //Menger Sponge, version 2
// pub fn scene() -> Box<Op> {
//     let mut cross_0 = Union(
//         Box::new(Union(
//             Box::new(Cube(Vec3{x:10000.0,y:1.,z:1.}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0)),
//             Box::new(Cube(Vec3{x:1.,y:10000.0,z:1.}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0))
//         )),
//         Box::new(Cube(Vec3{x:1.,y:1.,z:10000.0}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0))
//     );
//     //let layer_0 = Box::new(SwirlY(Box::new(Move(Box::new(Cube(Vec3{x:1.,y:1.,z:1.}, Vec3{x:225.0, y:225.0, z:225.0}, 1.0, 1, 0.0, 1.0)),Vec3{x:2.0,y:2.0,z:2.0})),2.0));
//     let layer_0 = Box::new(Move(Box::new(SwirlY(Box::new(Cube(Vec3{x:1.,y:1.,z:1.}, Vec3{x:225.0, y:225.0, z:225.0}, 1.0, 1, 0.0, 1.0)),0.2)),Vec3{x:2.0,y:2.0,z:2.0}));
//     let mut cross_1 = Box::new(Scale(Box::new(cross_0.clone()),0.333));
//     cross_1 = Box::new(RotateZ(Box::new(*cross_1), -0.1));
//     cross_1 = Box::new(RotateY(Box::new(*cross_1), 0.5));
//     cross_1 = Box::new(SwirlY(cross_1, -0.3));
//     let layer_1 = Cut(
//         Box::new(*layer_0.clone()),
//         //Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0/(3.*i as f32), y:2.0/(3.*i as f32), z:2.0/(3.*i as f32)}))
//         Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0, y:2.0, z:2.0}))
//     );
//     let mut cross_2 = Box::new(Scale(Box::new(*cross_1.clone()),0.333));
//     cross_2 = Box::new(RotateZ(Box::new(*cross_2), 1.1));
//     cross_2 = Box::new(RotateY(Box::new(*cross_2), 0.2));
//     cross_2 = Box::new(SwirlY(cross_2, 0.3));
//     let mut layer_2 = Cut(
//         Box::new(layer_1.clone()),
//         Box::new(InfRep(Box::new(*cross_2.clone()), Vec3{x:0.666, y:0.666, z:0.666}))
//     );
//     let mut cross_3 = Box::new(Scale(Box::new(*cross_2.clone()),0.333));
//     cross_3 = Box::new(RotateZ(Box::new(*cross_3), -1.0));
//     cross_3 = Box::new(RotateY(Box::new(*cross_3), -0.1));
//     cross_3 = Box::new(SwirlY(cross_3, 0.1));
//     let mut layer_3 = Cut(
//         Box::new(layer_2.clone()),
//         Box::new(InfRep(Box::new(*cross_3.clone()), Vec3{x:0.222, y:0.222, z:0.222}))
//     );
//     let mut cross_4 = Box::new(Scale(Box::new(*cross_3.clone()),0.333));
//     cross_4 = Box::new(RotateZ(Box::new(*cross_4), 0.2));
//     cross_4 = Box::new(RotateY(Box::new(*cross_4), -0.5));
//     cross_4 = Box::new(SwirlY(cross_4, -0.4));
//     let mut layer_4 = Cut(
//         Box::new(layer_3.clone()),
//         Box::new(InfRep(Box::new(*cross_4.clone()), Vec3{x:0.074, y:0.074, z:0.074}))
//     );
//     let mut cross_5 = Box::new(Scale(Box::new(*cross_4.clone()),0.333));
//     cross_5 = Box::new(RotateZ(Box::new(*cross_5), 0.13));
//     cross_5 = Box::new(RotateY(Box::new(*cross_5), 0.3));
//     cross_5 = Box::new(SwirlY(cross_5, 0.2));
//     let mut layer_5 = Cut(
//         Box::new(layer_4.clone()),
//         Box::new(InfRep(Box::new(*cross_5.clone()), Vec3{x:0.074/3., y:0.074/3., z:0.074/3.}))
//     );

//     let objects = Box::new(Move(Box::new(layer_5), Vec3{x:-2.0, y:-2.3, z:-2.0}));
//     //let objects = Box::new(Move(Box::new(objects), Vec3{x:0.5, y:-1.3, z:-2.0}));
//     let objects = Box::new(RotateZ(Box::new(*objects), -0.6));
//     let objects = Box::new(RotateY(Box::new(*objects), 4.0));
//     return objects
// }


//spheres on plane
// pub fn scene_4() -> Box<Op> {




//     let mut plane = Box::new(Cube(Vec3{x:100.0, y:1.0, z:100.0}, Vec3{x:210.0, y:210.0, z:210.0}, 1.0, 1, 0.0));
//     let mut sphere1 = Box::new(Move(Box::new(Sphere(Vec3{x:0.0, y:1.0, z:0.0}, 1.0, 2, 5.0)), Vec3{x:0.0, y:-2.0, z:-1.5}));

//     let mut sphere2 = Box::new(Sphere(Vec3{x:150.0, y:150.0, z:150.0}, 1.0, 2, 0.0));
//     sphere2 = Box::new(Scale(Box::new(*sphere2),3.));
//     sphere2 = Box::new(Move(Box::new(*sphere2),Vec3{x:3.5, y:-4.0, z:2.0}));

//     let mut sphere3 = Box::new(Sphere(Vec3{x:1.0, y:0.0, z:0.0}, 1.0, 2, 0.0));
//     sphere3 = Box::new(Scale(Box::new(*sphere3),1.));
//     sphere3 = Box::new(Move(Box::new(*sphere3),Vec3{x:0.0, y:-2.0, z:2.0}));


//     let mut objects = Union(
//         Box::new(*sphere3.clone()),
//         Box::new(Union(
//             Box::new(*sphere2.clone()),
//             Box::new(SmoothUnion(
//                 Box::new(*plane.clone()),
//                 Box::new(*sphere1.clone()),
//                 2.0,
//             ))
//         ))    
//     );

//     objects = *Box::new(Move(Box::new(objects.clone()), Vec3{x:5.0, y:3.0, z:0.0}));
//     return Box::new(objects)
// }

// //desert
// pub fn scene_5() -> Box<Op> {
//     let mut plane = Box::new(Cube(Vec3{x:100.0, y:10.0, z:100.0}, Vec3{x:140.0, y:110.0, z:110.0}, 1.0, 1, 0.0));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 3.0, 0.11));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 2.0, 0.51));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 1.0, 1.01));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 0.5, 2.01));
    
    
    
//     let objects = Box::new(Move(Box::new(*plane), Vec3{x:3.0, y:13.0, z:0.0}));
//     return objects
// }

// //Sand and cactus scene
// pub fn scene_6() -> Box<Op> {
//     let mut plane = Box::new(Cube(Vec3{x:100.0, y:10.0, z:100.0}, Vec3{x:140.0, y:110.0, z:110.0}, 1.0, 1, 0.0));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 2.0, 0.11));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 1.0, 0.51));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 0.5, 1.01));
//     plane = Box::new(RotateY(Box::new(*plane), 0.5));
//     plane = Box::new(SinDistortHeight(Box::new(*plane), 0.25, 2.01));


//     let pot1 = Box::new(CappedCone(1.0, 1.5, 1.0, Vec3{x: 173.0, y:80.0, z:73.0}, 1.0, 1, 0.0));
//     let pot2 = Box::new(Move(Box::new(CappedCone(1.0, 1.5, 1.0, Vec3{x: 173.0, y:80.0, z:73.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:-0.8, z:0.0}));

//     let pot = Cut(
//         Box::new(*pot1),
//         Box::new(*pot2)
//     );

//     let mut cactus_stem = Box::new(Ellipsoid(Vec3{x:1.0, y:3.0, z:1.0}, Vec3{x:0.0, y:1.0, z:0.0}, 1.0, 1, 0.0));
//     cactus_stem = Box::new(Move(Box::new(*cactus_stem.clone()),Vec3{x:0.0, y:-2.5, z:0.0}));

//     let thorn = Box::new(Line(Vec3{x:0.0, y:0.0, z:-1.2}, Vec3{x:0.0, y:0.0, z:1.2}, 0.05, Vec3{x:0.0, y:0.0, z:0.0}, 1.0, 1, 0.0));
//     let thorns_rot1 = Box::new(RotateY(Box::new(*thorn.clone()), PI/3.0));
//     let thorns_rot2 = Box::new(RotateY(Box::new(*thorns_rot1.clone()), PI/3.0));

//     let thorns_level = Union(
//         Box::new(*thorn.clone()),
//         Box::new(Union(
//             Box::new(*thorns_rot1.clone()),
//             Box::new(*thorns_rot2.clone())
//         ))
//     );

//     let mut thorns = Union (
//         Box::new(thorns_level.clone()),
//         Box::new(Union(
//             Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-0.8, z:0.0})),
//             Box::new(Union(
//                 Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-1.6, z:0.0})),
//                 Box::new(Union(
//                     Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-2.4, z:0.0})),
//                     Box::new(Move(Box::new(Scale(Box::new(thorns_level.clone()),0.8)),Vec3{x:0.0, y:-3.2, z:0.0}))
//                 ))
//             ))
//         ))
//     ); 

//     thorns = *Box::new(Move(Box::new(thorns),Vec3{x:0.0, y:-1.4, z:0.0}));

//     let cactus = Union(
//         Box::new(*cactus_stem.clone()),
//         Box::new(thorns.clone())
//     );

//     let mut pot_and_cactus = Union(
//         Box::new(pot.clone()),
//         Box::new(cactus.clone())
//     );

//     pot_and_cactus = *Box::new(Scale(Box::new(pot_and_cactus.clone()), 1.0));

//     pot_and_cactus = *Box::new(Move(Box::new(pot_and_cactus.clone()), Vec3{x:3.0, y:-11.0, z:0.0}));

//     let total = Union(
//         Box::new(pot_and_cactus.clone()),
//         Box::new(*plane.clone())
//     );

//     let objects = Box::new(Move(Box::new(total), Vec3{x:7.0, y:14.0, z:0.0}));

//     return objects
// }

// //street
// pub fn scene_7() -> Box<Op> {
//     let lamp_height = -4.0;

//     let lamp = Box::new(Sphere(Vec3{x:253.0, y:251.0, z:211.0}, 1.0, 1, 5.0));
//     let pole = Box::new(CappedCone(-lamp_height, 0.1, 0.1, Vec3{x:150.0, y:150.0, z:150.0}, 1.0, 1, 0.0));
//     let lamp_post = Union(
//         Box::new(*lamp.clone()),
//         Box::new(Move(Box::new(*pole.clone()), Vec3{x:0.0, y:-lamp_height, z:0.0})),
//     );

//     let pavement = Box::new(Cube(Vec3{x:10.0, y:0.1, z:10.0}, Vec3{x:100.0, y:100.0, z:100.0}, 1.0, 1, 0.0));
//     //let line = Box::new(Cube(Vec3{x:2.0, y:0.1, z:0.2}, Vec3{x:1.0, y:1.0, z:0.0}, 1.0, 1, 0.0));

//     // let mut lines = Union(
//     //     Box::new(Move(Box::new(*line.clone()), Vec3{x:0.0, y:-0.1, z:-4.0})),
//     //     Box::new(Move(Box::new(*line.clone()), Vec3{x:5.0, y:-0.1, z:-4.0})),
//     // );

//     // let mut street = Union(
//     //     Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
//     //     Box::new(Move(Box::new(lines.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
//     // );

//     // let wall = Box::new(Cube(Vec3{x:3.0, y:8.0, z:3.0}, Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0));
//     // let window = Box::new(Cube(Vec3{x:1.0, y:2.0, z:0.2}, Vec3{x:1.0, y:200.0, z:0.0}, 1.0, 1, 1.5));

//     // let building = Union(
//     //     Box::new(Move(Box::new(*wall.clone()), Vec3{x:0.0, y:-5.0, z:7.0})),
//     //     Box::new(Move(Box::new(*window.clone()), Vec3{x:0.0, y:-2.0, z:4.0})),
//     // );


//     // let mut scene = Union(
//     //     Box::new(Move(Box::new(lamp_post.clone()), Vec3{x:0.0, y:lamp_height, z:0.0})),
//     //     Box::new(Union(
//     //         Box::new(Move(Box::new(building.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
//     //         Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
//     //     ))
//     // );

//     let mut scene = Union(
//         Box::new(Move(Box::new(lamp_post.clone()), Vec3{x:0.0, y:lamp_height, z:0.0})),
//         Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
//     );

//     scene = *Box::new(InfRep(Box::new(scene),Vec3{x:10.0, y:1000.0, z:1000.0}));

//     scene = *Box::new(Move(Box::new(scene.clone()), Vec3{x:5.0, y:1.0, z:4.0}));
//     scene = *Box::new(MirrorZ(Box::new(scene.clone())));
    
//     return Box::new(scene)
// }

//spheres on plane
// pub fn scene() -> Box<Op> {

//     let plane_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 10.0,
//         refractive_index: 0.0 
//     };

//     let cube_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 3,
//         emission_rate: 0.0,
//         refractive_index: 1.5
//     };

//     let sphere2_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.1, z: 0.1 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 1.5 
//     };

//     let sphere3_att = ObjectData { 
//         color: Vec3 { x: 0.1, y: 0.9, z: 0.1 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 1.5 
//     };

//     //Plane
//     let mut plane = Box::new(Plane(-1.0, plane_att));
//     //let mut plane = Box::new(Cube(Vec3{x:100.0, y:1.0, z:100.0}, Vec3{x:210.0, y:210.0, z:210.0}, 1.0, 1, 0.0, 0.0));
    
//     //Glass cube
//     let mut cube = Box::new(Cube(Vec3{x:1.0, y:1.0, z:1.0}, cube_att));
//     cube = Box::new(Scale(Box::new(*cube),0.35));
//     cube = Box::new(Move(Box::new(*cube),Vec3{x:-1.0, y:-1.35, z:1.0}));

//     //Torus
//     // let mut torus = Box::new(Torus(Vec2{x:1.0, y:0.5},Vec3{x:0.0, y:220.0, z:0.0}, 1.0, 2, 0.0, 1.5));
//     // torus = Box::new(Scale(Box::new(*torus),0.5));
//     // torus = Box::new(Move(Box::new(*torus),Vec3{x:-2.0, y:1.0, z:0.0}));
//     // torus = Box::new(RotateZ(Box::new(*torus), PI/1.5));

//     //light green Large sphere
//     let mut sphere2 = Box::new(Sphere(sphere2_att));
//     sphere2 = Box::new(Scale(Box::new(*sphere2),1.5));
//     sphere2 = Box::new(Move(Box::new(*sphere2),Vec3{x:1.0, y:-2.5, z:-1.0}));

//     //Red small sphere
//     let mut sphere3 = Box::new(Sphere(sphere3_att));
//     sphere3 = Box::new(Scale(Box::new(*sphere3),1.0));
//     sphere3 = Box::new(Move(Box::new(*sphere3),Vec3{x:1.0, y:-2.0, z:2.0}));

//     let mut objects = Union(
//         Box::new(*plane.clone()),
//         Box::new(Union(
//             Box::new(*sphere2.clone()),
//             Box::new(Union(
//                 cube,
//                 sphere3
//                 ))
//           //      Box::new(*sphere2.clone()),
//           //      Box::new(Union(
//           //          Box::new(*sphere3.clone()),
//           //          Box::new(*cube.clone()),
//         )));//)))));

//     objects = *Box::new(Move(Box::new(objects.clone()), Vec3{x:4.0, y:2.0, z:0.0}));
//     return Box::new(objects)
// }



//Glass on table
// pub fn scene() -> Box<Op> {
    
//     //Floor
//     let mut plane = Box::new(Plane(1.0, Vec3{x:210.0, y:210.0, z:210.0}, 1.0, 1, 0.0, 0.0));
    
//     //Table top
//     let mut table_top = Box::new(Cube(Vec3{x:0.5, y:0.025, z:0.5}, Vec3{x:252.0, y:204.0, z:156.0}, 1.0, 1, 0.0, 0.0));
    
//     //Table leg
//     let mut leg1 = Box::new(Line(Vec3{x:-0.45, y:0.025, z:-0.45}, Vec3{x:-0.45, y:10., z:-0.45}, 0.04, Vec3{x:252.0, y:204.0, z:156.0}, 1.0, 1, 0.0, 0.0));
//     let mut leg2 = Box::new(Line(Vec3{x:0.45, y:0.025, z:-0.45}, Vec3{x:0.45, y:10., z:-0.45}, 0.04, Vec3{x:252.0, y:204.0, z:156.0}, 1.0, 1, 0.0, 0.0));
//     let mut leg3 = Box::new(Line(Vec3{x:0.45, y:0.025, z:0.45}, Vec3{x:0.45, y:10., z:0.45}, 0.04, Vec3{x:252.0, y:204.0, z:156.0}, 1.0, 1, 0.0, 0.0));
//     let mut leg4 = Box::new(Line(Vec3{x:-0.45, y:0.025, z:0.45}, Vec3{x:-0.45, y:10., z:0.45}, 0.04, Vec3{x:252.0, y:204.0, z:156.0}, 1.0, 1, 0.0, 0.0));

//     //Table assembly
//     let mut table = Union(
//         Box::new(*table_top.clone()),
//         Box::new(Union(
//             Box::new(*leg1.clone()),
//             Box::new(Union(
//                 Box::new(*leg2.clone()),
//                 Box::new(Union(
//                     Box::new(*leg3.clone()),
//                     Box::new(*leg4.clone()),
//     )))))));

//     //Glass
//     let glass1 = Box::new(CappedCone(0.15, 0.08, 0.05, Vec3{x: 0.0, y:0.0, z:0.0}, 1.0, 3, 0.0, 1.5));
//     let glass2 = Box::new(Move(Box::new(CappedCone(0.15, 0.08, 0.05, Vec3{x: 0.0, y:0.0, z:0.0}, 1.0, 3, 0.0, 1.5)), Vec3{x:0.0, y:-0.01, z:0.0}));

//     let mut glass = Cut(
//         Box::new(*glass1),
//         Box::new(*glass2)
//     );

//     glass = *Box::new(Move(Box::new(glass), Vec3{x:-0.4, y:-0.15, z:0.4}));

//     let mut objects = Union(
//         Box::new(table.clone()),
//         Box::new(Union(
//             Box::new(glass),
//             Box::new(*plane),
//     )));
    

//     objects = *Box::new(RotateY(Box::new(objects),0.65));
//     objects = *Box::new(Move(Box::new(objects), Vec3{x:1.2, y:0.2, z:0.0}));

//     return Box::new(objects)
// }





// //room with skylight
// pub fn scene() -> Box<Op> {
    
//     let floor_size: f32 = 0.2;
//     let room_height: f32 = 6.0;
//     let room_depth: f32 = 15.0;
//     let room_width: f32 = 7.0;
//     let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\tiles.png";
//     let tex1 = image::open(path1).expect("File not found!");
//     let tex1_scale = 5.;

//     let mut tiles = Box::new(Plane(0.0, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     tiles = Box::new(Move(Box::new(*tiles), Vec3{x:-5.0, y:0.0, z:-5.0}));
//     tiles = Box::new(Texturize(Box::new(*tiles), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));
    
//     //Floor
//     // let mut tile1 = Box::new(Cube(Vec3{x:floor_size, y:0.1, z:floor_size}, Vec3{x:240.0, y:240.0, z:240.0}, 1.0, 1, 0.0, 0.0));
//     // let mut tile2 = Box::new(Cube(Vec3{x:floor_size, y:0.1, z:floor_size}, Vec3{x:200.0, y:200.0, z:200.0}, 1.0, 1, 0.0, 0.0));
//     // let mut tile3 = Box::new(Cube(Vec3{x:floor_size, y:0.1, z:floor_size}, Vec3{x:240.0, y:240.0, z:240.0}, 1.0, 1, 0.0, 0.0));
//     // let mut tile4 = Box::new(Cube(Vec3{x:floor_size, y:0.1, z:floor_size}, Vec3{x:200.0, y:200.0, z:200.0}, 1.0, 1, 0.0, 0.0));

//     // tile1 = Box::new(Move(Box::new(*tile1), Vec3{x:-floor_size*1.0, y:0.0, z:-floor_size*1.0}));
//     // tile2 = Box::new(Move(Box::new(*tile2), Vec3{x:floor_size*1.0, y:0.0, z:-floor_size*1.0}));
//     // tile3 = Box::new(Move(Box::new(*tile3), Vec3{x:floor_size*1.0, y:0.0, z:floor_size*1.0}));
//     // tile4 = Box::new(Move(Box::new(*tile4), Vec3{x:-floor_size*1.0, y:0.0, z:floor_size*1.0}));

//     // let mut tile = Union(
//     //             Box::new(*tile1.clone()),
//     //             Box::new(Union(
//     //                 Box::new(*tile2.clone()),
//     //                 Box::new(Union(
//     //                     Box::new(*tile3.clone()),
//     //                     Box::new(*tile4.clone()),
//     // )))));

//     // let mut tiles = Box::new(InfRep(Box::new(tile), Vec3{x:floor_size*4.0, y:1000.0, z:floor_size*4.0}));
//     // tiles = Box::new(Move(Box::new(*tiles.clone()), Vec3{x:-100.0, y:0.0, z:-100.0}));
    

//     //room wall and roof
//     let mut room1 = Box::new(Cube(Vec3{x:room_depth/2.0+1.0, y:room_height/2.0+0.5, z:room_width/2.0+1.0}, Vec3{x:240.0, y:240.0, z:240.0}, 1.0, 1, 0.0, 0.0));
//     let mut room2 = Box::new(Cube(Vec3{x:room_depth/2.0, y:room_height/2.0, z:room_width/2.0}, Vec3{x:240.0, y:240.0, z:240.0}, 1.0, 1, 0.0, 0.0));
//     let mut skylight =  Box::new(Cube(Vec3{x:room_depth/2.0-1.0, y:room_height+100.0, z:room_width/2.0-2.0}, Vec3{x:240.0, y:240.0, z:240.0}, 1.0, 1, 0.0, 0.0));
    
//     let mut cutter = Box::new(Union(
//         Box::new(*room2),
//         Box::new(*skylight),
//     ));
    
//     let room = Box::new(Cut(
//         Box::new(*room1),
//         Box::new(*cutter),
//     ));

    
//     //Round pillars
//     let mut pillar_top = Box::new(CappedCone(0.15, 0.5, 0.5, Vec3{x: 1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0));
//     pillar_top = Box::new(Move(Box::new(*pillar_top), Vec3{x:0.0, y:-room_height/2.0, z:0.0}));
//     let pillar_mid = Box::new(CappedCone(room_height/2.0, 0.4, 0.4, Vec3{x: 1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0));
//     let pillar_bot = Box::new(CappedCone(0.2, 0.5, 0.5, Vec3{x: 1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 1.0));
    
//     //Assembly - Pillar
//     let pillar = Box::new(Union(
//         Box::new(*pillar_top),
//         Box::new(Union(
//             Box::new(*pillar_mid),
//             Box::new(*pillar_bot),
//     ))));

//     let mut pillars = Box::new(InfRep(Box::new(*pillar), Vec3{x:1.8, y:1000.0, z:4.2}));
//     pillars = Box::new(Move(Box::new(*pillars), Vec3{x:-2.0, y:0.0, z:-2.1}));

//     // //Plant
//     // let mut stem = Box::new(Line(Vec3{x:0.4, y:0.0, z:0.0}, Vec3{x:0.4, y:-room_height/2.0, z:0.0}, 0.2, Vec3{x:100.0, y:1.0, z:100.0}, 1.0, 1, 0.0, 1.0));
//     // stem = Box::new(SwirlY(Box::new(*stem), 6.0));
//     // stem = Box::new(Move(Box::new(*stem), Vec3{x:-2.0+1.8*2.0, y:0.0, z:2.1}));
    
//     // //Altar
//     // let mut altar_base = Box::new(Cube(Vec3{x:0.2, y:0.5, z:0.2}, Vec3{x:200.0, y:100.0, z:100.0}, 1.0, 1, 0.0, 1.0));
//     // altar_base = Box::new(SwirlY(Box::new(*altar_base), 4.0));
//     // altar_base = Box::new(Move(Box::new(*altar_base), Vec3{x:2.0, y:-0.5, z:0.0}));
    
//     // let mut altar_sphere = Box::new(Sphere(Vec3{x:200.0, y:10.0, z:10.0}, 1.0, 3, 0.0, 1.0));
//     // altar_sphere = Box::new(Scale(Box::new(*altar_sphere), 0.5));
//     // altar_sphere = Box::new(Move(Box::new(*altar_sphere), Vec3{x:2.0, y:-1.25, z:0.0}));
    
//     // //Altar Assembly
//     // let mut altar = Box::new(Union(
//     //     Box::new(*altar_base),
//     //     Box::new(*altar_sphere),
//     // ));

//     //let mut sphere = Box::new(Sphere(Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 5.0, 1.0));
//     //sphere = Box::new(Scale(Box::new(*sphere), 0.75));
//     //sphere = Box::new(Move(Box::new(*sphere), Vec3{x:2.0, y:-1.25, z:0.0}));

//     //Assembly
//     let mut objects = Box::new(Union(
//         Box::new(*room),
//         Box::new(Union(
//             Box::new(*tiles),
//             Box::new(*pillars)
//                 //Box::new(*pillars),
//                 //Box::new(*sphere),
//                     //Box::new(*stem),  
//                     //Box::new(*altar),  
//     ))));

//     //Camera positioning
//     objects = Box::new(Move(Box::new(*objects), Vec3{x:3.0, y:1.0, z:0.0}));
//     objects = Box::new(RotateY(Box::new(*objects),-0.2));
    
//     return Box::new(*objects)
    
// }




//swirl test
// pub fn scene() -> Box<Op> {
//     //Step length modifier = 0.5

//     let plane = Box::new(Plane(1.0, Vec3{x:200.0, y:200.0, z:200.0}, 1.0, 1, 0.0, 0.0));
//     let mut cube = Box::new(Cube(Vec3{x:0.6, y:2.0, z:0.6}, Vec3{x:240.0, y:140.0, z:140.0}, 1.0, 1, 0.0, 0.0));
//     cube = Box::new(Move(Box::new(*cube), Vec3{x:0.0, y:-1.0, z:0.0}));
//     cube = Box::new(SwirlY(Box::new(*cube),2.0));

//     //Assembly
//     let mut objects = Box::new(Union(
//         Box::new(*cube),
//         Box::new(*plane),
//     ));

//     //Camera positioning
//     objects = Box::new(RotateZ(Box::new(*objects),-0.7));
//     objects = Box::new(Move(Box::new(*objects), Vec3{x:6.0, y:0.0, z:0.0}));
    
    
//     return Box::new(*objects)
    
// }



//texture test
// pub fn scene() -> Box<Op> {
//     //Textures
//     let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\dirt.png";
//     let tex1 = image::open(path1).expect("File not found!");
//     let tex1_scale = 200.0;
//     let path2 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\grass_large.png";
//     let tex2 = image::open(path2).expect("File not found!");
//     let tex2_scale = 500.0;
    

//     let mut plane = Box::new(Plane(1.0, Vec3{x:200.0, y:200.0, z:200.0}, 1.0, 1, 0.0, 0.0));
//     plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

//     let mut sphere = Box::new(Sphere(Vec3{x:250.0, y:250.0, z:250.0}, 1.0, 1, 5.0, 1.0));
//     sphere = Box::new(Scale(Box::new(*sphere), 0.75));
//     sphere = Box::new(Move(Box::new(*sphere), Vec3{x:4.0, y:0.25, z:0.0}));
//     sphere = Box::new(Texturize(Box::new(*sphere), tex2, Vec3{x:1.0, y:0.0, z:0.0}*tex2_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex2_scale));


//     //Assemble
//     let objects = Box::new(SmoothUnion(
//         Box::new(*sphere),
//         Box::new(*plane),
//         2.0,
//     ));

//     return Box::new(*objects)
// }


//fog test
// pub fn scene() -> Box<Op> {

//     let mut wall1 = Box::new(Cube(Vec3{x:100.0, y:100.0, z:0.1}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     wall1 = Box::new(Move(Box::new(*wall1), Vec3{x:100.5, y:0.0, z:0.0}));

//     let mut wall2 = Box::new(Cube(Vec3{x:100.0, y:100.0, z:0.1}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     wall2 = Box::new(Move(Box::new(*wall2), Vec3{x:-100.5, y:0.0, z:0.0}));

//     let mut wall3 = Box::new(Cube(Vec3{x:0.1, y:100.0, z:100.0}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     wall3 = Box::new(Move(Box::new(*wall3), Vec3{x:1.0, y:0.0, z:-100.0}));

//     let mut light = Box::new(Cube(Vec3{x:1.0, y:100.0, z:0.1}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 5.0, 0.0));
//     light = Box::new(Move(Box::new(*light), Vec3{x:0.0, y:0.0, z:2.0}));

//     //Assembly
//     let mut walls = Box::new(Union(
//         Box::new(*wall1),
//         Box::new(Union(
//             Box::new(*wall2),
//             Box::new(*wall3),
//         ))
//     ));

//     let mut sphere1 = Box::new(Sphere(Vec3{x:1.0, y:25.0, z:25.0}, 1.0, 1, 0.0, 1.0));
//     sphere1 = Box::new(Scale(Box::new(*sphere1), 0.5));
//     sphere1 = Box::new(Move(Box::new(*sphere1), Vec3{x:0.0, y:1.5, z:-2.0}));

//     let mut sphere2 = Box::new(Sphere(Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 2, 0.0, 1.0));
//     sphere2 = Box::new(Scale(Box::new(*sphere2), 0.5));
//     sphere2 = Box::new(Move(Box::new(*sphere2), Vec3{x:0.0, y:0.0, z:-2.0}));

//     let mut sphere3 = Box::new(Sphere(Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 3, 0.0, 0.5));
//     sphere3 = Box::new(Scale(Box::new(*sphere3), 0.5));
//     sphere3 = Box::new(Move(Box::new(*sphere3), Vec3{x:0.0, y:-1.5, z:-2.0}));

//     let mut spheres = Box::new(Union(
//         Box::new(*sphere1),
//         Box::new(Union(
//             Box::new(*sphere2),
//             Box::new(*sphere3)
//         ))
//     ));

//     //Assembly
//     let mut objects = Box::new(Union(
//         Box::new(*walls),
//         Box::new(Union(
//             Box::new(*spheres),
//             Box::new(*light)
//         ))
//     ));

//     //Camera positioning
//     //objects = Box::new(RotateZ(Box::new(*objects),-0.7));
//     objects = Box::new(Move(Box::new(*objects), Vec3{x:4.0, y:0.0, z:3.0}));
    
    
//     return Box::new(*objects)
    
// }


//Fog in room scene
// pub fn scene() -> Box<Op> {
//     //Textures
//     //let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\floor_boards.png";
//     //let tex1 = image::open(path1).expect("File not found!");
//     //let tex1_scale = 150.0;
//     //let path2 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\grass_large.png";
//     //let tex2 = image::open(path2).expect("File not found!");
//     //let tex2_scale = 500.0;
    

//     let mut plane = Box::new(Plane(1.0, Vec3{x:0.8, y:0.8, z:0.8}, 1.0, 1, 1.0, 0.0));
//     //plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

//     let room = Cut(
//         Box::new(Cube(Vec3{x:10.0, y:2.0, z:1.0}, Vec3{x:0.8, y:0.8, z:0.8}, 1.0, 1, 0.0, 0.0)),
//         Box::new(Move(Box::new(Cube(Vec3{x:4.0, y:1.9, z:0.9}, Vec3{x:0.8, y:0.8, z:0.8}, 1.0, 1, 0.0, 0.0)), Vec3{x:0.0, y:0.0, z:0.0}))
//     );

//     let room_with_window = Cut(
//         Box::new(room.clone()),
//         Box::new(Move(Box::new(Cube(Vec3{x:0.6, y:0.4, z:0.3}, Vec3{x:0.8, y:0.8, z:0.8}, 1.0, 1, 0.0, 0.0)), Vec3{x:2.0, y:-0.2, z:-0.9}))
//     );

//     let mut sphere = Box::new(Sphere(Vec3{x:1.0, y:0.0, z:0.0}, 1.0, 1, 0.0, 0.5));
//     sphere = Box::new(Scale(Box::new(*sphere), 0.2));
//     sphere = Box::new(Move(Box::new(*sphere), Vec3{x:2.5, y:0.8, z:0.0}));

//     //Assemble
//     let objects = Box::new(Union(
//         Box::new(room_with_window.clone()),
//         Box::new(Union(
//             Box::new(*plane),
//             Box::new(*sphere),
//         ))
//     ));

//     return Box::new(*objects)
// }


// pub fn scene() -> Box<Op> {
    

//     let mut plane = Box::new(Plane(1.0, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     //plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

//     let room = Cut(
//         Box::new(Cube(Vec3{x:10.0, y:2.0, z:1.0}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0)),
//         Box::new(Move(Box::new(Cube(Vec3{x:4.0, y:1.9, z:0.9}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0)), Vec3{x:0.0, y:0.0, z:0.0}))
//     );

//     let room_with_window = Cut(
//         Box::new(room.clone()),
//         Box::new(Move(Box::new(Cube(Vec3{x:0.6, y:0.4, z:0.3}, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0)), Vec3{x:2.0, y:0.0, z:-0.9}))
//     );

//     let mut sphere = Box::new(Sphere(Vec3{x:1.0, y:1.0, z:1.0}, 1.0, 1, 20.0, 0.5));
//     sphere = Box::new(Scale(Box::new(*sphere), 1.0));
//     sphere = Box::new(Move(Box::new(*sphere), Vec3{x:2.5, y:0.0, z:-4.0}));

//     //Assemble
//     let objects = Box::new(Union(
//         Box::new(room_with_window.clone()),
//         Box::new(Union(
//             Box::new(*plane),
//             Box::new(*sphere),
//         ))
//     ));

//     return Box::new(*objects)
// }


// pub fn scene() -> Box<Op> {
    
//     let scale = 1.0;
//     // let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\floor_boards_high_resolution.jpg";
//     // let tex1 = image::open(path1).expect("File not found!");
//     // let tex1_scale = 1000.0*scale;

//     //let mut plane = Box::new(Plane(1.0, Vec3{x:128.0, y:128.0, z:128.0}, 1.0, 1, 0.0, 0.0));
//     //plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

//     let mut frac = Box::new(Frac(Vec3{x:200.0, y:200.0, z:200.0}, 1.0, 1, 0.0, 0.0));
    
//     let mut cube = Box::new(Cube(Vec3 {x: 10.0, y:5.0, z:10.0}, Vec3 {x:1.0, y:1.0, z:1.0}, 1.0, 1, 0.0, 0.0));
//     cube = Box::new(Move(Box::new(*cube), Vec3{x:0.0, y:-5.0, z:0.0}));
    
//     frac = Box::new(Cut(
//         Box::new(*frac),
//         Box::new(*cube)
//     ));

//     frac = Box::new(Scale(Box::new(*frac), scale));
    
//     frac = Box::new(Move(Box::new(*frac), Vec3{x:1.2, y:0.3, z:0.0}*scale));

//     //let mut plane = Box::new(Plane(scale, Vec3{x:128.0,y:128.0,z:128.0}, 1.0, 1, 0.0, 0.0));
//     //plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));


//     // frac = Box::new(Union(
//     //     Box::new(*frac),
//     //     Box::new(*plane),
//     // ));
    
//     frac = Box::new(RotateZ(Box::new(*frac), -0.5));

    

//     //frac = Box::new(Move(Box::new(*frac), Vec3{x:0.0, y:0.0, z:-1.0}));
//     //frac = Box::new(RotateY(Box::new(*frac), 1.5));
    
//     //Assemble
//     // let objects = Box::new(Union(
//     //     Box::new(*plane),
//     //     Box::new(*fraq),
//     //     )
//     // );

//     return Box::new(*frac)
// }


// pub fn scene() -> Box<Op> {
    
//     let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\leaf2.png";
//     let tex1 = image::open(path1).expect("File not found!");
//     let tex1_scale = 200.0;
    
//     let path2 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\grass.png";
//     let tex2 = image::open(path2).expect("File not found!");
//     let tex2_scale = 10.0;

//     let path3 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\jungle.png";
//     let tex3 = image::open(path3).expect("File not found!");
//     let tex3_scale = 12.0;

//     let path4 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\bug.png";
//     let tex4 = image::open(path4).expect("File not found!");
//     let tex4_scale = 400.0;

//     // let mut plane = Box::new(Plane(0.5, Vec3{x:0.5, y:0.5, z:0.5}, 1.0, 1, 0.0, 0.0));
//     // plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

    
//     // let mut sphere = Box::new(Sphere(Vec3 { x: 0.8, y: 0.1, z: 0.1 }, 0.8, 2, 1.0, 1.0));
//     // sphere = Box::new(Move(Box::new(*sphere), Vec3 { x: 3.0, y: 0.0, z: 0.0 }));

//     // let objects = Box::new(Union(
//     //     Box::new(*plane),
//     //     Box::new(*sphere),
//     // ));
//     let leaf_size = 2.0;
//     let leaf1 = Box::new(Ellipsoid(Vec3 { x: 1.0, y: 1.0, z: 2.0 }*leaf_size, Vec3 { x: 0.0, y: 0.9, z: 0.0 }, 1.0, 1, 1.0, 1.0));
//     let mut leaf_cutter = Box::new(Cube(Vec3 { x: 10.0, y: 1.0, z: 10.0 }, Vec3 { x: 0.0, y: 0.9, z: 0.0 }, 1.0, 1, 1.0, 1.0));
//     let mut leaf_inter = Box::new(Cube(Vec3 { x: 10.0, y: 1.0, z: 10.0 }, Vec3 { x: 0.0, y: 0.9, z: 0.0 }, 1.0, 1, 1.0, 1.0));
    
//     leaf_cutter = Box::new(Move(Box::new(*leaf_cutter), Vec3 { x: 0.0, y: 1.0, z: 0.0 }));
//     leaf_inter = Box::new(Move(Box::new(*leaf_inter), Vec3 { x: 0.0, y: 0.95, z: 0.0 }));
    
//     let leaf2 = Box::new(Cut(
//         Box::new(*leaf1),
//         Box::new(*leaf_cutter),
//     ));

//     let leaf3 = Box::new(Intersection(
//         Box::new(*leaf2),
//         Box::new(*leaf_inter),
//     ));

//     let mut leaf_cutter2 = Box::new(Cube(Vec3 { x: 1.0, y: 10.0, z: 10.0 }, Vec3 { x: 0.0, y: 0.9, z: 0.0 }, 1.0, 1, 1.0, 1.0));
//     leaf_cutter2 = Box::new(Move(Box::new(*leaf_cutter2), Vec3 { x: 1.0, y: 0.0, z: 0.0 }));

//     let mut leaf4 = Box::new(Cut(
//         Box::new(*leaf3),
//         Box::new(*leaf_cutter2),
//     ));

//     let leaf5 = Box::new(Move(Box::new(*leaf4), Vec3 { x: 0.0, y: 0.0, z: 5.0 }));
//     let leaf5 = Box::new(Texturize(Box::new(*leaf5), tex1, Vec3 { x: 1.0, y: 0.0, z: 0.0 }*tex1_scale, Vec3 { x: 0.0, y: 0.0, z: 1.0 }*tex1_scale));
//     let leaf5 = Box::new(Move(Box::new(*leaf5), Vec3 { x: 0.0, y: 0.0, z: -5.0 }));


//     let mut leaf6 = Box::new(SmoothUnion(
//             (leaf5.clone()),
//             (Box::new(RotateZ(Box::new(*leaf5), (PI/1.3)))),
//             0.0,
//         ));
//     //leaf6 = Box::new(RotateY(Box::new(*leaf6), PI/4.));
//     leaf6 = Box::new(Move(Box::new(*leaf6), Vec3 { x: 0.0, y: 0.0, z: 1.7 }*leaf_size));
//     leaf6 = Box::new(RotateX(Box::new(*leaf6), -PI/10.));
    
//     let leafs = Box::new(Union(
//         Box::new(Move(Box::new(RotateY(Box::new(*leaf6.clone()), -PI/1.2)), Vec3{x:0.0, y:-2.0, z:0.0})),
//         leaf6,
//     ));
    
//     //leaf6 = Box::new(Round(leaf6, 0.05));

//     let stem = Box::new(Line(Vec3 { x:0.0, y:-10.0, z:0.0 }, Vec3 { x:0.0, y:10.0, z:0.0 }, 0.4, Vec3{x:0.0, y:0.7, z:0.0}, 1.0, 1, 1.0, 1.0));
    
//     let mut plant = Box::new(Union(
//         Box::new(*leafs), 
//         Box::new(*stem)
//     ));

//     //plant = Box::new(SwirlY(Box::new(*plant), PI/6.0));

//     let mut plane = Box::new(Plane(4.0, Vec3{x:0.5, y:0.5, z:0.5}, 1.0, 1, 0.0, 0.0));
//     plane = Box::new(Texturize(Box::new(*plane), tex2, Vec3{x:1.0, y:0.0, z:0.0}*tex2_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex2_scale));


//     let mut wdrop = Box::new(Sphere(Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 1.0, 3, 1.0, 1.5));
//     wdrop = Box::new(Move(Box::new(Scale(Box::new(*wdrop), 0.4)), Vec3 {x:0.0, y:-0.5, z:0.5}));

//     let plant = Box::new(SmoothUnion(
//         Box::new(*plant), 
//         Box::new(*wdrop),
//         0.1,
//     ));

//     let mut objects = Box::new(Union(
//         Box::new(*plant), 
//         Box::new(*plane)
//     ));


//     //Bug
//     let mut bug_sphere = Box::new(Sphere(Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 0.9, 1, 1.0, 1.0));
//     bug_sphere = Box::new(Scale(Box::new(*bug_sphere), 0.5));
//     let mut bug_cutter1 = Box::new(Cube(Vec3 { x:2.0, y:2.0, z:2.0 }, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 1.0, 1, 1.0, 1.0));
//     bug_cutter1 = Box::new(Move(Box::new(*bug_cutter1), Vec3 { x: 0.0, y: 2.0, z: 0.0 }));
//     let bug_cutter2 = Box::new(Cube(Vec3 { x:0.1, y:2.0, z:2.0 }, Vec3 { x: 1.0, y: 0.0, z: 0.0 }, 1.0, 1, 1.0, 1.0));

//     let mut bug = Box::new(Cut(
//         Box::new(*bug_sphere),
//         Box::new(*bug_cutter1),
//     ));

//     bug = Box::new(Cut(
//         Box::new(*bug),
//         Box::new(*bug_cutter2),
//     ));

//     let bug_leg = Box::new(Line(Vec3 {x:-0.6, y:0.0, z:0.0}, Vec3{x:0.6, y:0.0, z:0.0}, 0.06, Vec3{x:0.0, y:0.0, z:0.0}, 1.0, 1, 1.0, 1.0));
//     let bug_legs = Box::new(Union(
//         Box::new(*bug_leg.clone()),
//         Box::new(Union(
//             Box::new(RotateY(Box::new(*bug_leg.clone()), PI/4.0)),
//             Box::new(RotateY(Box::new(*bug_leg.clone()), -PI/4.0))
//     ))));

//     bug = Box::new(Move(Box::new(*bug),Vec3 {x:1.0, y:0.0, z:1.0}));
//     bug = Box::new(Texturize(Box::new(*bug), tex4, Vec3{x:1.0, y:0.0, z:0.0}*tex4_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex4_scale));
//     bug = Box::new(Move(Box::new(*bug),Vec3 {x:-1.0, y:0.0, z:-1.0}));

//     let bug_eye1 = Box::new(Line(Vec3 {x:-0.2, y:0.0, z:0.0}, Vec3{x:-0.2, y:-0.2, z:0.6}, 0.02, Vec3{x:0.0, y:0.0, z:0.0}, 1.0, 1, 1.0, 1.0));
//     let bug_eye2 = Box::new(Line(Vec3 {x:0.2, y:0.0, z:0.0}, Vec3{x:0.2, y:-0.2, z:0.6}, 0.02, Vec3{x:0.0, y:0.0, z:0.0}, 1.0, 1, 1.0, 1.0));
    
//     let bug_eyes = Box::new(Union(
//         Box::new(*bug_eye1),
//         Box::new(*bug_eye2)
//     ));

//     bug = Box::new(Union(
//         Box::new(*bug_legs),
//         Box::new(Union(
//             Box::new(*bug_eyes),
//             Box::new(*bug)
//     ))));

//     bug = Box::new(RotateX(Box::new(*bug), -PI/10.));
//     bug = Box::new(RotateZ(Box::new(*bug), -PI/8.));
//     bug = Box::new(Move(Box::new(*bug),Vec3 {x:0.0, y:-1.1, z:2.2}));
    

//     //Assembly
//     let objects = Box::new(Union(
//         Box::new(*objects),
//         Box::new(*bug)
//     ));
//     let objects = Box::new(Move(Box::new(*objects), Vec3 {x:5.0, y:1.5, z:-2.0 }));
    
//     //Background
//     let mut background = Box::new(Cube(Vec3 { x: 1.0, y: 1000.0, z: 1000.0 }, Vec3 { x: 0.0, y: 0.0, z: 0.0 }, 1.0, 1, 1.0, 1.0));
//     background = Box::new(Texturize(Box::new(*background), tex3, Vec3{x:0.0, y:0.0, z:1.0}*tex3_scale, Vec3{x:0.0, y:1.0, z:0.0}*tex3_scale));
//     background = Box::new(Move(Box::new(*background), Vec3 {x:50.0, y:115.0, z:100.0}));
    

//     //Final Assembly
//     let scene = Box::new(Union(
//         Box::new(*background),
//         Box::new(*objects)
//     ));

//     //let objects = Box::new(Move(Box::new(*bug), Vec3 {x:5.0, y:0.0, z:0.0 }));
//     return Box::new(*scene)
// }



// // //Cornell box
// pub fn scene() -> Box<Op> {


//     let sphere_att = ObjectData{
//         color: Vec3{x:1.0, y:1.0, z:1.0},
//         reflectance: 1.0,
//         surface_model: 3,
//         emission_rate: 0.0,
//         refractive_index: 1.5,
//     };
    
//     let sphere_att_2 = ObjectData{
//         color: Vec3{x:1.0, y:1.0, z:1.0},
//         reflectance: 1.0,
//         surface_model: 2,
//         emission_rate: 0.0,
//         refractive_index: 1.5,
//     };

//     let sphere_att_3 = ObjectData{
//         color: Vec3{x:1.0, y:1.0, z:1.0},
//         reflectance: 1.0,
//         surface_model: 3,
//         emission_rate: 0.0,
//         refractive_index: 1.5,
//     };

//     let cube_att_top = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     let cube_att_bot = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     let cube_att_left = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.0, z: 0.0 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     let cube_att_front = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     let cube_att_right = ObjectData { 
//         color: Vec3 { x: 0.0, y: 0.9, z: 0.0 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     let cube_light_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 5.0,
//         refractive_index: 0.0 
//     };

//     let cube_glas_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 3,
//         emission_rate: 0.0,
//         refractive_index: 1.5 
//     };

//     let cube_side = 1.5;
//     let light_side = 1.0;
//     let mut cube1 = Box::new(Cube(Vec3{x:cube_side, y:cube_side, z:cube_side}, cube_att_front));
//     let mut cube2 = Box::new(Cube(Vec3{x:cube_side, y:cube_side, z:cube_side}, cube_att_bot));
//     let mut cube3 = Box::new(Cube(Vec3{x:cube_side, y:cube_side, z:cube_side}, cube_att_top));
//     let mut cube4 = Box::new(Cube(Vec3{x:cube_side, y:cube_side, z:cube_side}, cube_att_right));
//     let mut cube5 = Box::new(Cube(Vec3{x:cube_side, y:cube_side, z:cube_side}, cube_att_left));
//     let mut cube_light = Box::new(Cube(Vec3{x:light_side, y:cube_side, z:light_side}, cube_light_att));
//     //let mut torus = Box::new(Torus(Vec2 { x:0.7, y:0.4}, Vec3 { x: 0.9, y: 0.9, z: 0.0 }, 1.0, 2, 0.0, 1.5));
//     let mut sphere = Box::new(Sphere(sphere_att));
//     let mut sphere2 = Box::new(Sphere(sphere_att_2));
//     //let mut sphere3 = Box::new(Sphere(sphere_att_3));
//     let mut cube_glas = Box::new(Cube(Vec3{x:cube_side/3.0, y:cube_side/3.0, z:cube_side/3.0}, cube_glas_att));



//     cube1 = Box::new(Move(Box::new(*cube1), Vec3 {x:cube_side*2.0, y:0.0, z:0.0}));
//     cube2 = Box::new(Move(Box::new(*cube2), Vec3 {x:0.0, y:cube_side*2.0, z:0.0}));
//     cube3 = Box::new(Move(Box::new(*cube3), Vec3 {x:0.0, y:-cube_side*2.0, z:0.0}));
//     cube4 = Box::new(Move(Box::new(*cube4), Vec3 {x:0.0, y:0.0, z:cube_side*2.0}));
//     cube5 = Box::new(Move(Box::new(*cube5), Vec3 {x:0.0, y:0.0, z:-cube_side*2.0}));
//     cube_light = Box::new(Move(Box::new(*cube_light), Vec3 {x:0.0, y:-cube_side*2.0+0.1, z:0.0}));

//     cube_glas = Box::new(RotateZ(Box::new(*cube_glas), PI/1.8));
//     cube_glas = Box::new(RotateY(Box::new(*cube_glas), PI/3.8));

//     //torus = Box::new(RotateZ(Box::new(*torus), PI/1.8));
//     //torus = Box::new(RotateY(Box::new(*torus), PI/3.8));

//     let sphere_size = cube_side/3.0;
//     //sphere = Box::new(Scale(Box::new(*sphere), sphere_size));
//     //sphere = Box::new(Move(Box::new(*sphere), Vec3{x:-cube_side/3., y:cube_side-sphere_size, z:cube_side/2.0}));
//     sphere2 = Box::new(Scale(Box::new(*sphere2), sphere_size));
//     sphere2 = Box::new(Move(Box::new(*sphere2), Vec3{x:cube_side/3., y:cube_side-sphere_size, z:-cube_side/2.0}));
//     //sphere3 = Box::new(Scale(Box::new(*sphere3), sphere_size));
//     //sphere3 = Box::new(Move(Box::new(*sphere3), Vec3{x:-cube_side/3., y:cube_side-sphere_size, z:-cube_side/2.0}));
//     cube_glas = Box::new(Move(Box::new(*cube_glas), Vec3 {x:0.0, y:0.0, z:0.0})); //cube_side/2.0}));


//     //Assemble
//     let mut cube12 = Box::new(Union(
//         Box::new(*cube1),
//         Box::new(*cube2),
//         )
//     );

//     let mut cube34 = Box::new(Union(
//         Box::new(*cube3),
//         Box::new(*cube4),
//         )
//     );

//     // let mut cube56 = Box::new(Union(
//     //     Box::new(*cube5),
//     //     Box::new(*cube4),
//     //     )
//     // );

//     let mut cube12345 = Box::new(Union(
//         Box::new(*cube12),
//         Box::new(Union(
//             Box::new(*cube34),
//             Box::new(*cube5)
//     ))));

//     let mut objects = Box::new(Union(
//         cube12345,
//         Box::new(Union(
//             sphere2, 
//             Box::new(Union(
//                 cube_light,
//                 cube_glas,
//                 // Box::new(Union(
//                 //     sphere3,
//                 //     ,
//     ))))));

//     objects = Box::new(Move(Box::new(*objects), Vec3 { x:cube_side*2.66, y: 0.0, z: 0.0 }));
//     //objects = Box::new(RotateZ(Box::new(*objects), -PI/4.0));
//     //objects = Box::new(RotateY(Box::new(*objects), PI/4.0));

//     return Box::new(*objects)
// }


// // //Depth of field test
// pub fn scene() -> Box<Op> {

//     let sphere_att = ObjectData{
//         color: Vec3{x:1.0, y:0.1, z:0.1},
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 50.0,
//         refractive_index: 1.5,
//     };

//     let cube_att = ObjectData{
//         color: Vec3{x:1.0, y:1.0, z:1.0},
//         reflectance: 1.0,
//         surface_model: 3,
//         emission_rate: 0.0,
//         refractive_index: 1.5,
//     };

//     let plane_att = ObjectData { 
//         color: Vec3 { x: 0.9, y: 0.9, z: 0.9 },
//         reflectance: 1.0,
//         surface_model: 1,
//         emission_rate: 0.0,
//         refractive_index: 0.0 
//     };

//     //Textures
//     let path1 = r"C:\Users\Erik\Documents\Rust_Scripts\RayMarcher\Textures\tiles.png";
//     let tex1 = image::open(path1).expect("File not found!");
//     let tex1_scale = 1.5;


//     let mut plane = Box::new(Plane(1.0, plane_att));
//     plane = Box::new(Texturize(Box::new(*plane), tex1, Vec3{x:1.0, y:0.0, z:0.0}*tex1_scale, Vec3{x:0.0, y:0.0, z:1.0}*tex1_scale));

//     let mut sphere = Box::new(Sphere(sphere_att));
//     sphere = Box::new(Scale(Box::new(*sphere), 0.5));
//     sphere = Box::new(Move(Box::new(*sphere), Vec3{x:3.0, y:0.5, z:0.0}));
   
//     let mut cube: Box<Op> = Box::new(Cube(Vec3 {x:1.0, y:1.0, z:1.0}, cube_att));//Box::new(Cube(Vec3 {x:1.0, y:1.0, z:1.0 }, cube_att));
//     cube = Box::new(Scale(Box::new(*cube), 0.5));
//     //cube = Box::new(RotateY(Box::new(*cube), PI/4.0));
//     cube = Box::new(Move(Box::new(*cube), Vec3{x:3.0, y:0.5, z:1.5}));
   
//     // let mut prism = Box::new(Prism(Vec2 {x:1.0 ,y:1.0}, cube_att));
//     // prism = Box::new(RotateX(Box::new(*prism), PI));
//     // prism = Box::new(RotateY(Box::new(*prism), PI/3.0));
//     // prism = Box::new(Scale(Box::new(*prism), 0.5));
//     // prism = Box::new(Move(Box::new(*prism), Vec3{x:3.0, y:0.5, z:0.3}));
   

//     //Assemble
//     let objects = Box::new(Union(
//         sphere,
//         Box::new(Union(
//             cube,
//             plane
//     ))));

//     return Box::new(*objects)
// }




//Frac2 test
pub fn scene() -> Box<Op> {

    let frac2_att = ObjectData{
        color: Vec3{x:0.8, y:0.8, z:0.8},
        reflectance: 1.0,
        surface_model: 1,
        emission_rate: 50.0,
        refractive_index: 1.5,
    };

    let mut frac2 = Box::new(Frac2(frac2_att));
    frac2 = Box::new(RotateY(Box::new(*frac2), -PI/4.0));
    frac2 = Box::new(RotateX(Box::new(*frac2), PI/4.0));
    frac2 = Box::new(Move(Box::new(*frac2), Vec3 { x:2.0, y:0.0, z:0.0}));

    return Box::new(*frac2)
}



