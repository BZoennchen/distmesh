use core::{f64, fmt};
use robust::orient2d;

pub const EPSILON: f64 = f64::EPSILON * 2.0;

pub trait DSPoint {
  /// Returns a new point which is equal to ```self``` - ```other```.
  fn subtract(&self, other: &Self) -> Self;

  /// Returns a new point which is equal to ```self``` + ```other```.
  fn add(&self, other: &Self) -> Self;

  /// Moves this point by ```+other```.
  fn add_mut(&mut self, other: &Self);

/// Returns the **squared distance** between ```self``` and ```other```.
/// This is faster than computing its **distance**.
  fn distance_sq(&self, other: &Self) -> f64;

  /// Returns the **distance** between ```self``` and ```other```.
  fn distance(&self, other: &Self) -> f64 {
    self.distance_sq(other).sqrt()
  }

  /// Moves this point by ```-other```.
  fn subtract_mut(&mut self, other: &Self);

  /// Returns a scaled version of this point (interpreted as vector) by multiplying each of its component by ```scalar```.
  fn mult(&self, scalar: f64) -> Self;

  /// Returns a scaled version of this point (interpreted as vector) by dividing each of its component by ```scalar```.
  fn div(&self, scalar: f64) -> Self;

  /// Returns the **center point** between ```self``` and ```other```.
  fn center(&self, other: &Self) -> Self;

  /// Returns the **squared length** of this point interpreted as a vector.
  /// This is faster than computing its **length**.
  fn len_sq(&self) -> f64 {
    self.x() * self.x() + self.y() * self.y()
  }

  /// Returns the **length** of this point interpreted as a vector.
  fn len(&self) -> f64 {
    self.len_sq().sqrt()
  }

  /// Returns a **normed** version (the direction) of this point interpreted as a vector.
  fn norm(&self) -> Self;
  fn x(&self) -> f64;
  fn y(&self) -> f64;

  /// Returns a the vector pointing to the circumcenter of the triangle formed by ```self```, ```b```, and ```c``` with respect to ```self```.
  /// If ```self``` is the origin this is equal to the actual circumcenter.
  fn circumdelta(&self, b: &Self, c: &Self) -> (f64, f64) {
    let dx = b.x() - self.x();
    let dy = b.y() - self.y();
    let ex = c.x() - self.x();
    let ey = c.y() - self.y();

    let bl = dx * dx + dy * dy;
    let cl = ex * ex + ey * ey;
    let d = 0.5 / (dx * ey - dy * ex);

    let x = (ey * bl - dy * cl) * d;
    let y = (dx * cl - ex * bl) * d;
    (x, y)
  }

  /// Returns the **squared distance** of ```self```, ```b```, and ```c``` to their shared circumcenter.
  fn circumradius_sq(&self, b: &Self, c: &Self) -> f64 {
      let (x, y) = self.circumdelta(b, c);
      x * x + y * y
  }

  /// Returns the **circumcenter* of ```self```, ```b```, and ```c```.
  fn circumcenter(&self, b: &Self, c: &Self) -> Self;

  /// Returns **true** if and only if ```self``` is within the circumcircle formed by ```a```, ```b```, and ```c``` assuming they are ordered **counterclockwise**!
  fn in_circle(&self, a: &Self, b: &Self, c: &Self) -> bool {
    let dx = self.x() - c.x();
    let dy = self.y() - c.y();
    let ex = a.x() - c.x();
    let ey = a.y() - c.y();
    let fx = b.x() - c.x();
    let fy = b.y() - c.y();

    let ap = dx * dx + dy * dy;
    let bp = ex * ex + ey * ey;
    let cp = fx * fx + fy * fy;

    dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
  }

  /// Returns **true** if and only if ```self``` and ```p``` are so close that we consider them to be at the same position.
  fn nearly_equals(&self, p: &Self) -> bool {
      f64::abs(self.x() - p.x()) <= EPSILON && f64::abs(self.y() - p.y()) <= EPSILON
  }

  /// Returns a **positive** value if ```self```, ```q``` and ```r``` occur in counterclockwise order 
  /// (```r``` is to the left of the directed line ```self``` --> ```q```)
  /// Returns a **negative** value if they occur in clockwise order(```r``` is to the right of the directed line ```self``` --> ```q```)
  /// Returns zero is they are collinear
  fn orient(&self, q: &Self, r: &Self) -> f64;

  /// Returns a **true** value if ```self```, ```q``` and ```r``` occur in counterclockwise order 
  /// (```r``` is to the left of the directed line ```self``` --> ```q```)
  /// Returns a **false** value if they occur in clockwise order(```r``` is to the right of the directed line ```self``` --> ```q```)
  /// Returns zero is they are collinear
  fn ccw(&self, q: &Self, r: &Self) -> bool;
}

