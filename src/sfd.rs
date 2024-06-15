use delaunator::Point;
use crate::dspoint::DSPoint;

const EPSILON: f64 = 0.0001;

pub trait SignedDistanceFunction {
    fn distance(&self, point: &Point) -> f64;

    fn grad_with_eps(&self, point: &Point, epsilon: f64) -> Point {
        let hx: Point = Point {x: epsilon, y: 0.0};
        let hy: Point = Point {x: 0.0, y: epsilon};
        let dist = self.distance(point);
        let dx = (self.distance(&point.add(&hx)) - dist)/epsilon;
        let dy = (self.distance(&point.add(&hy)) - dist)/epsilon;
        Point {x: dx, y: dy}
    }

    fn grad(&self, point: &Point) -> Point {
        self.grad_with_eps(point, EPSILON)
    }
}

pub struct Rect {
    pub center: Point,
    pub width: f64,
    pub height: f64,
}

impl Rect {
    pub fn new(center: Point, width: f64, height: f64) -> Self {
        Rect { center, width, height }
    }
}

impl SignedDistanceFunction for Rect {
    fn distance(&self, point: &Point) -> f64 {
        let x_dist = f64::abs(self.center.x-point.x) - self.width/2.0;
        let y_dist = f64::abs(self.center.y-point.y) - self.height/2.0;
        f64::max(x_dist, y_dist)
    }
}

pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(center: Point, radius: f64) -> Self {
        Circle { center, radius }
    }
}

impl SignedDistanceFunction for Circle {
    fn distance(&self, point: &Point) -> f64 {
        self.center.distance(point) - self.radius
    }
}

pub struct SDFUnion {
    pub sdfs: Vec<Box<dyn SignedDistanceFunction>>,
}

impl SignedDistanceFunction for SDFUnion {
    fn distance(&self, point: &Point) -> f64 {
        let mut dist = f64::MAX;
        
        for sdf in self.sdfs.iter() {
            dist = dist.min(sdf.distance(point));
        }

        dist
    }
}

impl SDFUnion {
    pub fn new(sdfs: Vec<Box<dyn SignedDistanceFunction>>) -> Self {
        SDFUnion { sdfs }
    }
}
