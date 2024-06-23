use meshing::geometry::Point;
use meshing::geometry::{find_visible_edge,legalize,is_illegal};
use meshing::mesh::Mesh;

#[test]
fn simple_face_iteration() {
  let p = Point {x: 5.0, y: -1.0};
  assert!(p.in_circle(&Point {x: 0.0, y: 0.0}, &Point {x:10.0, y: 0.0}, &Point {x: 5.0, y: 1.0}));
  
  let mut mesh = Mesh::triangle(Point {x: 0.0, y: 0.0}, Point {x:10.0, y: 0.0}, Point {x: 5.0, y: 1.0});
  let halfedge = find_visible_edge(&mesh, &p);
  assert!(mesh.iter_edges().count() == 6);
  
  mesh.insert(halfedge, p);
  assert!(is_illegal(&mesh, halfedge));
  assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|u| u.x == 5.0 && u.y == -1.0));
  assert!(mesh.iter_edges().count() == 10);
  assert!(mesh.iter_edges().any(|halfedge| is_illegal(&mesh, halfedge)));
  
  legalize(&mut mesh, halfedge);
  assert!(mesh.iter_edges().all(|halfedge| !is_illegal(&mesh, halfedge)));
}