#[derive(Clone, PartialEq, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl From<&Point> for robust::Coord<f64> {
    fn from(p: &Point) -> robust::Coord<f64> {
        robust::Coord::<f64> { x: p.x, y: p.y }
    }
}

impl DSPoint for Point {
  fn norm(&self) -> Self {
    self.div(self.len())
  }

  fn mult(&self, scalar: f64) -> Self {
    Self {x: self.x * scalar, y: self.y * scalar}
  }

  fn div(&self, scalar: f64) -> Self {
    Self {x: self.x / scalar, y: self.y / scalar}
  }

  fn subtract(&self, other: &Self) -> Self {
    Self {x: self.x - other.x, y: self.y - other.y}
  }

  fn subtract_mut(&mut self, other: &Self) {
    self.x -= other.x;
    self.y -= other.y;
  }

  fn add(&self, other: &Self) -> Self {
    Self {x: self.x + other.x, y: self.y + other.y}
  }

  fn add_mut(&mut self, other: &Self) {
    self.x += other.x;
    self.y += other.y;
  }

  fn center(&self, other: &Self) -> Self {
    let du = other.subtract(&self).mult(0.5);
    self.add(&du)
  }

  fn distance_sq(&self, other: &Self) -> f64 {
    self.subtract(other).len_sq()
  }

  fn x(&self) -> f64 {
        self.x
    }

  fn y(&self) -> f64 {
    self.y
  }

  fn circumcenter(&self, b: &Self, c: &Self) -> Self {
      let (x, y) = self.circumdelta(b, c);
      Self {
          x: self.x + x,
          y: self.y + y,
      }
  }

  fn orient(&self, q: &Self, r: &Self) -> f64 {
    orient2d(self.into(), q.into(), r.into())
  }

  fn ccw(&self, q: &Self, r: &Self) -> bool {
    orient2d(self.into(), q.into(), r.into()) > 0.0
  } 
}

impl Point {}

/// Returns the **center point** of a bounding box containing all ```points```.
pub fn calc_bbox_center(points: &[Point]) -> Point {
  let mut min_x = f64::INFINITY;
  let mut min_y = f64::INFINITY;
  let mut max_x = f64::NEG_INFINITY;
  let mut max_y = f64::NEG_INFINITY;
  for p in points.iter() {
      min_x = min_x.min(p.x);
      min_y = min_y.min(p.y);
      max_x = max_x.max(p.x);
      max_y = max_y.max(p.y);
  }
  Point {
      x: (min_x + max_x) / 2.0,
      y: (min_y + max_y) / 2.0,
  }
}

/// Returns the **index** of the **closest point** within ```points``` with respect to ```p0```.
pub fn find_closest_point(points: &[Point], p0: &Point) -> Option<usize> {
  let mut min_dist = f64::INFINITY;
  let mut k: usize = 0;
  for (i, p) in points.iter().enumerate() {
      let d = p0.distance_sq(p);
      if d > 0.0 && d < min_dist {
          k = i;
          min_dist = d;
      }
  }
  if min_dist == f64::INFINITY {
      None
  } else {
      Some(k)
  }
}

  /// Returns a **equiliteral** triangle in **counterclockwise** order.
  /// ```seg_len``` determines the side length of the triangle.
pub fn equiliteral_triangle(seg_len: f64) -> (Point, Point, Point) {
  let u1 = Point {x: 0.0, y: 0.0};
  let u2 = Point {x: seg_len, y: 0.0};
  let u3 = Point {x: seg_len/2.0, y: seg_len * f64::sqrt(3.0)/2.0};
  (u1, u2, u3)
}

/// Returns the average quality of triangles which is a metric for the quality of a triangular mesh.
///
/// # Arguments
/// 
/// * `points` - The slice of points of the triangulation
/// * `triangles` - Indices of triangles where three consecutive indices form a triangle
/// 
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
/// use meshing::geometry::{quality,Point};
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

#[cfg(test)]
mod tests {
  const TEST_EPSILON: f64 = 0.0000001;
  use super::*;

  mod test_geometry {
    use super::*;

    #[test]
    fn test_max_quality() {
      let u1 = Point {x: 0.0, y:0.0};
      let u2 = Point {x: 1.0, y:0.0};
      let u3 = Point {x: 0.5, y:f64::sqrt(3.0)/2.0};
      assert!(quality(&u1, &u2, &u3) > 0.99);
    }

    #[test]
    fn test_low_quality() {
      let u1 = Point {x: 0.0, y:0.0};
      let u2 = Point {x: 1.0, y:0.0};
      let u3 = Point {x: 0.5, y: 0.1};
      assert!(quality(&u1, &u2, &u3) < 0.1);
    }

