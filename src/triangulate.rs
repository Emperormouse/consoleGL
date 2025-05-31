use crate::point::Point3d;

pub fn triangulate (p1: Point3d, p2: Point3d, p3: Point3d,
    p4: Point3d, d1: f32, d2: f32, d3: f32, d4: f32) -> Point3d {

    let (x1, y1, z1) = (p1.x, p1.y, p1.z);
    let (x2, y2, z2) = (p2.x, p2.y, p2.z);
    let (x3, y3, z3) = (p3.x, p3.y, p3.z);
    let (x4, y4, z4) = (p4.x, p4.y, p4.z);

    let z = ( ((d1*d1 - d2*d2) * (x3-x1) - (d1*d1 - d3*d3) * (x2 - x1) -  (x1*x1 - x2*x2 + y1*y1 - y2*y2 + z1*z1 - z2*z2) * (x3-x1) + (x1*x1 - x3*x3 + y1*y1 - y3*y3 + z1*z1 - z3*z3) * (x2-x1)) / ((y2-y1)*(x3-x1) - (y3-y1)*(x2-x1)) - ((d1*d1 - d4*d4) * (x3-x1) - (d1*d1 - d3*d3) * (x4 - x1) -  (x1*x1 - x4*x4 + y1*y1 - y4*y4 + z1*z1 - z4*z4) * (x3-x1) + (x1*x1 - x3*x3 + y1*y1 - y3*y3 + z1*z1 - z3*z3) * (x4-x1)) / ((y4-y1)*(x3-x1) - (y3-y1)*(x4-x1)) ) / (((z4 - z1)*(x3-x1) + (z3 - z1)*(x4-x1)) / ((y4-y1)*(x3-x1) - (y3-y1)*(x4-x1))  - ((z2 - z1)*(x3-x1) + (z3 - z1)*(x2-x1)) / ((y2-y1)*(x3-x1) - (y3-y1)*(x2-x1)));
    let y = ( (d1*d1 - d2*d2) * (x3-x1) - (d1*d1 - d3*d3)*(x2-x1) - (x1*x1 - x2*x2 + y1*y1 - y2*y2 + z*(z2-z1) + z1*z1 - z2*z2)*(x3-x1) + (x1*x1 - x3*x3 + y1*y1 - y3*y3 + z*(z3-z1) + z1*z1 - z3*z3)*(x2-x1) ) / ( (y2-y1)*(x3-x1) - (y3-y1)*(x2-x1) );
    let x = (d1*d1 - d2*d2 - (x1*x1 - x2*x2 + y*(y2-y1) + y1*y1 - y2*y2 + z*(z2-z1) + z1*z1 - z2*z2))/(x2-x1);
    
    return Point3d {
        x: x/2.0,
        y: y/2.0,
        z: z/2.0,
    };
}

