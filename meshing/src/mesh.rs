use crate::geometry::{DSPoint, Point};

pub const EMPTY: usize = usize::MAX;

pub struct FaceIterator<'a> {
  mesh: &'a Mesh,
  face: usize,
  start: usize,
  current: usize,
}

impl<'a> FaceIterator<'a> {
  pub fn empty(mesh: &'a Mesh) -> Self {
    FaceIterator {mesh: mesh, face: EMPTY, start: EMPTY, current: EMPTY}
  }
}

impl<'a> Iterator for FaceIterator<'a> {
  type Item = usize;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.face == EMPTY {
      return None;
    }

    if self.start == EMPTY {
      self.start = self.mesh.faces[self.face].halfedge;
      self.current = self.start;
    } else {
      self.current = self.mesh.halfedges[self.current].next;
      if (self.current) == (self.start) {
        return None;
      }
    }
    return Some(self.current);
  }
}

pub struct HalfedgeIterator<'a> {
  mesh: &'a Mesh,
  unvisited_faces: Vec<usize>,
  visited_faces: Vec<bool>,
  face_iterator: FaceIterator<'a>,
}

impl<'a> HalfedgeIterator<'a> {
  pub fn new(mesh: &'a Mesh) -> Self {
    let unvisited_faces: Vec<usize> = Vec::new();
    let mut visited_faces = vec![false; mesh.number_of_faces()];
    let some_face = mesh.some_face();

    let face_iterator = match some_face {
      Some(face) => {
        visited_faces[face] = true;
        mesh.iter_face(face)
      },
      None => FaceIterator::empty(mesh)
    };


    HalfedgeIterator {
      mesh,
      unvisited_faces: unvisited_faces,
      visited_faces: visited_faces,
      face_iterator,
    }
  }
}

impl<'a> Iterator for HalfedgeIterator<'a> {
  type Item = usize;
  
  fn next(&mut self) -> Option<Self::Item> {
    match self.face_iterator.next() {
      Some(halfedge) => {
        let new_face = self.mesh.face(self.mesh.twin(halfedge));
        if !self.visited_faces[new_face] {
          self.unvisited_faces.push(new_face)
        }
        Some(halfedge)
      },
      None => {
        if self.unvisited_faces.is_empty() {
          return None;
        }

        loop {
          let face = self.unvisited_faces.pop();
          if face.is_none() {
            return None;
          }

          let face = face.unwrap();
          if self.visited_faces[face] {
            continue;
          }

          self.face_iterator = self.mesh.iter_face(face);
          self.visited_faces[face] = true;
          return self.next()
        }
      }
    }
  }
}

pub struct FacesIterator<'a> {
  mesh: &'a Mesh,
  index: usize,
}

impl<'a> Iterator for FacesIterator<'a> {
  type Item = usize;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.mesh.faces.len() {
      return None;
    }

    while self.mesh.faces[self.index].face_type != FaceType::Normal {
      self.index += 1;
    }

    if self.index >= self.mesh.faces.len() {
      return None;
    } else {
      self.index += 1;
      return Some(self.mesh.faces[self.index-1].id);
    }
  }
}

pub struct VertexIterator<'a> {
  mesh: &'a Mesh,
  index: usize,
}

impl<'a> Iterator for VertexIterator<'a> {
  type Item = usize;
  
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.mesh.vertices.len() {
      return None;
    }

    self.index += 1;
    return Some(self.mesh.vertices[self.index-1].id);
  }
}

#[derive(Debug)]
pub struct Mesh {
  faces: Vec<Face>,
  holes: Vec<usize>,
  halfedges: Vec<Halfedge>,
  vertices: Vec<Vertex>,
  boundary: usize,
}

impl Mesh {

