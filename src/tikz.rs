use crate::DistMesh;
use crate::Point;

pub fn to_tikz_string(distmesh: &DistMesh) -> String {
  let mut tikz: String = String::new();

  tikz.push_str(&preamble());
  
  // 1. faces
  tikz.push_str(&faces_to_tikz(distmesh));

  // 2. edges
  tikz.push_str(&edges_to_tikz(distmesh));

  // 3. vertices
  tikz.push_str(&vertices_to_tikz(distmesh));

  tikz.push_str(&outro());

  tikz
}

fn faces_to_tikz(distmesh: &DistMesh) -> String {
  let mut tikz: String = String::new();
  
  for index in 0..distmesh.triangulation.triangles.len()  {
    if index % 3 == 2 {
        tikz.push_str(&face_to_tikz_string(distmesh, index));
        tikz.push_str("\n");
        //\filldraw[fill=faceColor1](0.8000,-0.7764)--(0.7408,-0.7396)--(0.7744,-0.7724)-- (0.8000,-0.7764);
    }
  }

  tikz
}

fn preamble() -> String {
  let mut tex = String::new();
  tex.push_str("\\documentclass{standalone}\n");
  tex.push_str("\\usepackage{tikz}\n\n");
  tex.push_str("% Color Definitions\n");
  tex.push_str("\\definecolor{faceColor}{RGB}{255,254,163}\n");
  tex.push_str("\\definecolor{faceColorFill}{RGB}{255,254,163}\n");
  tex.push_str("\\definecolor{edgeColor}{RGB}{0,0,0}\n");
  tex.push_str("\\definecolor{vertexColor}{RGB}{0,0,0}\n");
  tex.push_str("\\definecolor{vertexColorFill}{RGB}{0,0,0}\n");
  tex.push_str("\n\\pgfmathsetmacro{\\circleSize}{2.0pt}\n\\begin{document}");
  tex.push_str("% Change scaling to [x=1mm,y=1mm] if TeX reports 'Dimension too large'.\n\\begin{tikzpicture}[x=1mm,y=1mm]\n");
  tex
}

fn outro() -> String {
  String::from("\\end{tikzpicture}\n\\end{document}")
}

fn face_to_tikz_string(distmesh: &DistMesh, index: usize) -> String {
  let mut tikz: String = String::from("\\filldraw[color=faceColor,fill=faceColorFill]");
  for i in 0..3 {
    let p: &Point = &distmesh.points[distmesh.triangulation.triangles[index-(i%3)]];
    tikz.push_str(&point_to_tikz(p));
    if i != 2 {
      tikz.push_str("--");
    }
    
  }
  tikz.push(';');
  tikz
}

fn point_to_tikz(point: &Point) -> String {
  let mut tikz = String::new();
  tikz.push('(');
  tikz.push_str(&point.x.to_string());
  tikz.push(',');
  tikz.push_str(&point.y.to_string());
  tikz.push(')');
  tikz
}

fn halfedge_to_tikz(distmesh: &DistMesh, halfedge: usize) -> String {
  let mut tikz: String = String::new();
  let iu = distmesh.triangulation.triangles[halfedge];
  let itwin = distmesh.triangulation.halfedges[halfedge];
  let iv = distmesh.triangulation.triangles[itwin];

  let u: &Point = &distmesh.points[iu];
  let v: &Point = &distmesh.points[iv];

  tikz.push_str(&points_to_tikz(u, v));
  tikz
}

fn points_to_tikz(u: &Point, v: &Point) -> String {
  let mut tikz: String = String::new();
  tikz.push_str("\\draw[color=edgeColor]");
  tikz.push_str(&point_to_tikz(u));
  tikz.push_str("--");
  tikz.push_str(&point_to_tikz(v));
  tikz.push(';');
  tikz
}

fn edges_to_tikz(distmesh: &DistMesh) -> String {
 //\draw[color=edgeColor1](-0.5075,0.8298) -- (-0.5266,0.8501);
 // \draw[color=vertexColor1, fill=vertexColor1Fill](-0.5266,0.8501) circle (\circleSize);
  let mut tikz: String = String::new();
  for &halfedge in &distmesh.triangulation.halfedges {
    if !distmesh.is_empty(halfedge) {
      tikz.push_str(&halfedge_to_tikz(distmesh, halfedge));
      tikz.push_str("\n");
    }
  }

  let len = distmesh.triangulation.hull.len();
  for i in 0..len {
    let iv = distmesh.triangulation.hull[i];
    let iu = distmesh.triangulation.hull[(i + 1) % len];

    let u: &Point = &distmesh.points[iu];
    let v: &Point = &distmesh.points[iv];

    tikz.push_str(&points_to_tikz(u, v));
    tikz.push_str("\n");
  }
  tikz
}

fn vertices_to_tikz(distmesh: &DistMesh) -> String {
  let mut tikz: String = String::new();

  for point in &distmesh.points {
    tikz.push_str("\\draw[color=vertexColor, fill=vertexColorFill]");
    tikz.push_str(&point_to_tikz(point));
    tikz.push_str("circle");
    tikz.push('(');
    tikz.push_str("\\circleSize");
    tikz.push(')');
    tikz.push(';');
    tikz.push_str("\n");
  }

  tikz
}