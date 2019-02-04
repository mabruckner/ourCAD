pub use boolean::*;
//use display;
use solid::*;

pub fn extrude(face: &Face, direction: &Vector) -> Solid {
  unimplemented!();
}

/// Turns a face into a series of closed edge loops.
pub fn distill(face: &Face) -> (Vec<Point>, Vec<Vec<usize>>) {
  let mut points = Vec::new();
  let mut edges: Vec<(usize, usize)> = Vec::new();
  for edge in face.edges() {
    let (mut ida, mut idb) = (None, None);
    for (i, &point) in points.iter().enumerate() {
      if point == edge.a {
        ida = Some(i);
      }
      if point == edge.b {
        idb = Some(i);
      }
    }
    edges.push((
      ida.unwrap_or_else(|| {
        points.push(edge.a);
        points.len() - 1
      }),
      idb.unwrap_or_else(|| {
        points.push(edge.b);
        points.len() - 1
      }),
    ));
  }
  let mut loops = Vec::new();
  while edges.len() > 0 {
    let mut chain = {
      let e = edges.pop().unwrap();
      vec![e.0, e.1]
    };
    while chain[0] != chain[chain.len() - 1] {
      let t = chain[chain.len() - 1];
      for i in 0..edges.len() {
        match edges[i] {
          (x, idx) | (idx, x) if x == t => {
            edges.swap_remove(i);
            chain.push(idx);
            break;
          }
          _ => (),
        }
      }
    }
    chain.pop();
    loops.push(chain);
  }
  (points, loops)
}

pub fn tri_to_face(tri: &[Point; 3]) -> Face {
  Face {
    plane: Plane {
      norm: (tri[1] - tri[0]).cross(&(tri[2] - tri[0])).into(),
      point: tri[0],
    },
    loops: vec![vec![tri[0], tri[1], tri[2]]],
  }
}

pub fn triangulate_solid(solid: Solid) -> Vec<[Point; 3]> {
  let mut out = Vec::new();
  for face in solid.faces {
    out.extend(triangulate_face(face).into_iter());
  }
  out
}

pub fn triangulate_face(face: Face) -> Vec<[Point; 3]> {
  //display::quick_display(vec![face.clone()]);
  dbg!(&face);
  let (points, loops) = distill(&face);
  let mut base_edges = Vec::new();
  for chain in loops {
    for i in 0..chain.len() {
      base_edges.push((chain[i], chain[(i + 1) % chain.len()]));
    }
  }
  let mut edges = base_edges.clone();
  for i in 0..points.len() {
    'jloop: for j in (i + 1)..points.len() {
      for &edge in &base_edges {
        if edge == (i, j) || edge == (j, i) {
          continue 'jloop;
        }
      }
      let (a, b) = (points[i], points[j]);
      for &(id1, id2) in &edges {
        let (d0, d1, d2, d3) = (
          points[id1] - a,
          b - points[id1],
          points[id2] - b,
          a - points[id2],
        );
        let base = d0.cross(&d1);
        if d1.cross(&d2) * base > 0.0 && d2.cross(&d3) * base > 0.0 && d3.cross(&d0) * base > 0.0 {
          continue 'jloop;
        }
      }
      edges.push((i, j));
    }
  }
  // this is not good! fix Face::contains soon!
  #[cfg(feature = "bad_math")]
  let pertubation = [small * 0.01, small * 0.02, small * 0.03].into();
  {
    let mut i = base_edges.len();
    while i < edges.len() {
      let midpoint = points[edges[i].0].pos * 0.5 + points[edges[i].1].pos * 0.5;
      #[cfg(feature = "bad_math")]
      let midpoint = midpoint + pertubation;
      if !face.contains(&Point { pos: midpoint }).despair() {
        edges.swap_remove(i);
        continue;
      }
      i += 1;
    }
  }
  let mut tris = Vec::new();
  for i in 0..points.len() {
    let mut adjacent = Vec::new();
    let mut j = 0;
    while j < edges.len() {
      match edges[j] {
        (x, ad) | (ad, x) if x == i => {
          edges.swap_remove(j);
          adjacent.push(ad);
        }
        _ => {
          j += 1;
        }
      }
    }
    adjacent.sort_unstable();
    for &(a, b) in &edges {
      match (adjacent.binary_search(&a), adjacent.binary_search(&b)) {
        (Ok(_), Ok(_)) => {
          tris.push([points[i], points[a], points[b]]);
        }
        _ => (),
      }
    }
  }
  /*display::quick_display(vec![Face {
      edges: edges.into_iter().map(|(a,b)| Edge { a: points[a], b: points[b] }).collect(),
      plane: face.plane
  }]);
  unimplemented!();*/
  tris
}
