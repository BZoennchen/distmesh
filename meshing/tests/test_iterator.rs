use meshing::geometry::Point;
use meshing::geometry::equiliteral_triangle;
use meshing::mesh::Mesh;

#[test]
fn simple_face_iteration() {
  let (p1, p2, p3) = equiliteral_triangle(1.0);

  let mesh = Mesh::triangle(Point {x: p1.x, y: p1.y}, Point {x: p2.x, y: p2.y}, Point {x: p3.x, y: p3.y});
  
  assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p1.x && p.y == p1.y));
  assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p2.x && p.y == p2.y));
  assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p3.x && p.y == p3.y));
  assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).count() == 3);
}

#[test]
fn simple_edge_iteration() {
  let (p1, p2, p3) = equiliteral_triangle(1.0);
  let mesh = Mesh::triangle(p1, p2, p3);
  assert!(mesh.iter_edges().count() == 6);
}

#[test]
fn simple_faces_iteration() {
  let (p1, p2, p3) = equiliteral_triangle(1.0);
  let mesh = Mesh::triangle(p1, p2, p3);
  assert!(mesh.iter_faces().count() == 1);
}

#[test]
fn simple_vertex_iteration() {
  let (p1, p2, p3) = equiliteral_triangle(1.0);
  let mesh = Mesh::triangle(p1, p2, p3);
  assert!(mesh.iter_vertices().count() == 3);
}