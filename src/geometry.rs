use delaunator::Point;

pub trait DSPoint {
  fn subtract(&self, other: &Self) -> Self;
  fn add(&self, other: &Self) -> Self;
  fn add_mut(&mut self, other: &Self);
  fn subtract_mut(&mut self, other: &Self);
  fn mult(&self, scalar: f64) -> Self;
  fn div(&self, scalar: f64) -> Self;
  fn center(&self, other: &Self) -> Self;
  fn len(&self) -> f64;
  fn len_sq(&self) -> f64;
  fn norm(&self) -> Self;
  fn distance(&self, other: &Self) -> f64;
  fn distance_sq(&self, other: &Self) -> f64;
  fn x(&self) -> f64;
  fn y(&self) -> f64;
}

impl DSPoint for Point {
  
  fn distance_sq(&self, other: &Self) -> f64 {
      self.subtract(other).len_sq()
  }

  fn distance(&self, other: &Self) -> f64 {
    self.distance_sq(other).sqrt()
}

  fn norm(&self) -> Self {
      self.div(self.len())
  }

  fn mult(&self, scalar: f64) -> Self {
    Point {x: self.x * scalar, y: self.y * scalar}
  }

  fn div(&self, scalar: f64) -> Self {
    Point {x: self.x / scalar, y: self.y / scalar}
  }

  fn len_sq(&self) -> f64 {
      self.x * self.x + self.y * self.y
  }
  
  fn len(&self) -> f64 {
      self.len_sq().sqrt()
  }

  fn subtract(&self, other: &Self) -> Self {
      Point {x: self.x - other.x, y: self.y - other.y}
  }

  fn subtract_mut(&mut self, other: &Self) {
    self.x -= other.x;
    self.y -= other.y;
  }

  fn add(&self, other: &Self) -> Self {
    Point {x: self.x + other.x, y: self.y + other.y}
  }

  fn add_mut(&mut self, other: &Self) {
    self.x += other.x;
    self.y += other.y;
  }

  fn center(&self, other: &Self) -> Self {
    let du = other.subtract(&self).mult(0.5);
    self.add(&du)
  }
  
  fn x(&self) -> f64 {
        self.x
    }
  
  fn y(&self) -> f64 {
        self.y
    }
}

pub fn signed_area_of_polygon(points: &[&Point]) -> f64 {
  let mut area: f64 = 0.0;
  if points.len() >= 3 {
    for i in 0..points.len() {
      let j = (i+1) % points.len();
      area += points[i].x * points[j].y - points[j].x * points[i].y;
    }
  }
  area / 2.0
}

pub fn signed_area_of_triangle(u: &Point, v: &Point, w: &Point) -> f64 {
  signed_area_of_polygon(&[u, v, w])
}

pub fn polygon_centroid(polygon: &[&Point]) -> Option<Point> {
  let area = signed_area_of_polygon(polygon);
  let mut x: f64 = 0.0;
  let mut y: f64 = 0.0;

  assert!(polygon.len() > 2);
  if polygon.len() <= 2 || area == 0.0 {
    return None;
  }

  for i in 0..polygon.len() {
    let j = (i+1) % polygon.len();

    x += (polygon[i].x + polygon[j].x) * (polygon[i].x * polygon[j].y - polygon[i].y * polygon[j].x);
    y += (polygon[i].y + polygon[j].y) * (polygon[i].x * polygon[j].y - polygon[i].y * polygon[j].x);

  }
  x /= 6.0 * area;
  y /= 6.0 * area;

  Some(Point {x, y})
}

pub fn equiliteral_triangle(seg_len: f64) -> (Point, Point, Point) {
  let u1 = Point {x: 0.0, y: 0.0};
  let u2 = Point {x: seg_len, y: 0.0};
  let u3 = Point {x: seg_len/2.0, y: seg_len * f64::sqrt(3.0)/2.0};

  (u1, u2, u3)
}