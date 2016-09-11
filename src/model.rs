use luminance_gl::gl33::Tessellation;
use std::collections::BTreeMap;
//use std::fs::File;
//use std::io::Read;
use std::path::{Path, PathBuf};
use wavefront_obj::{self, obj};

// FIXME: implement materials
pub type Material = ();

pub type Vertex = (VertexPos, VertexNor, VertexTexCoord);
pub type VertexPos = [f32; 3];
pub type VertexNor = [f32; 3];
pub type VertexTexCoord = [f32; 2];

pub struct Model<'a> {
  parts: Vec<Part<'a>>
}

impl<'a> Model<'a> {
  pub fn from_parts(parts: Vec<Part<'a>>) -> Self {
    Model {
      parts: parts
    }
  }
}

pub struct Part<'a> {
  tess: Tessellation,
  mat: Option<&'a Material>
}

impl<'a> Part<'a> {
  pub fn new(tess: Tessellation, mat: Option<&'a Material>) -> Self {
    Part {
      tess: tess,
      mat: mat
    }
  }

  pub fn tessellation(&self) -> &Tessellation {
    &self.tess
  }

  pub fn material(&self) -> Option<&Material> {
    self.mat.clone()
  }
}

// FIXME: review all the code bellow
////pub fn load<P>(path: P, mode: tessellation::Mode) -> Result<gl33::Tessellation, TessellationError> where P: AsRef<Path> {
////  let path = path.as_ref();
////
////  info!("loading tessellation: \x1b[35m{:?}", path);
////
////  let mut input = String::new();
////
////  // load the data directly into memory; no buffering nor streaming
////  {
////    let mut file = try!(File::open(path).map_err(|e| TessellationError::FileNotFound(path.to_path_buf(), format!("{:?}", e))));
////    let _ = file.read_to_string(&mut input);
////  }
////
////  // parse the obj file and convert it
////  let obj_set = try!(obj::parse(input).map_err(TessellationError::ParseFailed));
////
////  convert_obj(obj_set, mode)
////}
////
////// Turn a loaded wavefront obj object into a `Model`
////fn convert_obj(obj_set: obj::ObjSet, mode: tessellation::Mode) -> Result<gl33::Tessellation, TessellationError> {
////  if obj_set.objects.len() != 1 {
////    return Err(TessellationError::MultiObjects);
////  }
////
////  let obj = &obj_set.objects[0];
////  info!("  found object \x1b[35m{}", obj.name);
////
////  // interleave the vertices
////  let interleaved_vertices = interleave_vertices(&obj.vertices, &obj.normals, &obj.tex_vertices);
////
////  Err(TessellationError::Error)
////}

// Convert wavefront_obj’s Geometry into a pair of vertices and indices.
//
// This function will regenerate the indices on the fly based on which are used in the shapes in the
// geometry. It’s used to create independent tessellation.
fn convert_geometry(geo: &obj::Geometry, positions: &[obj::Vertex], normals: &[obj::Normal], tvertices: &[obj::TVertex]) -> Result<(Vec<Vertex>, Vec<u32>), ModelError> {
  let mut vertices = Vec::new(); // FIXME: better allocation scheme?
  let mut indices = Vec::new();
  let mut index_map = BTreeMap::new();

  for prim in geo.shapes.iter().map(|s| s.primitive) {
    let keys = try!(create_keys_from_primitive(prim));

    for key in keys {
      match index_map.get(&key).map(|&i| i) {
        Some(index) => {
          // that triplet already exists; just append the index in the indices buffer
          indices.push(index);
        },
        None => {
          // this is a new, not yet discovered triplet; create the corresponding vertex and add it
          // to the vertices buffer, and map the triplet to the index in the indices buffer
          let vertex = interleave_vertex(&positions[key.0], &normals[key.1], &tvertices[key.2]);
          let index = vertices.len() as u32;

          vertices.push(vertex);
          indices.push(index);
          index_map.insert(key, index);
        }
      }
    }
  }

  Ok((vertices, indices))
}

// Create triplet keys from wavefront_obj primitives. If any primitive doesn’t have all the triplet
// information (position, normal, tex), a ModelError::UnsupportedVertex error is returned instead.
fn create_keys_from_primitive(prim: obj::Primitive) -> Result<Vec<(usize, usize, usize)>, ModelError> {
  match prim {
    obj::Primitive::Point(i) => {
      let a = try!(vtnindex_to_key(i));
      Ok(vec![a])
    },
    obj::Primitive::Line(i, j) => {
      let a = try!(vtnindex_to_key(i));
      let b = try!(vtnindex_to_key(j));
      Ok(vec![a, b])
    },
    obj::Primitive::Triangle(i, j, k) => {
      let a = try!(vtnindex_to_key(i));
      let b = try!(vtnindex_to_key(j));
      let c = try!(vtnindex_to_key(k));
      Ok(vec![a, b, c])
    }
  }
}

// Convert from a wavefront_obj VTNIndex into our triplet, raising error if not possible.
fn vtnindex_to_key(i: obj::VTNIndex) -> Result<(usize, usize, usize), ModelError> {
  match i {
    (pi, Some(ti), Some(ni)) => Ok((pi, ni, ti)),
    _ => Err(ModelError::UnsupportedVertex)
  }
}

fn interleave_vertex(p: &obj::Vertex, n: &obj::Normal, t: &obj::TVertex) -> Vertex {
  (convert_vertex(p), convert_nor(n), convert_tvertex(t))
}

fn convert_vertex(v: &obj::Vertex) -> VertexPos {
  [v.x as f32, v.y as f32, v.z as f32]
}

fn convert_nor(n: &obj::Normal) -> VertexNor {
  convert_vertex(n)
}

fn convert_tvertex(t: &obj::TVertex) -> VertexTexCoord {
  [t.x as f32, t.y as f32]
}

////mod hot {
////  use super::{TessellationError, Vertex};
////
////  use luminance::tessellation;
////  use luminance_gl::gl33;
////  use resource::ResourceManager;
////  use std::ops::Deref;
////  use std::path::{Path, PathBuf};
////
////  pub struct Tessellation {
////    path: PathBuf,
////    mode: tessellation::Mode,
////    tess: gl33::Tessellation
////  }
////
////  impl Tessellation {
////    fn load<P>(manager: &mut ResourceManager, path: P, mode: tessellation::Mode) -> Result<Self, TessellationError> where P: AsRef<Path> {
////      Err(TessellationError::Error)
////    }
////  }
////  
////  impl Deref for Tessellation {
////    type Target = gl33::Tessellation;
////
////    fn deref(&self) -> &Self::Target {
////      &self.tess
////    }
////  }
////}
////
pub enum ModelError {
  FileNotFound(PathBuf, String),
  ParseFailed(wavefront_obj::ParseError),
  MultiObjects,
  UnsupportedVertex
}
