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
}