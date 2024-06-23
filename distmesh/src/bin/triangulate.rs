use delaunator::{Point, triangulate};

fn main() {
    let points = vec![
        Point { x: 0., y: 0. },
        Point { x: 1., y: 0. },
        Point { x: 2., y: 1. },
        Point { x: 0., y: 1. },
    ];
    
    let result = triangulate(&points);
    println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
    println!("{:?}", result.halfedges);
    println!("{:?}", result.hull); 
}