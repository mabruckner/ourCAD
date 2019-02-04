use solid::*;
//use display;

pub enum Boolean {
  Union,
  Intersection,
  Difference,
}

pub fn boolean(a: &Solid, b: &Solid, op: Boolean) -> Solid {
  let (a_i, a_o) = cut(a, b);
  let (b_i, b_o) = cut(b, a);
  dbg!(&b_i);
  let (mut faces, more_faces) = match op {
    Boolean::Union => (a_o, b_o),
    Boolean::Intersection => (a_i, b_i),
    Boolean::Difference => (a_o, b_i),
  };
  faces.extend(more_faces.into_iter());
  Solid { faces: faces }
}

pub fn cut(target: &Solid, tool: &Solid) -> (Vec<Face>, Vec<Face>) {
  let (mut i_faces, mut o_faces) = (Vec::new(), Vec::new());
  for face in &target.faces {
    let stamp = slice(tool, &face.plane);
    let o = face_boolean(&face, &stamp, Boolean::Difference);
    let i = face_boolean(&face, &stamp, Boolean::Intersection);
    //display::quick_display(vec![face.clone(), stamp.clone()]);
    if o.loops.len() > 0 {
      o_faces.push(o);
    }
    if i.loops.len() > 0 {
      i_faces.push(i);
    }
  }
  (i_faces, o_faces)
}

fn shatter(target: &Face, tool: &Vec<Edge>) -> Vec<Edge> {
  // if there's a zero-length edge it might cause problems
  let mut fragments: Vec<Edge> = target.edges().collect();
  let mut i = 0;
  while i < fragments.len() {
    for t_edge in tool {
      let f_edge = fragments[i];
      let f_t = f_edge.b - f_edge.a;
      let t_t = t_edge.b - t_edge.a;
      if f_t.cross(&t_t) == [0.0, 0.0, 0.0].into() {
        // lines are parallel
        //unimplemented!(); // I think this can be handled gracefully.
        //dbg!((f_edge, t_edge));
      } else {
        // not parallel
        let t = (t_edge.a - f_edge.a).cross(&t_t) * target.plane.norm;
        let t = t / (f_t.cross(&t_t) * target.plane.norm);
        if t < 1.0 - small && t > small {
          //dbg!(t);
          let pt = f_edge.a.pos + f_t * t;
          let u = ((pt - t_edge.a.pos) * t_t) / (t_t * t_t);
          if u < 1.0 + small && u > -small {
            dbg!((t, u));
            //display::quick_display(vec![Face{plane: target.plane.clone(), edges: vec![f_edge.clone(), t_edge.clone()]}]);
            let newpoint = Point {
              pos: f_edge.a.pos + f_t * t,
            };
            fragments[i] = Edge {
              a: f_edge.a,
              b: newpoint,
            };
            fragments.push(Edge {
              a: newpoint,
              b: f_edge.b,
            });
          }
        }
      }
    }
    i += 1;
  }
  fragments
}

pub fn face_boolean(a: &Face, b: &Face, op: Boolean) -> Face {
  if a.plane != b.plane {
    panic!();
  }
  let a_fragments = shatter(&a, &b.edges().collect());
  let b_fragments = shatter(&b, &a.edges().collect());
  let mut out: Vec<Edge> = Vec::new();
  // these hope and despair values are incorrect
  out.extend(a_fragments.into_iter().filter(|e| {
    match (
      b.contains(&Point {
        pos: e.a.pos * 0.5 + e.b.pos * 0.5,
      }),
      &op,
    ) {
      (x, Boolean::Intersection) => x.hope(),
      (x, Boolean::Union) | (x, Boolean::Difference) => !x.hope(),
    }
  }));
  out.extend(b_fragments.into_iter().filter(|e| {
    match (
      a.contains(&Point {
        pos: e.a.pos * 0.5 + e.b.pos * 0.5,
      }),
      &op,
    ) {
      (x, Boolean::Union) => !x.despair(),
      (x, Boolean::Intersection) | (x, Boolean::Difference) => x.despair(),
    }
  }));
  if let Ok(mut face) = Face::from_edges(out) {
    face.plane = a.plane;
    face
  } else {
    Face {
      plane: a.plane,
      loops: Vec::new(),
    }
  }
}

/// This has problems! It **will** cause problems! Frustrating problems! Fix it someday!
pub fn slice(s: &Solid, p: &Plane) -> Face {
  let x = p.norm.0 * p.point.pos;
  let mut edges = Vec::new();
  for face in &s.faces {
    let mut points = Vec::new();
    for edge in face.edges() {
      let a = edge.a.pos * p.norm;
      let b = edge.b.pos * p.norm;
      let t = (x - a) / (b - a);
      if 0.0 - small < t && t < 1.0 + small {
        points.push(Point {
          pos: edge.a.pos * (1.0 - t) + edge.b.pos * t,
        });
      }
    }
    let d = p.norm.0.cross(&face.plane.norm.0);
    if d == [0.0; 3].into() {
      continue;
    }
    let d: Unit = d.into();
    points.sort_unstable_by(|a, b| (a.pos * d.0).partial_cmp(&(b.pos * d.0)).unwrap());
    for i in 0..(points.len() / 2) {
      edges.push(Edge {
        a: points[i * 2],
        b: points[i * 2 + 1],
      });
    }
  }
  if let Ok(mut face) = Face::from_edges(edges) {
    face.plane = *p;
    face
  } else {
    Face {
      plane: *p,
      loops: Vec::new(),
    }
  }
}
