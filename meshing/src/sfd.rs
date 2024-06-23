use crate::geometry::Point;
use crate::geometry::DSPoint;

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

#[derive(Clone)]
pub struct Rect {
    center: Point,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(center: Point, width: f64, height: f64) -> Self {
        Rect { center, width, height }
    }

    fn x_dist(&self, point: &Point) -> f64 {
        (self.center.x-point.x).abs() - self.width/2.0
    }

    fn y_dist(&self, point: &Point) -> f64 {
        (self.center.y-point.y).abs() - self.height/2.0
    }
}

impl SignedDistanceFunction for Rect {
    /*fn distance(&self, point: &Point) -> f64 {
        f64::max(self.x_dist(point), self.y_dist(point))
    }*/

    fn distance(&self, point: &Point) -> f64 {
        let dx = (point.x - self.center.x).abs() - self.width/2.0;
        let dy = (point.y - self.center.y).abs() - self.height/2.0;
    
        let outside_distance = (dx.max(0.0).powi(2) + dy.max(0.0).powi(2)).sqrt();
        let inside_distance = dx.max(dy).min(0.0);
    
        if dx > 0.0 || dy > 0.0 {
            outside_distance
        } else {
            inside_distance
        }
    }

    /*fn grad_with_eps(&self, point: &Point, _: f64) -> Point {
        let x_dist = self.x_dist(point);
        let y_dist = self.y_dist(point);
        if x_dist > y_dist {
            if self.center.x < point.x {
                Point {x: 1.0, y: 0.0}
            } else {
                Point {x: -1.0, y: 0.0}
            }
        } else {
            if self.center.y < point.y {
                Point {x: 0.0, y: 1.0}
            } else {
                Point {x: 0.0, y: -1.0}
            }
        }
    }*/
}

#[derive(Clone)]
pub struct Circle {
    center: Point,
    radius: f64,
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

pub struct Ring {
    center: Point,
    inner_radius: f64,
    outer_radius: f64,
}

impl Ring {
    pub fn new(center: Point, inner_radius: f64, outer_radius: f64) -> Self {
        assert!(inner_radius < outer_radius);
        Self {center, inner_radius, outer_radius}
    }
}

impl SignedDistanceFunction for Ring {
    fn distance(&self, point: &Point) -> f64 {
        let r1 = (self.outer_radius + self.inner_radius) / 2.0;
        let r2 = (self.outer_radius - self.inner_radius) / 2.0;

        let dx = point.x - self.center.x;
        let dy = point.y - self.center.y;
        let len = (dx*dx + dy*dy).sqrt();
        (len-r1).abs() - r2
    }
}

pub struct SDFUnion {
    sdfs: Vec<Box<dyn SignedDistanceFunction>>,
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