    #[test]
    fn test_equiliteral_triangle() {
      let len = 1.0_f64;
      let (u1, u2, u3) = equiliteral_triangle(len);
      assert!(u1.ccw(&u2, &u3));
      assert!(u1.x().abs() < TEST_EPSILON);
      assert!(u1.y().abs() < TEST_EPSILON);
      assert!((u2.x()-len).abs() < TEST_EPSILON);
      assert!(u2.y().abs() < TEST_EPSILON);
      assert!((u3.x()-len/2.0).abs() < TEST_EPSILON);
      assert!(u3.y()-len < 0.0);
    }

    #[test]
    fn test_calc_bbox_center() {
      assert!(calc_bbox_center(&vec![Point{x:0.0, y:0.0}]).nearly_equals(&Point {x: 0.0, y:0.0}));
      assert!(calc_bbox_center(&vec![Point{x:0.0, y:0.0}, Point{x:1.0, y:1.0}]).nearly_equals(&Point {x: 0.5, y:0.5}));
      assert!(calc_bbox_center(&vec![Point{x:0.0, y:0.0}, Point{x:1.0, y:1.0}, Point{x:0.0, y:1.0}]).nearly_equals(&Point {x: 0.5, y:0.5}));
    }

    #[test]
    fn test_find_closest_point() {
      assert!(find_closest_point(&vec![Point{x:0.0, y:0.0}], &Point{x:1.0, y:1.0}).unwrap() == 0);
      assert!(find_closest_point(&vec![Point{x:0.0, y:0.0}, Point{x:0.5, y:0.5}], &Point{x:1.0, y:1.0}).unwrap() == 1);
      assert!(find_closest_point(&vec![], &Point{x:1.0, y:1.0}) == None);
    }
  }

  mod test_point {
    use super::*;

    #[test]
    fn test_orientation(){
      let u1 = Point {x: 0.0, y: 0.0};
      let q = Point {x: 1.0, y: 0.0};
      
      let r1 = Point {x: 0.0, y: 1.0};
      let r2 = Point {x: 0.0, y: -1.0};
      let r3 = Point {x: 5.0, y: 0.0};

      assert!(u1.orient(&q, &r1) > 0.0);
      assert!(u1.ccw(&q, &r1) == true);

      assert!(u1.orient(&q, &r2) < 0.0);
      assert!(u1.ccw(&q, &r2) == false);

      // edge case => colinear
      assert!(u1.orient(&q, &r3) == 0.0);
      assert!(u1.ccw(&q, &r3) == false);
    }

    #[test]
    fn test_nearly_eqauls(){
      let u1 = Point {x: 0.0, y: 0.0};
      let u2 = Point {x: 1e-16, y: 0.0};
      assert!(u2.nearly_equals(&u1));
      assert!(u2.nearly_equals(&u2));
    }

    #[test]
    fn test_circumdelta(){
      let u1 = Point {x: 0.0, y: 0.0};
      let u2 = Point {x: 1.0, y: 0.0};
      let u3 = Point {x: 0.0, y: 1.0};

      let (x, y) = u1.circumdelta(&u2, &u3);
      assert!((x-0.5).abs() < TEST_EPSILON);
      assert!((y-0.5).abs() < TEST_EPSILON);
    }

    #[test]
    fn test_circumcenter(){
      let u1 = Point {x: 0.0, y: 0.0};
      let u2 = Point {x: 1.0, y: 0.0};
      let u3 = Point {x: 0.0, y: 1.0};

      let ccenter = u1.circumcenter(&u2, &u3);
      assert!(ccenter.nearly_equals(&Point {x:0.5, y:0.5}));
    }

    #[test]
    fn test_in_circle() {
      let u1 = Point {x: 0.0, y: 0.0};
      let u2 = Point {x: 1.0, y: 0.0};
      let u3 = Point {x: 0.0, y: 1.0};
      assert!(u1.ccw(&u2, &u3));

      assert!(Point{x: EPSILON, y: EPSILON}.in_circle(&u1, &u2, &u3) == true);
      assert!(Point{x: -EPSILON, y: -EPSILON}.in_circle(&u1, &u2, &u3) == false);
      assert!(Point{x: 10.0, y: 10.0}.in_circle(&u1, &u2, &u3) == false);
      assert!(Point{x: 0.5, y: 2.0}.in_circle(&u1, &u2, &u3) == false);
      assert!(Point{x: -0.5, y: -2.0}.in_circle(&u1, &u2, &u3) == false);
      assert!(Point{x: 0.5, y: 1.1}.in_circle(&u1, &u2, &u3) == true);
    }
  }
}