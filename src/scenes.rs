
use crate::operations::Op;
use crate::Op::Move;
use crate::Op::Scale;
use crate::Op::Cut;
use crate::Op::Union;
use crate::Op::SmoothUnion;
use crate::Op::SinDistortHeight;
use crate::Op::Sphere;
use crate::Op::Cube;
use crate::Op::CappedCone;
use crate::Op::Ellipsoid;
use crate::Op::Line;
use crate::Op::RotateY;
use crate::Op::InfRep;
use crate::Op::MirrorZ;

use core::f32::consts::PI;

use crate::vec::Vec3;


//Surface models:
//lambertian = 1
//mirror = 2

//room scene
pub fn scene_1() -> Box<Op> {
    let mut room = Cut(
        Box::new(Cube(Vec3{x:10.0, y:1.0, z:1.0}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0)),
        Box::new(Move(Box::new(Cube(Vec3{x:4.0, y:0.9, z:0.9}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:0.0, z:0.0}))
    );

    // let room_with_window = Cut(
    //     Box::new(room.clone()),
    //     Box::new(Move(Box::new(Cube(Vec3{x:0.3, y:0.3, z:0.3}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:-0.3, z:-0.9}))
    // );

    let cube_stack = Union(
        Box::new(Move(Box::new(RotateY(Box::new(Cube(Vec3{x:0.2, y:0.2, z:0.2}, Vec3{x:255.0, y:0.0, z:0.0}, 1.0, 1, 0.0)),1.0)), Vec3{x:-0.5, y:0.7, z:-0.3})),
        Box::new(Move(Box::new(RotateY(Box::new(Cube(Vec3{x:0.1, y:0.1, z:0.1}, Vec3{x:0.0, y:255.0, z:0.0}, 1.0, 2, 0.0)),0.5)), Vec3{x:-0.5, y:0.4, z:-0.3})), 
    );

    let sphere_mirror = Box::new(Move(Box::new(Scale(Box::new(Sphere(Vec3{x:0.0, y:0.0, z:255.0}, 1.0, 1, 0.0)),0.2)), Vec3{x:-0.4, y:0.7, z:0.4}));

    let room_and_objs = Union(
       Box::new(room.clone()),
       Box::new(Union(
            Box::new(cube_stack.clone()),
            Box::new(*sphere_mirror.clone()),
       ))
    );

    let room_and_sun = Union(
        Box::new(room_and_objs.clone()),
        Box::new(Move(Box::new(Scale(Box::new(Sphere(Vec3{x:253.0, y:251.0, z:211.0}, 1.0, 1, 20.0)),0.2)), Vec3{x:0.0, y:-0.9, z:0.0})),
    );

    let objects = Box::new(Move(Box::new(room_and_sun), Vec3{x:3.0, y:0.0, z:0.0}));
    return objects
}

//Infballs scene
pub fn scene_2() -> Box<Op> {
    let mut objects = Box::new(Sphere(Vec3{x:255.0, y:255.0, z:255.0}, 0.9, 2, 0.0));
    objects = Box::new(InfRep(Box::new(*objects), Vec3{x:5.0, y:5.0, z:5.0}));
    objects = Box::new(Move(Box::new(*objects), Vec3{x:5., y:-102.5, z:-102.5}));
    return objects
}

//Menger Sponge
pub fn scene() -> Box<Op> {
    let mut cross_0 = Union(
        Box::new(Union(
            Box::new(Cube(Vec3{x:10000.0,y:1.,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0)),
            Box::new(Cube(Vec3{x:1.,y:10000.0,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0))
        )),
        Box::new(Cube(Vec3{x:1.,y:1.,z:10000.0}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0))
    );

    let layer_0 = Box::new(Move(Box::new(Cube(Vec3{x:1.,y:1.,z:1.}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0)),Vec3{x:2.0,y:2.0,z:2.0}));
    let cross_1 = Box::new(Scale(Box::new(cross_0.clone()),0.333));
    let layer_1 = Cut(
        Box::new(*layer_0.clone()),
        //Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0/(3.*i as f32), y:2.0/(3.*i as f32), z:2.0/(3.*i as f32)}))
        Box::new(InfRep(Box::new(*cross_1.clone()), Vec3{x:2.0, y:2.0, z:2.0}))
    );
    let cross_2 = Box::new(Scale(Box::new(*cross_1.clone()),0.333));
    let mut layer_2 = Cut(
        Box::new(layer_1.clone()),
        Box::new(InfRep(Box::new(*cross_2.clone()), Vec3{x:0.666, y:0.666, z:0.666}))
    );
    let cross_3 = Box::new(Scale(Box::new(*cross_2.clone()),0.333));
    let mut layer_3 = Cut(
        Box::new(layer_2.clone()),
        Box::new(InfRep(Box::new(*cross_3.clone()), Vec3{x:0.222, y:0.222, z:0.222}))
    );
    let cross_4 = Box::new(Scale(Box::new(*cross_3.clone()),0.333));
    let mut layer_4 = Cut(
        Box::new(layer_3.clone()),
        Box::new(InfRep(Box::new(*cross_4.clone()), Vec3{x:0.074, y:0.074, z:0.074}))
    );
    let cross_5 = Box::new(Scale(Box::new(*cross_4.clone()),0.333));
    let mut layer_5 = Cut(
        Box::new(layer_4.clone()),
        Box::new(InfRep(Box::new(*cross_5.clone()), Vec3{x:0.074/3., y:0.074/3., z:0.074/3.}))
    );

    let mut sphere = Box::new(Sphere(Vec3{x:150.0, y:150.0, z:150.0}, 0.9, 2, 0.0));
    sphere = Box::new(Scale(Box::new(*sphere),0.2));
    sphere = Box::new(Move(Box::new(*sphere),Vec3{x:2.0, y:2.0, z:2.0}));

    let mut sphere2 = Box::new(Sphere(Vec3{x:255.0, y:0.0, z:0.0}, 1., 1, 0.0));
    sphere2 = Box::new(Scale(Box::new(*sphere2),0.02));
    sphere2 = Box::new(Move(Box::new(*sphere2),Vec3{x:1.12, y:2.31, z:2.05}));


    let mut objects = Union(
        Box::new(*sphere.clone()),
        Box::new(Union(
            Box::new(*sphere2.clone()),
            Box::new(layer_5.clone()),
        ))
    );

    //let objects = Box::new(Move(Box::new(objects), Vec3{x:-1.0, y:-2.3, z:-2.0}));
    let objects = Box::new(Move(Box::new(objects), Vec3{x:-1.0, y:-2.3, z:-2.0}));
    //objects = Box::new(RotateY(Box::new(*objects), 0.5));
    return objects
}

//spheres on plane
pub fn scene_4() -> Box<Op> {
    let mut plane = Box::new(Cube(Vec3{x:100.0, y:1.0, z:100.0}, Vec3{x:210.0, y:210.0, z:210.0}, 1.0, 1, 0.0));
    let mut sphere1 = Box::new(Move(Box::new(Sphere(Vec3{x:0.0, y:255.0, z:0.0}, 1.0, 2, 5.0)), Vec3{x:0.0, y:-2.0, z:-1.5}));

    let mut sphere2 = Box::new(Sphere(Vec3{x:150.0, y:150.0, z:150.0}, 1.0, 2, 0.0));
    sphere2 = Box::new(Scale(Box::new(*sphere2),3.));
    sphere2 = Box::new(Move(Box::new(*sphere2),Vec3{x:3.5, y:-4.0, z:2.0}));

    let mut sphere3 = Box::new(Sphere(Vec3{x:255.0, y:0.0, z:0.0}, 1.0, 2, 0.0));
    sphere3 = Box::new(Scale(Box::new(*sphere3),1.));
    sphere3 = Box::new(Move(Box::new(*sphere3),Vec3{x:0.0, y:-2.0, z:2.0}));


    let mut objects = Union(
        Box::new(*sphere3.clone()),
        Box::new(Union(
            Box::new(*sphere2.clone()),
            Box::new(SmoothUnion(
                Box::new(*plane.clone()),
                Box::new(*sphere1.clone()),
                2.0,
            ))
        ))    
    );

    objects = *Box::new(Move(Box::new(objects.clone()), Vec3{x:5.0, y:3.0, z:0.0}));
    return Box::new(objects)
}

//desert
pub fn scene_5() -> Box<Op> {
    let mut plane = Box::new(Cube(Vec3{x:100.0, y:10.0, z:100.0}, Vec3{x:140.0, y:110.0, z:110.0}, 1.0, 1, 0.0));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 3.0, 0.11));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 2.0, 0.51));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 1.0, 1.01));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 0.5, 2.01));
    
    
    
    let objects = Box::new(Move(Box::new(*plane), Vec3{x:3.0, y:13.0, z:0.0}));
    return objects
}