  pub fn triangle(u1: Point, u2: Point, u3: Point) -> Mesh {
    //TODO we assume ccw ordering here!
    debug_assert!(u1.ccw(&u2, &u3));
    let mut mesh = Self::empty();
    let inner_face = mesh.create_face(FaceType::Normal);
    let v1 = mesh.create_vertex(u1);
    let v2 = mesh.create_vertex(u2);
    let v3 = mesh.create_vertex(u3);

    let halfedge1 = mesh.create_halfedge(v1, Some(inner_face));
    let halfedge2 = mesh.create_halfedge(v2, Some(inner_face));
    let halfedge3 = mesh.create_halfedge(v3, Some(inner_face));
    mesh.set_edge_of_vertex(v1, halfedge1);
    mesh.set_edge_of_vertex(v2, halfedge2);
    mesh.set_edge_of_vertex(v3, halfedge3);
    mesh.set_edge_of_face(inner_face, halfedge1);

    let twin1 = mesh.create_halfedge(v3, Some(mesh.boundary));
    let twin2 = mesh.create_halfedge(v1, Some(mesh.boundary));
    let twin3 = mesh.create_halfedge(v2, Some(mesh.boundary));
    mesh.set_edge_of_face(mesh.boundary, twin1);
    
    mesh.set_cycle_and_twins(halfedge1, halfedge2, halfedge3, twin1, twin2, twin3);
    mesh
  }

  pub fn iter_vertices(&self) -> VertexIterator {
    VertexIterator {mesh: self, index: 0}
  }

  pub fn iter_faces(&self) -> FacesIterator {
    FacesIterator {mesh: self, index: 0}
  }

