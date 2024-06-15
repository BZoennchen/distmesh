use distmesh::prelude::*;

fn main() {
    let npoints = 300;
    let niterations = 1000;

    println!("build distmesh");
    let mut builder = DistMeshBuilder::new(npoints)      
        .x1(-350.0).x2(350.0)
        .y1(-350.0).y2(350.0)
        //.dist_fn(Box::new(Circle::new(Point {x: 0.0, y: 0.0}, 300.0)));
        .dist_fn(Box::new(Rect::new(Point {x: 0.0, y: 0.0}, 300.0, 300.0)));
        //.add_fixpoint(Point {x: 150.0, y: 150.0})
        //.add_fixpoint(Point {x: -150.0, y: 150.0})
        //.add_fixpoint(Point {x: 150.0, y: -150.0})
        //.add_fixpoint(Point {x: -150.0, y: -150.0});

    let m = 20;
    for i in 0..m {
        builder = builder.add_fixpoint(Point {x: -150.0 + (i/(m-1)) as f64 * 300.0, y: -150.0});
        builder = builder.add_fixpoint(Point {x: -150.0 + (i/(m-1)) as f64 * 300.0, y: 150.0});
        builder = builder.add_fixpoint(Point {x: -150.0, y: -150.0 + (i/(m-1)) as f64 * 300.0});
        builder = builder.add_fixpoint(Point {x: 150.0, y: -150.0 + (i/(m-1)) as f64 * 300.0});
    }

    let mut distmesh = builder.build();
    println!("finish building distmesh");

    for i in 0..niterations {
        distmesh.update(DELTA_T);
        println!("step {}, quality: {}", (i+1), quality(&distmesh.points, &distmesh.triangulation.triangles));
    }
}