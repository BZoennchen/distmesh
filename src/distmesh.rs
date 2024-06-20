use std::usize;

use delaunator::{next_halfedge, prev_halfedge, triangulate, Point, Triangulation, EMPTY, EPSILON};
use crate::{dspoint::DSPoint, sfd::SignedDistanceFunction, Rect};
use rand::random;

pub type EdgeLenFn = fn(u: &Point) -> f64;

const OMEGA: f64 = 1.2;
const BREAK_POINT: f64 = 2.0;
pub const DELTA_T: f64 = 0.15;
pub const PUSH_BACK_EPS: f64 = 0.000001;

pub struct BoundigBox {
  x: f64,
  y: f64,
  w: f64,
  h: f64,
}

impl BoundigBox {
  fn to_rect(&self) -> Rect {
    let center_x = self.x - self.w/2.0;
    let center_y = self.y - self.h/2.0;
    Rect::new(Point {x: center_x, y: center_y}, self.w, self.h)
  }
}

pub struct DistMeshBuilder {
  npoints: usize,
  x1: f64,
  y1: f64,
  x2: f64,
  y2: f64,
  fixpoints: Vec<Point>,
  edge_len_fn: Option<EdgeLenFn>,
  dist_fn: Option<Box<dyn SignedDistanceFunction>>,
  smoothing_fn: fn(labmda_k: f64) -> f64,
  use_virtual_edges: bool,
  break_edges: bool, 
}

impl DistMeshBuilder {
  pub fn new(npoints: usize) -> Self {
    let x1 = 0.0;
    let y1 = 0.0;
    let x2 = 1.0;
    let y2 = 1.0;

    DistMeshBuilder {
      npoints, x1, y1, x2, y2, 
      fixpoints: Vec::new(), 
      edge_len_fn: Some(|_: &Point| {1.0}), 
      dist_fn: None,
      smoothing_fn: bosson,
      use_virtual_edges: false,
      break_edges: false,
    }
  }

  pub fn add_fixpoint(mut self, fixpoint: Point) -> Self {
    self.fixpoints.push(fixpoint);
    self
  }

  pub fn bosson(mut self) -> Self {
    self.smoothing_fn = bosson;
    self
  }

  pub fn persson(mut self) -> Self {
    self.smoothing_fn = persson;
    self
  }

  pub fn virtual_edges(mut self) -> Self {
    self.use_virtual_edges = true;
    self
  }

  pub fn break_edges(mut self) -> Self {
    self.break_edges = true;
    self
  }

  pub fn x1(mut self, x1: f64) -> Self {
    self.x1 = x1;
    self
  }

  pub fn y1(mut self, y1: f64) -> Self {
    self.y1 = y1;
    self
  }

  pub fn x2(mut self, x2: f64) -> Self {
    self.x2 = x2;
    self
  }

  pub fn y2(mut self, y2: f64) -> Self {
    self.y2 = y2;
    self
  }

  pub fn dist_fn(mut self, dist_fn: Box<dyn SignedDistanceFunction>) -> Self {
    self.dist_fn = Some(dist_fn);
    self
  }

  pub fn edge_len_fn(mut self, edge_len_fn: EdgeLenFn) -> Self {
    self.edge_len_fn = Some(edge_len_fn);
    self
  }

  pub fn build(self) -> DistMesh {

    assert!(self.x1 < self.x2);
    assert!(self.y1 < self.y2);

    let bbox = BoundigBox {x: self.x1, y: self.y1, w: self.x2-self.x1, h: self.y2-self.y1};
    let dist_fn: Box<dyn SignedDistanceFunction> = self.dist_fn.or(Some(Box::new(bbox.to_rect()))).unwrap();
    
    let mut points: Vec<Point> = distribute_points(self.npoints, &bbox, &dist_fn);
    
    let mut fixpoints: Vec<bool> = Vec::with_capacity(self.fixpoints.len() + points.len());
    for _ in &points {
      fixpoints.push(false);
    }
    
    for point in self.fixpoints {
      points.push(point);
      fixpoints.push(true);
    }

    let triangulation = triangulate(&points);

    //let d: EdgeLenFn = |p: &Point| {1.0 + Rect::new(Point {x: 0.0, y: 0.0}, 500.0, 500.0).distance(p).abs()/500.0};

    DistMesh {
      points: points, 
      triangulation: triangulation, 
      edge_len_fn: self.edge_len_fn.expect("expect valid edge length function"),
      //edge_len_fn: d, 
      dist_fn: dist_fn,
      smoothing_fn: self.smoothing_fn,
      use_virtual_edges: self.use_virtual_edges,
      break_edges: self.break_edges,
      fixpoints: fixpoints,
      update_counter: 0,
    }
  }
}