  pub fn iter_face<'a>(&'a self, face: usize) -> FaceIterator {
    FaceIterator {mesh: self, face: face, start: EMPTY, current: EMPTY}
  }

  pub fn iter_edges<'a>(&'a self) -> HalfedgeIterator {
    HalfedgeIterator::new(self)
  }

  pub fn insert(&mut self, halfedge: usize, p: Point) -> usize {
    debug_assert!(self.halfedges.len() > halfedge);
    debug_assert!(
      self.faces[self.face(halfedge)].face_type == FaceType::Boundary || 
      self.faces[self.face(halfedge)].face_type == FaceType::Hole
    );
    
    let a = self.prev(halfedge);
    let b = self.next(halfedge);
    let border = self.face(halfedge);

    if self.edge_of_face(border) == halfedge {
      self.set_edge_of_face(border, b);
    }

    let face = self.create_face(FaceType::Normal);
    let v = self.create_vertex(p);

    self.set_edge_of_face(face, halfedge);
    self.set_face_of_edge(halfedge, face);

    let e1 = self.create_halfedge(v, Some(face));
    let e2 = self.create_halfedge(self.vertex(a), Some(face));
    let t1 = self.create_halfedge(self.vertex(halfedge), Some(border));
    let t2 = self.create_halfedge(v, Some(border));
    self.set_edge_of_vertex(v, e1);

    self.set_next(halfedge, e1);
    self.set_next(e1, e2);
    self.set_next(e2, halfedge);
    self.set_prev(halfedge, e2);
    self.set_prev(e2, e1);
    self.set_prev(e1, halfedge);
    
    self.set_twin(e1, t1);
    self.set_twin(t1, e1);
    self.set_twin(e2, t2);
    self.set_twin(t2, e2);

    self.set_next(a, t2);
    self.set_next(t2, t1);
    self.set_next(t1, b);

    self.set_prev(b, t1);
    self.set_prev(t1, t2);
    self.set_prev(t2, a);

    t2
  }

  pub fn legalize(&mut self, edge: usize) {
    if self.is_illegal(edge) {
      let twin = self.twin(edge);
      let p = self.vertex(self.next(edge));
  
      self.flip(edge);
  
      let vertex = self.vertex(edge);
  
      if vertex == p {
        let e1 = self.prev(edge);
        let e2 = self.next(self.twin(twin));
        self.legalize(e1);
        self.legalize(e2);
      }
      else {
        let e1 = self.next(edge);
        let e2 = self.prev(self.twin(edge));
        self.legalize(e1);
        self.legalize(e2);
      }
    }
  }
  
  pub fn is_illegal(&self, a: usize) -> bool {
    let b: usize = self.twin(a);
  
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
    let ar = self.prev(a);
  
    if self.is_border(b) {
        return false;
    }
  
    let al = self.next(a);
    let bl = self.prev(b);
    let br = self.next(b);
  
    let p0 = self.point_of_edge(al);
    let pr = self.point_of_edge(ar);
    let pl = self.point_of_edge(bl);
    let p1 = self.point_of_edge(br);
    // TODO: is the order right?
    let in_circ = p0.in_circle(pr, p1, pl);
    in_circ
  }

  pub fn boundary(&self) -> usize {
    self.boundary
  }

  pub fn next(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].next
  }

  pub fn prev(&self, halfedge: usize) -> usize{
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].prev
  }

  pub fn twin(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].twin
  }

  pub fn face(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].face
  }

  pub fn vertex(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].end
  }

  pub fn is_border(&self, halfedge: usize) -> bool {
    debug_assert!(halfedge != EMPTY);
    let face = self.face(halfedge);
    let twin_face = self.face(self.twin(halfedge));
    self.faces[face].face_type == FaceType::Boundary || self.faces[twin_face].face_type == FaceType::Hole
  }

  pub fn edge_of_vertex(&self, vertex: usize) -> usize {
    debug_assert!(vertex != EMPTY);
    debug_assert!(self.vertices.len() > vertex);
    self.vertices[vertex].halfedge
  }

  pub fn edge_of_face(&self, face: usize) -> usize {
    debug_assert!(face != EMPTY);
    self.faces[face].halfedge
  }

  //TODO: critical since we allow the point to be manipulated?
  pub fn point_of_vertex(&self, vertex: usize) -> &Point {
    debug_assert!(vertex != EMPTY);
    debug_assert!(self.vertices.len() > vertex);
    &self.vertices[vertex].point
  }

  //TODO: critical since we allow the point to be manipulated?
  pub fn point_of_edge(&self, halfedge: usize) -> &Point {
    self.point_of_vertex(self.vertex(halfedge))
  }

  pub fn some_face(&self) -> Option<usize> {
    self.faces.iter().find(|f| f.face_type == FaceType::Normal).map(|f| f.id)
  }

  pub fn number_of_faces(&self) -> usize {
    self.faces.len()
  } 

  pub fn validate(&self) -> bool {
    let valid_faces = self.faces.iter().enumerate().all(|(index, face)| {face.id == index});
    let valid_halfedges = self.halfedges.iter().enumerate().all(|(index, halfedge)| {halfedge.id == index});
    let valid_vertices = self.vertices.iter().enumerate().all(|(index, vertex)| {vertex.id == index});
    return valid_faces && valid_halfedges && valid_vertices
  }

  pub fn find_visible_edge(&self, p: &Point) -> Option<usize> {
    for halfedge in self.iter_face(self.boundary()) {
      let u2 = self.point_of_edge(halfedge);
      let u1 = self.point_of_edge(self.twin(halfedge));
      if p.orient(u1, u2) <= 0. {
        return Some(halfedge)
      }
    }
    None
  }

  fn empty() -> Self {
    let mut mesh = Mesh {faces: Vec::new(), holes: Vec::new(), halfedges: Vec::new(), vertices: Vec::new(), boundary: EMPTY};
    mesh.create_face(FaceType::Boundary);
    mesh
  }

  fn create_vertex(&mut self, u: Point) -> usize {
    let id = self.vertices.len();
    let vertex = Vertex::empty(id, u);
    self.vertices.push(vertex);
    id
  }
  
  fn create_halfedge(&mut self, vertex: usize, face: Option<usize>) -> usize {
    debug_assert!(self.vertices.len() > vertex);
    let id = self.halfedges.len();
    let mut halfedge = Halfedge::empty(id);
    halfedge.end = vertex;
    halfedge.face = face.unwrap_or(EMPTY);
    self.halfedges.push(halfedge);
    id
  }

  fn set_edge_of_vertex(&mut self, vertex: usize, halfedge: usize) {
    debug_assert!(self.vertices.len() > vertex);
    self.vertices[vertex].halfedge = halfedge;
  }

  fn set_vertex(&mut self, halfedge: usize, vertex: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].end = vertex;
  }

  fn set_face(&mut self, halfedge: usize, face: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].face = face;
  }

  fn set_cycle(&mut self, a: usize, b: usize, c: usize) {
    self.set_next(a, b);
    self.set_next(b, c);
    self.set_next(c, a);

    self.set_prev(a, c);
    self.set_prev(b, a);
    self.set_prev(c, b);
  }

  fn set_cycle_and_twins(&mut self, a: usize, b: usize, c: usize, ta: usize, tb: usize, tc: usize) {
    self.set_cycle(a, b, c);
    self.set_cycle(ta, tc, tb);
    self.set_twins(a, ta);
    self.set_twins(b,tb);
    self.set_twins(c,tc);
  }

  fn set_twins(&mut self, a: usize, b: usize) {
    self.set_twin(a,b);
    self.set_twin(b, a);
  }

  fn set_next(&mut self, halfedge: usize, next: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].next = next;
  }

  fn set_twin(&mut self, halfedge: usize, twin: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].twin = twin;
  }

  fn set_prev(&mut self, halfedge: usize, prev: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].prev = prev;
  }

  fn create_face(&mut self, face_type: FaceType) -> usize {
    let id = self.faces.len();
    match face_type {
      FaceType::Hole => self.holes.push(id),
      FaceType::Boundary => {
        assert!(self.boundary == EMPTY);
        self.boundary = id
      },
      _ => {}
    };

    let face = Face::new(id, EMPTY, face_type);
    self.faces.push(face);
    id
  }

  fn set_edge_of_face(&mut self, face: usize, halfedge: usize) {
    debug_assert!(face != EMPTY);
    self.faces[face].halfedge = halfedge;
  }

  fn set_face_of_edge(&mut self, halfedge: usize, face: usize) {
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].face = face;
  }

  fn flip(&mut self, halfedge: usize) {

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
    let a1 = self.next(a0);
    let a2 = self.next(a1);
  
    let b0 = self.twin(halfedge);
    let b1 = self.next(b0);
    let b2 = self.next(b1);
  
    let fa = self.face(a0);
    let fb = self.face(b0);
  
    let va1 = self.vertex(a1);
    let vb1 = self.vertex(b1);
  
    let va0 = self.vertex(a0);
    let vb0 = self.vertex(b0);
  
    if self.edge_of_face(fb) == b1 {
      self.set_edge_of_face(fb, a1);
    }
  
    if self.edge_of_face(fa) == a1 {
      self.set_edge_of_face(fa, b1);
    }
  
    // TODO: maybe without if, just do it? its faster?
    assert_eq!(self.vertex(b2), va0);
    assert_eq!(self.vertex(a2), vb0);
  
  
    if self.edge_of_vertex(va0) == a0 {
      self.set_edge_of_vertex(va0, b2);
    }
  
    if self.edge_of_vertex(vb0) == b0 {
      self.set_edge_of_vertex(vb0, a2);
    }
  
    self.set_edge_of_vertex(a0, va1);
    self.set_vertex(b0, vb1);
  
    self.set_next(a0, a2);
    self.set_next(a2, b1);
    self.set_next(b1, a0);
  
    self.set_next(b0, b2);
    self.set_next(b2, a1);
    self.set_next(a1, b0);
  
    self.set_face(a1, fb);
    self.set_face(b1, fa);
  }
}

