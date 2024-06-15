use delaunator::{Point, triangulate, Triangulation, EMPTY};

use crate::dspoint::DSPoint;

/*
public double getQuality()
	{
		Collection<VTriangle> triangles = getCurrentTriangles();
		double aveSum = 0;
		for(VTriangle triangle : triangles) {
			VLine[] lines = triangle.getLines();
			double a = lines[0].length();
			double b = lines[1].length();
			double c = lines[2].length();
			double part = 0.0;
			if(a != 0.0 && b != 0.0 && c != 0.0) {
				part = ((b + c - a) * (c + a - b) * (a + b - c)) / (a * b * c);
			}
			aveSum += part;
			if(Double.isNaN(part) || Double.isNaN(aveSum)) {
				throw new IllegalArgumentException(triangle + " is not a feasible triangle!");
			}
		}

		return aveSum / triangles.size();
	}


*/

pub fn quality(points: &[Point], triangles: &[usize]) -> f64 {
  let ntriagnles = triangles.len() / 3;
  let mut quality = 0.0;

  for i in 0..ntriagnles {
    let index = i * 3;
    let u1 = &points[triangles[index]];
    let u2 = &points[triangles[index+1]];
    let u3 = &points[triangles[index+2]];

    let a = u1.distance(u2);
    let b = u1.distance(u3);
    let c = u2.distance(u3);

    quality += ((b + c - a) * (c + a - b) * (a + b - c)) / (a * b * c);

  }

  quality / ntriagnles as f64
}