pub struct DistMesh {
  pub points: Vec<Point>,
  pub triangulation: Triangulation,
  edge_len_fn: EdgeLenFn,
  dist_fn: Box<dyn SignedDistanceFunction>,
  smoothing_fn: fn(lambda_k: f64) -> f64,
  use_virtual_edges: bool,
  break_edges: bool,
  fixpoints: Vec<bool>,
  update_counter: usize,
}

impl DistMesh {

  pub fn new(npoints: usize, bouding_box: BoundigBox, dist_fn: Box<dyn SignedDistanceFunction>) -> Self {
    let points: Vec<Point> = distribute_points(npoints, &bouding_box, &dist_fn);
    let edge_len_fn = |_: &Point| {1.0};
    let triangulation = triangulate(&points);
    DistMesh{ 
      points: points, triangulation, 
      edge_len_fn, dist_fn, 
      smoothing_fn: bosson,
      use_virtual_edges: false,
      break_edges: false,
      fixpoints: Vec::new(), 
      update_counter: 0}
  }

  pub fn update(&mut self, delta: f64) {
    // 1. compute scale value
    let scale = self.compute_scaling();

    if self.break_edges {
      self.break_edges(scale);
    }

    // 2. compute forces
    let forces = self.compute_forces(scale);

    // 3. update forces
    self.update_points(&forces, delta);

    // 4. push back
    self.pushback_points();

    // 5. trangulate
    //if self.update_counter % 20 == 0 {
      //self.collapse_edges(scale);
      self.triangulation = triangulate(&self.points);
    //}
    
    self.update_counter += 1;
  }

  pub fn collapse_edges(&mut self, scale: f64) {
    let len = self.triangulation.hull.len();
    let mut removes: Vec<usize> = Vec::new();
    for i in 0..len {
      let iu = self.triangulation.hull[i];
      let iv = self.triangulation.hull[(i + 1) % len];
      let iw = self.triangulation.hull[(i + 2) % len];

      let u: &Point = &self.points[iu];
      let v: &Point = &self.points[iv];
      let w: &Point = &self.points[iw];

      let uv: Point = u.subtract(v);
      let vw: Point = v.subtract(w);
      let uv_center = u.center(v);
      let vw_center = v.center(w);

      let h_k1: f64 = (self.edge_len_fn)(&uv_center) * scale;
      let h_k2: f64 = (self.edge_len_fn)(&vw_center) * scale;

      if uv.len() / h_k1 < 0.3 && vw.len() / h_k2 < 0.3 {
        removes.push(iv);
      }
    }

    removes.sort();
    removes.reverse();
    for i in 0..removes.len() {
      self.points.remove(i);
      self.fixpoints.remove(i);
    }

  }

  pub fn break_edges(&mut self, scale: f64) {
    let len = self.triangulation.hull.len();
    for i in 0..len {
      let iv = self.triangulation.hull[i];
      let iu = self.triangulation.hull[(i + 1) % len];

      let u: &Point = &self.points[iu];
      let v: &Point = &self.points[iv];
      let uv: Point = u.subtract(v);
      let center = u.center(v);

      let h_k: f64 = (self.edge_len_fn)(&center) * scale;
      let lambda_k: f64 = uv.len() / h_k;
      if lambda_k > BREAK_POINT {
        //center.add_mut(&Point {x: EPSILON, y: EPSILON});
        self.points.push(center);
        self.fixpoints.push(false);
      }
    }
  }

  pub fn is_fixpoint(&self, iu: usize) -> bool {
    self.fixpoints[iu]
  }

  fn pushback_points(&mut self) {
    let mut count = 0;
    for iu in 0..self.points.len() {
      if !self.is_fixpoint(iu) {
        let dist = self.dist_fn.distance(&self.points[iu]);
        if dist > 0.0 {
          let grad = self.dist_fn.grad_with_eps(&self.points[iu], PUSH_BACK_EPS);
          //println!("before point: ({},{})", point.x, point.y);
          self.points[iu].subtract_mut(&grad.mult(dist));
          count += 1;
          //println!("after point: ({},{})", point.x, point.y);
        }
      } else {
        //println!("fixpoint: ({},{})", point.x, point.y);
      }
    }
    //println!("noutside: {}", count);
  }

  fn update_points(&mut self, forces: &Vec<Point>, delta: f64) {
    for iu in 0..forces.len() {
      if !self.is_fixpoint(iu) {
        self.points[iu].add_mut(&forces[iu].mult(delta));
      }
    }
  }

  fn compute_ratio(&self, u: &Point, v: &Point) -> (f64, f64) {
    let dir = u.subtract(v);
  
    let center = u.center(v);
    let len_sq = dir.len_sq();
    let h = (self.edge_len_fn)(&center);
    (len_sq, h*h)
  }
  