//Sand and cactus scene
pub fn scene_6() -> Box<Op> {
    let mut plane = Box::new(Cube(Vec3{x:100.0, y:10.0, z:100.0}, Vec3{x:140.0, y:110.0, z:110.0}, 1.0, 1, 0.0));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 2.0, 0.11));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 1.0, 0.51));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 0.5, 1.01));
    plane = Box::new(RotateY(Box::new(*plane), 0.5));
    plane = Box::new(SinDistortHeight(Box::new(*plane), 0.25, 2.01));


    let pot1 = Box::new(CappedCone(1.0, 1.5, 1.0, Vec3{x: 173.0, y:80.0, z:73.0}, 1.0, 1, 0.0));
    let pot2 = Box::new(Move(Box::new(CappedCone(1.0, 1.5, 1.0, Vec3{x: 173.0, y:80.0, z:73.0}, 1.0, 1, 0.0)), Vec3{x:0.0, y:-0.8, z:0.0}));

    let pot = Cut(
        Box::new(*pot1),
        Box::new(*pot2)
    );

    let mut cactus_stem = Box::new(Ellipsoid(Vec3{x:1.0, y:3.0, z:1.0}, Vec3{x:0.0, y:255.0, z:0.0}, 1.0, 1, 0.0));
    cactus_stem = Box::new(Move(Box::new(*cactus_stem.clone()),Vec3{x:0.0, y:-2.5, z:0.0}));

    let thorn = Box::new(Line(Vec3{x:0.0, y:0.0, z:-1.2}, Vec3{x:0.0, y:0.0, z:1.2}, 0.05, Vec3{x:0.0, y:0.0, z:0.0}, 1.0, 1, 0.0));
    let thorns_rot1 = Box::new(RotateY(Box::new(*thorn.clone()), PI/3.0));
    let thorns_rot2 = Box::new(RotateY(Box::new(*thorns_rot1.clone()), PI/3.0));

    let thorns_level = Union(
        Box::new(*thorn.clone()),
        Box::new(Union(
            Box::new(*thorns_rot1.clone()),
            Box::new(*thorns_rot2.clone())
        ))
    );

    let mut thorns = Union (
        Box::new(thorns_level.clone()),
        Box::new(Union(
            Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-0.8, z:0.0})),
            Box::new(Union(
                Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-1.6, z:0.0})),
                Box::new(Union(
                    Box::new(Move(Box::new(thorns_level.clone()),Vec3{x:0.0, y:-2.4, z:0.0})),
                    Box::new(Move(Box::new(Scale(Box::new(thorns_level.clone()),0.8)),Vec3{x:0.0, y:-3.2, z:0.0}))
                ))
            ))
        ))
    ); 

    thorns = *Box::new(Move(Box::new(thorns),Vec3{x:0.0, y:-1.4, z:0.0}));

    let cactus = Union(
        Box::new(*cactus_stem.clone()),
        Box::new(thorns.clone())
    );

    let mut pot_and_cactus = Union(
        Box::new(pot.clone()),
        Box::new(cactus.clone())
    );

    pot_and_cactus = *Box::new(Scale(Box::new(pot_and_cactus.clone()), 1.0));

    pot_and_cactus = *Box::new(Move(Box::new(pot_and_cactus.clone()), Vec3{x:3.0, y:-11.0, z:0.0}));

    let total = Union(
        Box::new(pot_and_cactus.clone()),
        Box::new(*plane.clone())
    );

    let objects = Box::new(Move(Box::new(total), Vec3{x:7.0, y:14.0, z:0.0}));

    return objects
}

