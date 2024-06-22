use std::slice::Iter;

use delaunator::Point;

pub const EMPTY: usize = usize::MAX;

#[derive(Debug)]
pub struct Mesh {
  faces: Vec<Face>,
  holes: Vec<usize>,
  halfedges: Vec<Halfedge>,
  vertices: Vec<Vertex>,
  pub boundary: usize,
}

/*impl Iterator for Mesh {
  type Item = usize;
  
  fn next(&mut self) -> Option<Self::Item> {
    match self.some_face() {
      None => return None,
      Some(face) => {
        let halfedge = self.faces[face].halfedge;

        return Some(1)
      }
    };    
  }
}*/

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
    mesh.set_halfedge(v1, halfedge1);
    mesh.set_halfedge(v2, halfedge2);
    mesh.set_halfedge(v3, halfedge3);
    mesh.faces[inner_face].halfedge = halfedge1;

    mesh.halfedges[halfedge1].next = halfedge2;
    mesh.halfedges[halfedge2].next = halfedge3;
    mesh.halfedges[halfedge3].next = halfedge1;

    mesh.halfedges[halfedge1].prev = halfedge3;
    mesh.halfedges[halfedge2].prev = halfedge1;
    mesh.halfedges[halfedge3].prev = halfedge2;

    let twin1 = mesh.create_halfedge(v3, Some(mesh.boundary));
    let twin2 = mesh.create_halfedge(v1, Some(mesh.boundary));
    let twin3 = mesh.create_halfedge(v2, Some(mesh.boundary));
    mesh.faces[mesh.boundary].halfedge = twin1;

    mesh.halfedges[twin1].next = twin3;
    mesh.halfedges[twin2].next = twin1;
    mesh.halfedges[twin3].next = twin2;

    mesh.halfedges[twin1].prev = twin2;
    mesh.halfedges[twin2].prev = twin3;
    mesh.halfedges[twin3].prev = twin1;

    mesh.halfedges[halfedge1].twin = twin1;
    mesh.halfedges[halfedge2].twin = twin2;
    mesh.halfedges[halfedge3].twin = twin3;

    mesh.halfedges[twin1].twin = halfedge1;
    mesh.halfedges[twin2].twin = halfedge2;
    mesh.halfedges[twin3].twin = halfedge3;

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

  pub fn set_halfedge(&mut self, vertex: usize, halfedge: usize) {
    debug_assert!(self.vertices.len() > vertex);
    debug_assert!(self.vertices[vertex].halfedge == EMPTY);
    self.vertices[vertex].halfedge = halfedge;
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

  pub fn edge_of_vertex(&self, vertex: usize) -> usize {
    debug_assert!(vertex != EMPTY);
    debug_assert!(self.vertices.len() > vertex);
    self.vertices[vertex].halfedge
  }

  pub fn vertex_of_edge(&self, halfedge: usize) -> usize {
    debug_assert!(halfedge != EMPTY);
    debug_assert!(self.halfedges.len() > halfedge);
    self.halfedges[halfedge].end
  }

  pub fn edge_of_face(&self, face: usize) -> usize {
    debug_assert!(face != EMPTY);
    self.faces[face].halfedge
  }

  pub fn point_of_vertex(&self, vertex: usize) -> &Point {
    debug_assert!(vertex != EMPTY);
    debug_assert!(self.vertices.len() > vertex);
    &self.vertices[vertex].point
  }

  pub fn point_of_edge(&self, halfedge: usize) -> &Point {
    self.point_of_vertex(self.vertex_of_edge(halfedge))
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
  face_type: FaceType,
}

impl Halfedge {

  pub fn empty(id: usize) -> Self {
    Self {id, end: EMPTY, next: EMPTY, prev: EMPTY, twin: EMPTY, face: EMPTY, face_type: FaceType::Normal}
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