  fn compute_force(&self, u: &Point, v: &Point, scale: f64) -> Point {
    let uv: Point = u.subtract(v);
    let normed_dir = uv.norm();
    let center = u.center(v);

    let h_k: f64 = (self.edge_len_fn)(&center) * OMEGA * scale;
    let lambda_k: f64 = uv.len() / h_k;
    let nu_hat: f64 = (self.smoothing_fn)(lambda_k);
    let nu: f64 = nu_hat * h_k;
    
    let force = normed_dir.mult(nu);
    force
  }

  fn compute_scaling(&self) -> f64 {
    let mut sum_h_sq = 0.0;
    let mut sum_len_sq = 0.0;

    for &iedge in &self.triangulation.halfedges {
      if iedge != EMPTY {
        let iu = self.triangulation.triangles[iedge];
        let itwin = self.triangulation.halfedges[iedge];
        let iv = self.triangulation.triangles[itwin];

        let u: &Point = &self.points[iu];
        let v: &Point = &self.points[iv];
        let (len_sq, h_sq) = self.compute_ratio(u, v);
        
        sum_h_sq += h_sq;
        sum_len_sq += len_sq;
      }
    }

    let scale = (sum_len_sq / sum_h_sq).sqrt();

    let len = self.triangulation.hull.len();

    if len < 2 {
      return scale;
    }

    for i in 0..len {
      let iv = self.triangulation.hull[i];
      let iu = self.triangulation.hull[(i + 1) % len];

      let u: &Point = &self.points[iu];
      let v: &Point = &self.points[iv];
      let (len_sq, h_sq) = self.compute_ratio(u, v);
      
      sum_h_sq += h_sq;
      sum_len_sq += len_sq;

      let (len_sq, h_sq) = self.compute_ratio(v, u);
      
      sum_h_sq += h_sq;
      sum_len_sq += len_sq;
    }

    scale
  }

  fn compute_forces(&self, scale: f64) -> Vec<Point> {
    let mut forces: Vec<Point> = Vec::with_capacity(self.points.len());
    for _ in 0..self.points.len() {
      forces.push(Point {x: 0.0, y: 0.0});
    }
    
    for &iedge in &self.triangulation.halfedges {
      if iedge != EMPTY {
        let iu = self.triangulation.triangles[iedge];
        let itwin = self.triangulation.halfedges[iedge];
        let iv = self.triangulation.triangles[itwin];

        let u: &Point = &self.points[iu];
        let v: &Point = &self.points[iv];
        
        // add virtual force
        if self.use_virtual_edges {
          let inext = next_halfedge(iedge);
          if self.triangulation.halfedges[inext] == EMPTY {
            let iprev: usize = prev_halfedge(iedge);
            let iw =  self.triangulation.triangles[iprev];
            let w: &Point = &self.points[iw];
            let virtual_ppoint = v.center(w);
            forces[iu].add_mut(&self.compute_force(u, &virtual_ppoint, scale * f64::sqrt(3.0)/2.0));
          }
        }

        forces[iu].add_mut(&self.compute_force(u, v, scale));
      }
    }
    let len = self.triangulation.hull.len();

    if len < 2 {
      return forces;
    }

    for i in 0..len {
      let iv = self.triangulation.hull[i];
      let iu = self.triangulation.hull[(i + 1) % len];

      let u: &Point = &self.points[iu];
      let v: &Point = &self.points[iv];

      forces[iu].add_mut(&self.compute_force(u, v, scale));
      forces[iv].add_mut(&self.compute_force(v, u, scale));
    }

    forces
  }

  pub fn is_empty(&self, halfedge: usize) -> bool {
    halfedge == EMPTY
  }
}

fn distribute_points(n: usize, bouding_box: &BoundigBox, dist_fn: &Box<dyn SignedDistanceFunction>) -> Vec<Point> {
  let mut points: Vec<Point>= Vec::with_capacity(n);
  let mut count = 0;

  while count < n {
    let candidate = Point { 
      x: bouding_box.x + bouding_box.w/2.0 + random_range(-bouding_box.w/2.0, bouding_box.w/2.0), 
      y: bouding_box.y + bouding_box.h/2.0 + random_range(-bouding_box.h/2.0, bouding_box.h/2.0)
    };

    if dist_fn.distance(&candidate) < 0.0 {
      points.push(candidate);
      count += 1;
    }
  }

  points
}

fn random_range(a: f64, b: f64) -> f64 {
  let d = b-a;
  a + random::<f64>()*d
}

pub fn bosson(lambda_k: f64) -> f64 {
  (1.0-lambda_k.powi(4)) * (-lambda_k.powi(4)).exp()
}

pub fn persson(lambda_k: f64) -> f64 {
  f64::max(1.0 - lambda_k, 0.0)
}