#[derive(Debug, PartialEq)]
enum FaceType {
  Normal, 
  Hole, 
  Boundary,
  Destroyed,
}

#[derive(Debug)]
struct Halfedge {
  id: usize,
  end: usize,
  next: usize,
  prev: usize,
  twin: usize,
  face: usize,
}

impl Halfedge {

  fn empty(id: usize) -> Self {
    Self {id, end: EMPTY, next: EMPTY, prev: EMPTY, twin: EMPTY, face: EMPTY}
  }

  fn is_valid(&self) -> bool {
    self.next != EMPTY && self.prev != EMPTY && self.face != EMPTY
  }
}

#[derive(Debug)]
struct Face {
  id: usize,
  halfedge: usize,
  face_type: FaceType,
}

impl Face {

  fn empty(id: usize) -> Self {
    Self {id, halfedge: EMPTY, face_type: FaceType::Normal}
  }

  fn new(id: usize, edge: usize, face_type: FaceType) -> Self {
    Self {id, halfedge: edge, face_type: face_type}
  }
}

#[derive(Debug)]
struct Vertex {
  id: usize,
  halfedge: usize,
  point: Point,
}

impl Vertex {

  fn empty(id: usize, point: Point) -> Self {
    Self {id, halfedge: EMPTY, point}
  }

