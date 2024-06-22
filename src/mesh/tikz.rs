use nannou::draw::Mesh;

/*
pub fn to_tikz_string(distmesh: &Mesh) -> String {
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

fn faces_to_tikz(distmesh: &Mesh) -> String {
  let mut tikz: String = String::new();
  
  for i in 0..mesh.faces

  for index in 0..distmesh.triangulation.triangles.len()  {
    if index % 3 == 2 {
        tikz.push_str(&face_to_tikz_string(distmesh, index));
        tikz.push_str("\n");
        //\filldraw[fill=faceColor1](0.8000,-0.7764)--(0.7408,-0.7396)--(0.7744,-0.7724)-- (0.8000,-0.7764);
    }
  }

  tikz
} */