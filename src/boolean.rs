use solid::*;

pub enum Boolean {
  Union,
  Intersection,
  Difference,
}

pub fn boolean(a: &Solid, b: &Solid, op: Boolean) -> Solid {
  unimplemented!();
}

pub fn cut(target: &Solid, tool: &Solid) -> (Vec<Face>, Vec<Face>) {
  for face in &target.faces {
    let stamp = slice(tool, &face.plane);
    let mut new_pool = Vec::new();
    let mut sharp_pool: Vec<Edge> = Vec::new();

    for edge in &stamp.edges {
      let ia = face.contains(&edge.a);
      let ib = face.contains(&edge.b);
      if ia && ib {
        new_pool.push(*edge);
      } else if ia || ib {


      }
    }

  }
  unimplemented!();
}

fn shatter(target: &Vec<Edge>, tool: &Vec<Edge>) -> Vec<Edge> {
  // if there's a zero-length edge it might cause problems
  let fragments = target.clone();
  let mut i = 0;
  while i < fragments.len() {
    for t_edge in tool {
      let f_edge = fragments[i];
      let f_t = f_edge.b - f_edge.a;
      let t_t = t_edge.b - t_edge.a;
      if f_t.cross(&t_t) == [0.0, 0.0, 0.0].into() {
        // lines are parallel
        unimplemented!(); // I think this can be handled gracefully.
      } else {
        // not parallel
        unimplemented!();
      }

    }
    i += 1;
  }
  unimplemented!();
}

pub fn face_boolean(a: &Face, b: &Face, op: Boolean) -> Face {
  if a.plane != b.plane {
    panic!();
  }
  let mut a_fragments = shatter(&a.edges, &b.edges);
  let mut b_fragments = shatter(&b.edges, &a.edges);
  let mut i = 0;
  unimplemented!();
}

/// This has problems! It **will** cause problems! Frustrating problems! Fix it someday!
pub fn slice(s: &Solid, p: &Plane) -> Face {
  let x = p.norm.0 * p.point.pos;
  let mut edges = Vec::new();
  for face in &s.faces {
    let mut points = Vec::new();
    for edge in &face.edges {
      let a = edge.a.pos * p.norm;
      let b = edge.b.pos * p.norm;
      let t = (x - a) / (b - a);
      if 0.0 - small < t && t < 1.0 + small {
        points.push(Point { pos: edge.a.pos * (1.0 - t) + edge.b.pos * t });
      }
    }
    let d = p.norm.0.cross(&face.plane.norm.0);
    let d: Unit = d.into();
    points.sort_unstable_by(|a, b| (a.pos * d.0).partial_cmp(&(b.pos * d.0)).unwrap());
    for i in 0..(points.len() / 2) {
      edges.push(Edge {
        a: points[i * 2],
        b: points[i * 2 + 1],
      });
    }
  }
  Face {
    plane: *p,
    edges: edges,
  }
}
