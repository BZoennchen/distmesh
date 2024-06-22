use core::{cmp::Ordering, f64, fmt};
use robust::orient2d;
use crate::mesh::mesh::{EMPTY, Mesh};

pub const EPSILON: f64 = f64::EPSILON * 2.0;

fn flip(mesh: &mut Mesh, halfedge: usize) {

  // if the pair of triangles doesn't satisfy the Delaunay condition
  // (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
  // then do the same check/flip recursively for the new pair of triangles
  //
  //           va0                         va0
  //          /||\                        /  \
  //       a1/ || \b2                  a1/    \b2
  //        /  ||  \                   /      \
  //       / a0||b0 \       flip      /___b0___\
  //    va1\   ||    /vb1    =>    va1\---a0---/vb1
  //        \  ||   /                \       /
  //       a2\ ||  /b1               a2\     /b1
  //          \||/                      \  /
  //          vb0                        vb0
  //

  // 1. gather all the references required
  let a0 = halfedge;
  let a1 = mesh.next(a0);
  let a2 = mesh.next(a1);

  let b0 = mesh.twin(halfedge);
  let b1 = mesh.next(b0);
  let b2 = mesh.next(b1);

  let fa = mesh.face(a0);
  let fb = mesh.face(b0);

  let va1 = mesh.vertex(a1);
  let vb1 = mesh.vertex(b1);

  let va0 = mesh.vertex(a0);
  let vb0 = mesh.vertex(b0);

  if mesh.edge_of_face(fb) == b1 {
    mesh.set_edge_of_face(fb, a1);
  }

  if mesh.edge_of_face(fa) == a1 {
    mesh.set_edge_of_face(fa, b1);
  }

  // TODO: maybe without if, just do it? its faster?
  assert_eq!(mesh.vertex(b2), va0);
  assert_eq!(mesh.vertex(a2), vb0);


  if mesh.edge_of_vertex(va0) == a0 {
    mesh.set_halfedge(va0, b2);
  }

  if mesh.edge_of_vertex(vb0) == b0 {
    mesh.set_halfedge(vb0, a2);
  }

  mesh.set_halfedge(a0, va1);
  mesh.set_vertex(b0, vb1);

  mesh.set_next(a0, a2);
  mesh.set_next(a2, b1);
  mesh.set_next(b1, a0);

  mesh.set_next(b0, b2);
  mesh.set_next(b2, a1);
  mesh.set_next(a1, b0);

  mesh.set_face(a1, fb);
  mesh.set_face(b1, fa);
}

pub fn legalize(mesh: &mut Mesh, edge: usize) {
  if is_illegal(mesh,edge) {
    let twin = mesh.twin(edge);
    let p = mesh.vertex(mesh.next(edge));

    flip(mesh, edge);

    let vertex = mesh.vertex(edge);

    if vertex == p {
      let e1 = mesh.prev(edge);
      let e2 = mesh.next(mesh.twin(twin));
      legalize(mesh, e1);
      legalize(mesh, e2);
    }
    else {
      let e1 = mesh.next(edge);
      let e2 = mesh.prev(mesh.twin(edge));
      legalize(mesh, e1);
      legalize(mesh, e2);
    }
  }
}

pub fn is_illegal(mesh: &Mesh, a: usize) -> bool {
  let b = mesh.twin(a);

  // if the pair of triangles doesn't satisfy the Delaunay condition
  // (p1 is inside the circumcircle of [p0, pl, pr]), flip them,
  // then do the same check/flip recursively for the new pair of triangles
  //
  //           pl                    pl
  //          /||\                  /  \
  //       al/ || \bl            al/    \bl
  //        /  ||  \              /      \
  //       /  a||b  \    flip    /___a___\
  //     p0\   ||   /p1   =>   p0\---b---/p1
  //        \  ||  /              \      /
  //       ar\ || /br             ar\    /br
  //          \||/                  \  /
  //           pr                    pr
  //
  let ar = mesh.prev(a);

  if mesh.is_border(b) {
      return false;
  }

  let al = mesh.next(a);
  let bl = mesh.prev(b);
  let br = mesh.next(b);

  let p0 = mesh.point_of_edge(al);
  let pr = mesh.point_of_edge(ar);
  let pl = mesh.point_of_edge(bl);
  let p1 = mesh.point_of_edge(br);
  // TODO: is the order right?
  let in_circ = p0.in_circle(pr, p1, pl);
  in_circ
}