  fn new(id: usize, halfedge: usize, point: Point) -> Self {
    Self {id, halfedge, point}
  }
}

mod testing {
  use super::*;
  use crate::geometry::equiliteral_triangle;

  #[test]
  fn simple_face_iteration() {
    let p = Point {x: 5.0, y: -1.0};
    assert!(p.in_circle(&Point {x: 0.0, y: 0.0}, &Point {x:10.0, y: 0.0}, &Point {x: 5.0, y: 1.0}));
    
    let mut mesh = Mesh::triangle(Point {x: 0.0, y: 0.0}, Point {x:10.0, y: 0.0}, Point {x: 5.0, y: 1.0});
    let halfedge = mesh.find_visible_edge(&p).unwrap();
    assert!(mesh.iter_edges().count() == 6);
    
    mesh.insert(halfedge, p);
    assert!(mesh.is_illegal(halfedge));
    assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|u| u.x == 5.0 && u.y == -1.0));
    assert!(mesh.iter_edges().count() == 10);
    assert!(mesh.iter_edges().any(|halfedge| mesh.is_illegal(halfedge)));
    
    mesh.legalize(halfedge);
    assert!(mesh.iter_edges().all(|halfedge| !mesh.is_illegal(halfedge)));
  }

  #[test]
  fn test_simple_face_iteration() {
    let (p1, p2, p3) = equiliteral_triangle(1.0);

    let mesh = Mesh::triangle(Point {x: p1.x, y: p1.y}, Point {x: p2.x, y: p2.y}, Point {x: p3.x, y: p3.y});
    
    assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p1.x && p.y == p1.y));
    assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p2.x && p.y == p2.y));
    assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).any(|p| p.x == p3.x && p.y == p3.y));
    assert!(mesh.iter_face(mesh.boundary()).map(|halfedge| mesh.point_of_edge(halfedge)).count() == 3);
  }

  #[test]
  fn test_simple_edge_iteration() {
    let (p1, p2, p3) = equiliteral_triangle(1.0);
    let mesh = Mesh::triangle(p1, p2, p3);
    assert!(mesh.iter_edges().count() == 6);
  }

  #[test]
  fn test_simple_faces_iteration() {
    let (p1, p2, p3) = equiliteral_triangle(1.0);
    let mesh = Mesh::triangle(p1, p2, p3);
    assert!(mesh.iter_faces().count() == 1);
  }

  #[test]
  fn test_simple_vertex_iteration() {
    let (p1, p2, p3) = equiliteral_triangle(1.0);
    let mesh = Mesh::triangle(p1, p2, p3);
    assert!(mesh.iter_vertices().count() == 3);
  }
}