//street
pub fn scene_7() -> Box<Op> {
    let lamp_height = -4.0;

    let lamp = Box::new(Sphere(Vec3{x:253.0, y:251.0, z:211.0}, 1.0, 1, 5.0));
    let pole = Box::new(CappedCone(-lamp_height, 0.1, 0.1, Vec3{x:150.0, y:150.0, z:150.0}, 1.0, 1, 0.0));
    let lamp_post = Union(
        Box::new(*lamp.clone()),
        Box::new(Move(Box::new(*pole.clone()), Vec3{x:0.0, y:-lamp_height, z:0.0})),
    );

    let pavement = Box::new(Cube(Vec3{x:10.0, y:0.1, z:10.0}, Vec3{x:100.0, y:100.0, z:100.0}, 1.0, 1, 0.0));
    //let line = Box::new(Cube(Vec3{x:2.0, y:0.1, z:0.2}, Vec3{x:255.0, y:255.0, z:0.0}, 1.0, 1, 0.0));

    // let mut lines = Union(
    //     Box::new(Move(Box::new(*line.clone()), Vec3{x:0.0, y:-0.1, z:-4.0})),
    //     Box::new(Move(Box::new(*line.clone()), Vec3{x:5.0, y:-0.1, z:-4.0})),
    // );

    // let mut street = Union(
    //     Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
    //     Box::new(Move(Box::new(lines.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
    // );

    // let wall = Box::new(Cube(Vec3{x:3.0, y:8.0, z:3.0}, Vec3{x:255.0, y:255.0, z:255.0}, 1.0, 1, 0.0));
    // let window = Box::new(Cube(Vec3{x:1.0, y:2.0, z:0.2}, Vec3{x:255.0, y:200.0, z:0.0}, 1.0, 1, 1.5));

    // let building = Union(
    //     Box::new(Move(Box::new(*wall.clone()), Vec3{x:0.0, y:-5.0, z:7.0})),
    //     Box::new(Move(Box::new(*window.clone()), Vec3{x:0.0, y:-2.0, z:4.0})),
    // );


    // let mut scene = Union(
    //     Box::new(Move(Box::new(lamp_post.clone()), Vec3{x:0.0, y:lamp_height, z:0.0})),
    //     Box::new(Union(
    //         Box::new(Move(Box::new(building.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
    //         Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
    //     ))
    // );

    let mut scene = Union(
        Box::new(Move(Box::new(lamp_post.clone()), Vec3{x:0.0, y:lamp_height, z:0.0})),
        Box::new(Move(Box::new(*pavement.clone()), Vec3{x:0.0, y:0.0, z:0.0})),
    );

    scene = *Box::new(InfRep(Box::new(scene),Vec3{x:10.0, y:1000.0, z:1000.0}));

    scene = *Box::new(Move(Box::new(scene.clone()), Vec3{x:5.0, y:1.0, z:4.0}));
    scene = *Box::new(MirrorZ(Box::new(scene.clone())));
    
    return Box::new(scene)
}
