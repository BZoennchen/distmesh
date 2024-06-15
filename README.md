# DistMesh-rs

A Rust library for the generaiton of high-quality unstructured 2D meshes.
A port of [``DistMesh``](http://persson.berkeley.edu/distmesh/) by Persson and Strang with improvements introduced by [myself](https://mediatum.ub.tum.de/1593965?style=full_standard) under the name [EikMesh](https://www.sciencedirect.com/science/article/pii/S1877750318303193).

## Documentation

## Example

```rust
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
        println!("step {}, quality: {}", (i+1), quality(
          &distmesh.points, 
          &distmesh.triangulation.triangles)
        );
    }
}
```

## Performance

TODO