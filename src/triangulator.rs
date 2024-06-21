use delaunator::Point;

const EMPTY: usize = usize::MAX;

#[derive(Debug, PartialEq)]
enum FaceType {
  Normal, 
  Hole, 
  Boundary,
  Destroyed,
}

#[derive(Debug)]
struct Halfedge {
  end: usize,
  next: usize,
  prev: usize,
  twin: usize,
  face: usize,
  face_type: FaceType,
}

impl Halfedge {

  pub fn empty() -> Self {
    Self {end: EMPTY, next: EMPTY, prev: EMPTY, twin: EMPTY, face: EMPTY, face_type: FaceType::Normal}
  }

  pub fn is_valid(&self) -> bool {
    self.next != EMPTY && self.prev != EMPTY && self.face != EMPTY
  }
}

#[derive(Debug)]
struct Face {
  halfedge: usize,
  border: bool,
  face_type: FaceType,
}

impl Face {

  pub fn new(edge: usize) -> Self {
    Self {halfedge: edge, border: false, face_type: FaceType::Normal}
  }

  pub fn is_border(&self) -> bool {
    self.border
  }
}

#[derive(Debug)]
struct Vertex {
  halfedge: usize,
  point: Point,
}

impl Vertex {
  pub fn new(halfedge: usize, point: Point) -> Self {
    Self {halfedge, point}
  }
}

#[derive(Debug)]
struct Mesh {
  faces: Vec<Face>,
  holes: Vec<Face>,
  halfedges: Vec<Halfedge>,
  vertices: Vec<Vertex>,
  boundary: Face,
}

impl Mesh {
  pub fn next(&self, halfedge: &Halfedge) -> &Halfedge {
    assert!(halfedge.next != EMPTY);
    &self.halfedges[halfedge.next]
  }

  pub fn prev(&self, halfedge: &Halfedge) -> &Halfedge {
    assert!(halfedge.prev != EMPTY);
    &self.halfedges[halfedge.prev]
  }

  pub fn twin(&self, halfedge: &Halfedge) -> &Halfedge {
    assert!(halfedge.twin != EMPTY);
    &self.halfedges[halfedge.twin]
  }

  pub fn face(&self, halfedge: &Halfedge) -> &Face {
    assert!(halfedge.face != EMPTY);
    &self.faces[halfedge.face]
  }

  pub fn edge_of_vertex(&self, vertex: &Vertex) -> &Halfedge {
    assert!(vertex.halfedge != EMPTY);
    &self.halfedges[vertex.halfedge]
  }

  pub fn edge_of_face(&self, face: &Face) -> &Halfedge {
    assert!(face.halfedge != EMPTY);
    &self.halfedges[face.halfedge]
  }

  pub fn some_face(&self) -> Option<&Face> {
    self.faces.iter().find(|f| f.face_type == FaceType::Normal)
  }
}

/*pub fn set_twin(halfedge: Halfedge, twin: Halfedge) {
  halfedge.twin = twin;

}*/