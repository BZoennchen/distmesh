use delaunator::{Point, triangulate, Triangulation, EMPTY};
use crate::geometry::DSPoint;

/// Returns the average quality of triangles which is a metric for the quality of a triangular mesh.
///
/// # Arguments
/// 
/// * `points` - The slice of points of the triangulation
/// * `triangles` - Indices of triangles where three consecutive indices form a triangle
/// 
/// # Examples
/// 
/// ```
/// use distmesh::quality::avg_quality;
/// use delaunator::{Point, triangulate};
/// 
/// let points = vec![
///        Point { x: 0., y: 0. },
///        Point { x: 1., y: 0. },
///        Point { x: 1., y: 1. },
///        Point { x: 0., y: 1. },
///    ]; 
///    let result = triangulate(&points);
/// let quality = avg_quality(&points, &result.triangles);
/// assert!(quality > 0.8);
/// ```
pub fn avg_quality(points: &[Point], triangles: &[usize]) -> f64 {
  let ntriagnles = triangles.len() / 3;
  let mut avg_quality = 0.0;

  for i in 0..ntriagnles {
    let index = i * 3;
    let u1 = &points[triangles[index]];
    let u2 = &points[triangles[index+1]];
    let u3 = &points[triangles[index+2]];
    avg_quality += quality(u1, u2, u3);

  }

  avg_quality / ntriagnles as f64
}

/// Returns the quality of a triangle. This measurement is a metric for the quality of a triangular mesh.
///
/// # Arguments
/// 
/// * `u1` - The first point of the triangle
/// * `u2` - The second point of the triangle
/// * `u3` - The third point of the triangle
/// 
/// # Examples
/// 
/// ```
/// use distmesh::quality::quality;
/// use delaunator::{Point};
/// 
/// let u1 = Point {x: 0.0, y:0.0};
/// let u2 = Point {x: 1.0, y:0.0};
/// let u3 = Point {x: 0.5, y:f64::sqrt(3.0)/2.0};
/// assert!(quality(&u1, &u2, &u3) > 0.99);
/// ```
pub fn quality(u1: &Point, u2: &Point, u3: &Point) -> f64 {
    let a = u1.distance(u2);
    let b = u1.distance(u3);
    let c = u2.distance(u3);
    ((b + c - a) * (c + a - b) * (a + b - c)) / (a * b * c)
}