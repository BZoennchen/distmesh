# DistMesh-rs

A Rust library for the generaiton of high-quality unstructured 2D meshes.
This includes two packages:

+ ``meshing``: A ``Rust``implementation of the [half-edge data structure](https://www.flipcode.com/archives/The_Half-Edge_Data_Structure.shtml)
+ ``distmesh``: a port of [``DistMesh``](http://persson.berkeley.edu/distmesh/) by Persson and Strang with improvements introduced by [myself](https://mediatum.ub.tum.de/1593965?style=full_standard) under the name [EikMesh](https://www.sciencedirect.com/science/article/pii/S1877750318303193).

At the current stage ``distmesh`` relies on the ``delaunator`` crate and works independent of ``meshing``.
However, this will change soon if ``meshing`` is ready.

## Documentation

## Distmesh Examples

```rust
use distmesh::prelude::*;

fn main() {
    let npoints = 300;
    let niterations = 3000;

    println!("build distmesh");
    let builder = DistMeshBuilder::new(npoints)
      .x1(-350.0).x2(350.0)
      .y1(-350.0).y2(350.0)
      .dist_fn(Box::new(Circle::new(Point {x: 0.0, y: 0.0}, 300.0)));
    let mut distmesh = builder.build();
    println!("finish building distmesh");

    for i in 0..niterations {
        distmesh.update(DELTA_T);
        println!("step {}, quality: {}", (i+1), quality(
          &distmesh.points, 
          &distmesh.triangulation.triangles)
        );
    }
}
```

## Performance

TODO