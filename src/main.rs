use distmesh::prelude::*;

fn main() {
    let npoints = 300;
    let niterations = 3000;

    println!("build distmesh");
    let builder = DistMeshBuilder::new(npoints).x1(-400.0).x2(400.0).y1(-400.0).y2(400.0);
    let mut distmesh = builder.build();
    println!("finish building distmesh");

    for i in 0..niterations {
        distmesh.update(DELTA_T);
        println!("step {}, quality: {}", (i+1), quality(&distmesh.points, &distmesh.triangulation.triangles));
    }
}