use crate::geometry::Point;

pub const EMPTY: usize = usize::MAX;

#[derive(Debug)]
pub struct Mesh {
  faces: Vec<Face>,
  holes: Vec<usize>,
  halfedges: Vec<Halfedge>,
  vertices: Vec<Vertex>,
  boundary: usize,
}

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

impl Mesh {

  pub fn triangle(u1: Point, u2: Point, u3: Point) -> Mesh {
    //TODO we assume ccw ordering here!
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

  /*pub fn iter_faces(&self) -> Iter<'_, usize> {
    mesh.ha
  }*/

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
  
  pub fn create_halfedge(&mut self, vertex: usize, face: Option<usize>) -> usize {
    debug_assert!(self.vertices.len() > vertex);
    let id = self.halfedges.len();
    let mut halfedge = Halfedge::empty(id);
    halfedge.end = vertex;
    halfedge.face = face.unwrap_or(EMPTY);
    self.halfedges.push(halfedge);
    id
  }

  pub fn set_edge_of_vertex(&mut self, vertex: usize, halfedge: usize) {
    debug_assert!(self.vertices.len() > vertex);
    self.vertices[vertex].halfedge = halfedge;
  }

  pub fn set_vertex(&mut self, halfedge: usize, vertex: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].end = vertex;
  }

  pub fn set_face(&mut self, halfedge: usize, face: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].face = face;
  }

  pub fn set_cycle(&mut self, a: usize, b: usize, c: usize) {
    self.set_next(a, b);
    self.set_next(b, c);
    self.set_next(c, a);

    self.set_prev(a, c);
    self.set_prev(b, a);
    self.set_prev(c, b);
  }

  pub fn set_cycle_and_twins(&mut self, a: usize, b: usize, c: usize, ta: usize, tb: usize, tc: usize) {
    self.set_cycle(a, b, c);
    self.set_cycle(ta, tc, tb);
    self.set_twins(a, ta);
    self.set_twins(b,tb);
    self.set_twins(c,tc);
  }

  pub fn set_twins(&mut self, a: usize, b: usize) {
    self.set_twin(a,b);
    self.set_twin(b, a);
  }

  pub fn set_next(&mut self, halfedge: usize, next: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].next = next;
  }

  pub fn set_twin(&mut self, halfedge: usize, twin: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].twin = twin;
  }

  pub fn set_prev(&mut self, halfedge: usize, prev: usize) {
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].prev = prev;
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

  pub fn create_face(&mut self, face_type: FaceType) -> usize {
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

  pub fn vertex(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].end
  }

  pub fn edge_of_face(&self, face: usize) -> usize {
    debug_assert!(face != EMPTY);
    self.faces[face].halfedge
  }

  pub fn set_edge_of_face(&mut self, face: usize, halfedge: usize) {
    debug_assert!(face != EMPTY);
    self.faces[face].halfedge = halfedge;
  }

  pub fn set_face_of_edge(&mut self, halfedge: usize, face: usize) {
    debug_assert!(halfedge != EMPTY);
    self.halfedges[halfedge].face = face;
  }

  pub fn point_of_vertex(&self, vertex: usize) -> &Point {
    debug_assert!(vertex != EMPTY);
    debug_assert!(self.vertices.len() > vertex);
    &self.vertices[vertex].point
  }

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
}

/*pub fn set_twin(halfedge: Halfedge, twin: Halfedge) {
  halfedge.twin = twin;

}*/

#[derive(Debug, PartialEq)]
pub enum FaceType {
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

  pub fn empty(id: usize) -> Self {
    Self {id, end: EMPTY, next: EMPTY, prev: EMPTY, twin: EMPTY, face: EMPTY}
  }

  pub fn is_valid(&self) -> bool {
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

  pub fn empty(id: usize) -> Self {
    Self {id, halfedge: EMPTY, face_type: FaceType::Normal}
  }

  pub fn new(id: usize, edge: usize, face_type: FaceType) -> Self {
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

  pub fn new(id: usize, halfedge: usize, point: Point) -> Self {
    Self {id, halfedge, point}
  }
}