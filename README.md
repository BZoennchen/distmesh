# DistMesh-rs

A Rust library for the generaiton of high-quality unstructured 2D meshes.
A port of [``DistMesh``](http://persson.berkeley.edu/distmesh/) by Persson and Strang with improvements introduced by [myself](https://mediatum.ub.tum.de/1593965?style=full_standard) under the name [EikMesh](https://www.sciencedirect.com/science/article/pii/S1877750318303193).

## Documentation

## Example

```rust
use delaunator::{Point, triangulate};

let points = vec![
    Point { x: 0., y: 0. },
    Point { x: 1., y: 0. },
    Point { x: 1., y: 1. },
    Point { x: 0., y: 1. },
];

let result = triangulate(&points);

println!("{:?}", result.triangles); // [0, 2, 1, 0, 3, 2]
```

## Performance

TODO