use crate::mesh::Mesh;
use crate::geometry::Point;

pub fn to_tikz(mesh: &Mesh) -> String {
  let mut tikz: String = String::new();

  tikz.push_str(&preamble());
  
  // 1. faces
  tikz.push_str(&faces_to_tikz(mesh));

  // 2. edges
  tikz.push_str(&edges_to_tikz(mesh));

  // 3. vertices
  tikz.push_str(&vertices_to_tikz(mesh));

  tikz.push_str(&outro());

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

fn faces_to_tikz(mesh: &Mesh) -> String {
  let mut tikz: String = String::new();
  
  for face in mesh.iter_faces() {
    tikz.push_str(&face_to_tikz_string(mesh, face));
    tikz.push_str("\n");
    
    
  }
  tikz
}

fn face_to_tikz_string(mesh: &Mesh, face: usize) -> String {
  let mut tikz: String = String::from("\\filldraw[color=faceColor,fill=faceColorFill]");
  
  for halfedge in mesh.iter_face(face) {
    let p = mesh.point_of_edge(halfedge);
    tikz.push_str(&point_to_tikz(p));
    tikz.push_str("--");
  }
  tikz.pop();
  tikz.pop();
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

fn edges_to_tikz(mesh: &Mesh) -> String {
  let mut tikz: String = String::new();
  for halfedge in mesh.iter_edges() {
    tikz.push_str(&halfedge_to_tikz(mesh, halfedge));
    tikz.push_str("\n");
  }
  tikz
}

fn halfedge_to_tikz(mesh: &Mesh, halfedge: usize) -> String {
  let mut tikz: String = String::new();

  let u = mesh.point_of_edge(halfedge);
  let v = mesh.point_of_edge(mesh.twin(halfedge));

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

fn vertices_to_tikz(mesh: &Mesh) -> String {
  let mut tikz: String = String::new();

  for vertex in mesh.iter_vertices() {
    let point = mesh.point_of_vertex(vertex);
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