fn calc_bbox_center(points: &[Point]) -> Point {
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

fn find_closest_point(points: &[Point], p0: &Point) -> Option<usize> {
  let mut min_dist = f64::INFINITY;
  let mut k: usize = 0;
  for (i, p) in points.iter().enumerate() {
      let d = p0.dist2(p);
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

impl Point {
  fn dist2(&self, p: &Self) -> f64 {
      let dx = self.x - p.x;
      let dy = self.y - p.y;
      dx * dx + dy * dy
  }

  /// Returns a **negative** value if ```self```, ```q``` and ```r``` occur in counterclockwise order (```r``` is to the left of the directed line ```self``` --> ```q```)
  /// Returns a **positive** value if they occur in clockwise order(```r``` is to the right of the directed line ```self``` --> ```q```)
  /// Returns zero is they are collinear
  fn orient(&self, q: &Self, r: &Self) -> f64 {
      // robust-rs orients Y-axis upwards, our convention is Y downwards. This means that the interpretation of the result must be flipped
      orient2d(self.into(), q.into(), r.into())
  }

  fn circumdelta(&self, b: &Self, c: &Self) -> (f64, f64) {
      let dx = b.x - self.x;
      let dy = b.y - self.y;
      let ex = c.x - self.x;
      let ey = c.y - self.y;

      let bl = dx * dx + dy * dy;
      let cl = ex * ex + ey * ey;
      let d = 0.5 / (dx * ey - dy * ex);

      let x = (ey * bl - dy * cl) * d;
      let y = (dx * cl - ex * bl) * d;
      (x, y)
  }

  fn circumradius2(&self, b: &Self, c: &Self) -> f64 {
      let (x, y) = self.circumdelta(b, c);
      x * x + y * y
  }

  fn circumcenter(&self, b: &Self, c: &Self) -> Self {
      let (x, y) = self.circumdelta(b, c);
      Self {
          x: self.x + x,
          y: self.y + y,
      }
  }

  pub fn in_circle(&self, b: &Self, c: &Self, p: &Self) -> bool {
      let dx = self.x - p.x;
      let dy = self.y - p.y;
      let ex = b.x - p.x;
      let ey = b.y - p.y;
      let fx = c.x - p.x;
      let fy = c.y - p.y;

      let ap = dx * dx + dy * dy;
      let bp = ex * ex + ey * ey;
      let cp = fx * fx + fy * fy;

      dx * (ey * cp - bp * fy) - dy * (ex * cp - bp * fx) + ap * (ex * fy - ey * fx) < 0.0
  }

  fn nearly_equals(&self, p: &Self) -> bool {
      f64::abs(self.x - p.x) <= EPSILON && f64::abs(self.y - p.y) <= EPSILON
  }
}

// CCW ordered
pub fn equiliteral_triangle(seg_len: f64) -> (Point, Point, Point) {
  let u1 = Point {x: 0.0, y: 0.0};
  let u2 = Point {x: seg_len, y: 0.0};
  let u3 = Point {x: seg_len/2.0, y: seg_len * f64::sqrt(3.0)/2.0};

  (u1, u2, u3)
}

pub fn find_visible_edge(mesh: &Mesh, p: &Point) -> usize {
  for halfedge in mesh.iter_face(mesh.boundary) {
    let u2 = mesh.point_of_edge(halfedge);
    let u1 = mesh.point_of_edge(mesh.twin(halfedge));
    if p.orient(u1, u2) <= 0. {
      return halfedge
    }
  }
  panic!("Unable to find a visible edge for point {p}");
}