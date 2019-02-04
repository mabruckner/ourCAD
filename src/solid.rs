use display;
use std::ops::{Add, Mul, Neg, Not, Sub};

pub const small: f64 = 0.0001;

pub enum Tern {
  Yes,
  No,
  Maybe,
}

impl Tern {
  pub fn hope(self) -> bool {
    match self {
      Yes | Maybe => true,
      No => false,
    }
  }
  pub fn despair(self) -> bool {
    match self {
      Yes => true,
      Maybe | No => false,
    }
  }
}

impl Not for Tern {
  type Output = Tern;
  fn not(self) -> Tern {
    match self {
      Yes => No,
      Maybe => Maybe,
      No => Yes,
    }
  }
}

use Tern::{Maybe, No, Yes};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Transform {
  pub cols: [Vector; 4],
}

impl Transform {
  pub fn rotate_x(angle: f64) -> Transform {
    Transform {
      cols: [
        [1.0, 0.0, 0.0].into(),
        [0.0, angle.cos(), angle.sin()].into(),
        [0.0, -angle.sin(), angle.cos()].into(),
        [0.0; 3].into(),
      ],
    }
  }
}

impl Mul<Vector> for Transform {
  type Output = Vector;
  fn mul(self, vec: Vector) -> Vector {
    let mut acc = [0.0; 3].into();
    for i in 0..3 {
      acc = acc + self.cols[i] * vec.c[i];
    }
    acc
  }
}

impl Mul<Point> for Transform {
  type Output = Point;
  fn mul(self, pt: Point) -> Point {
    Point {
      pos: self * pt.pos + self.cols[3],
    }
  }
}

impl Mul<Unit> for Transform {
  type Output = Unit;
  fn mul(self, unit: Unit) -> Unit {
    (self * unit.0).into()
  }
}

impl Mul for Transform {
  type Output = Transform;
  fn mul(self, other: Transform) -> Transform {
    unimplemented!();
  }
}

impl Mul<Edge> for Transform {
  type Output = Edge;
  fn mul(self, other: Edge) -> Edge {
    Edge {
      a: self * other.a,
      b: self * other.b,
    }
  }
}

impl Mul<Plane> for Transform {
  type Output = Plane;
  fn mul(self, other: Plane) -> Plane {
    Plane {
      point: self * other.point,
      norm: self * other.norm,
    }
  }
}

impl Mul<Face> for Transform {
  type Output = Face;
  fn mul(self, other: Face) -> Face {
    let Face {
      plane: plane,
      loops: loops,
    } = other;
    Face {
      plane: self * plane,
      loops: loops
        .into_iter()
        .map(|l| l.into_iter().map(|x| self * x).collect())
        .collect(),
    }
  }
}

impl Mul<Solid> for Transform {
  type Output = Solid;
  fn mul(self, other: Solid) -> Solid {
    Solid {
      faces: other.faces.into_iter().map(|x| self * x).collect(),
    }
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector {
  pub c: [f64; 3],
}

impl Vector {
  pub fn new(c: [f64; 3]) -> Self {
    Vector { c: c }
  }
  pub fn cross(&self, other: &Vector) -> Self {
    [
      self.c[1] * other.c[2] - self.c[2] * other.c[1],
      self.c[2] * other.c[0] - self.c[0] * other.c[2],
      self.c[0] * other.c[1] - self.c[1] * other.c[0],
    ]
    .into()
  }
  pub fn len(&self) -> f64 {
    let mut acc = 0.0;
    for i in 0..3 {
      acc += self.c[i] * self.c[i];
    }
    acc.sqrt()
  }
}

impl Mul for Vector {
  type Output = f64;
  fn mul(self, other: Vector) -> f64 {
    let mut acc = 0.0;
    for i in 0..3 {
      acc += self.c[i] * other.c[i];
    }
    acc
  }
}
impl Mul<Unit> for Vector {
  type Output = f64;
  fn mul(self, other: Unit) -> f64 {
    self * other.0
  }
}

impl Mul<f64> for Vector {
  type Output = Vector;
  fn mul(mut self, rhs: f64) -> Self {
    for i in 0..3 {
      self.c[i] *= rhs;
    }
    self
  }
}

impl Add for Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    let mut out = [0.0; 3];
    for i in 0..3 {
      out[i] = self.c[i] + other.c[i];
    }
    out.into()
  }
}

impl Sub for Vector {
  type Output = Vector;
  fn sub(self, other: Vector) -> Vector {
    let mut out = [0.0; 3];
    for i in 0..3 {
      out[i] = self.c[i] - other.c[i];
    }
    out.into()
  }
}

impl Neg for Vector {
  type Output = Vector;
  fn neg(mut self) -> Vector {
    for i in 0..3 {
      self.c[i] *= -1.0;
    }
    self
  }
}

impl From<[f64; 3]> for Vector {
  fn from(val: [f64; 3]) -> Self {
    Vector::new(val)
  }
}

impl PartialEq for Vector {
  fn eq(&self, other: &Vector) -> bool {
    for i in 0..3 {
      if (self.c[i] - other.c[i]).abs() > small {
        return false;
      }
    }
    true
  }
}

impl Eq for Vector {}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Unit(pub Vector);

impl Neg for Unit {
  type Output = Unit;
  fn neg(self) -> Self {
    Unit(-self.0)
  }
}

impl From<Vector> for Unit {
  fn from(mut val: Vector) -> Self {
    let mut acc = 0.0;
    for i in 0..3 {
      acc = acc + val.c[i] * val.c[i];
    }
    acc = acc.sqrt();
    if acc < small {
      println!("ERROR {:?}, {:?}", val, acc);
      panic!();
    }
    for i in 0..3 {
      val.c[i] = val.c[i] / acc;
    }
    Unit(val)
  }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Point {
  pub pos: Vector,
}

impl Point {
  pub fn new(p: [f64; 3]) -> Self {
    Point {
      pos: Vector::new(p),
    }
  }
}

impl<'a> Sub for &'a Point {
  type Output = Vector;
  fn sub(self, rhs: &Point) -> Vector {
    *self - *rhs
  }
}

impl Sub for Point {
  type Output = Vector;
  fn sub(self, rhs: Point) -> Vector {
    self.pos - rhs.pos
  }
}

#[derive(Debug, Copy, Clone)]
pub struct Edge {
  pub a: Point,
  pub b: Point,
}

impl PartialEq for Edge {
  fn eq(&self, other: &Edge) -> bool {
    (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
  }
}

impl Eq for Edge {}

#[derive(Debug, Copy, Clone)]
pub struct Plane {
  pub point: Point,
  pub norm: Unit,
}

impl PartialEq for Plane {
  fn eq(&self, other: &Plane) -> bool {
    self.norm == other.norm && self.intersect_point(&other.point)
  }
}

impl Plane {
  pub fn intersect_point(&self, point: &Point) -> bool {
    (self.point.pos * self.norm.0 - point.pos * self.norm.0).abs() < small
  }
  pub fn intersect_edge(&self, edge: &Edge) -> bool {
    self.intersect_point(&edge.a) && self.intersect_point(&edge.b)
  }
  pub fn translate(&self, offset: Vector) -> Plane {
    Plane {
      point: Point {
        pos: self.point.pos + offset,
      },
      norm: self.norm,
    }
  }
}

impl Eq for Plane {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
  pub loops: Vec<Vec<Point>>,
  pub plane: Plane,
}

/// Turns a face into a series of closed edge loops.
fn distill(face: &Vec<Edge>) -> (Vec<Point>, Vec<Vec<usize>>) {
  let mut points = Vec::new();
  let mut edges: Vec<(usize, usize)> = Vec::new();
  for edge in face {
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
      if edges.len() == 0 {
        dbg!(face);
        dbg!(chain);
        dbg!(loops);
        dbg!(edges);
        dbg!(points);
        display::edge_display(face.clone());
        panic!();
      }
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

/// Compute the turn count of a closed polyline. Panics if given less than 2 points.
fn turns(points: &Vec<Point>, normal: &Unit) -> f64 {
  // prime the pump
  let (mut p0, mut p1) = (
    points.get(points.len() - 2).unwrap(),
    points.last().unwrap(),
  );
  // the angle accumulator
  let mut acc = 0.0;
  for p2 in points {
    // compute deltas
    let (da, db) = (p1 - p0, p2 - p1);
    // the x and y here are actually the x and y values of db in a coodinate system rotated so
    // that da is the x-axis and everything has been scaled by |da|*|db|.
    let y = normal.0 * da.cross(&db);
    let x = da * db;
    // add the angle to the accumulator
    acc += y.atan2(x);
    // push the values back
    p0 = p1;
    p1 = p2;
  }
  // I usually hold that radians are superior. This is a special case, as I don't expect this
  // number to touch any additional trigonometry.
  acc / (2.0 * std::f64::consts::PI)
}

/// Determine if a closed polyline specified by points contains point, and computes the turns about
/// point with parity defined by normal.
///
/// Returns a tuple containing a Tern, with Maybe representing coincidence with an edge or point,
/// and the turns about the point - information that is potentially informative beyond the
/// Yes/No/Maybe values.
///
/// an even number of turns (positive or negative) implies that the point lies outside the
/// bounded reigon, understanding that the region is of the xor type - self intersecting loops
/// inside the body of the polygon form a sort of "courtyard" that we define as outside,
/// despite being entirely surrounded by the polygon.
///
/// if acc differs significanly from an integer it can indicate a worrying loss of precision.
/// Because of the way the math is structured, this can happen naturally when point approaches
/// the polygon defined by points, In which case we would expect a Maybe value, indicating
/// coincidence. If a non-Maybe value and a non-integer turn count are returned together then
/// something has gone horribly wrong.
fn contains(points: &Vec<Point>, point: &Point, normal: &Unit) -> (Tern, f64) {
  // see turns() for a similar function that has been annotated
  let mut p0 = points.last().unwrap();
  let mut acc = 0.0;
  let mut on_edge = false;
  for p1 in points {
    let (da, db) = (p0 - point, p1 - point);
    let y = normal.0 * da.cross(&db);
    let x = da * db;
    if y.abs() < small && x < small {
      on_edge = true;
    }
    acc += y.atan2(x);
  }
  acc /= std::f64::consts::PI * 2.0;
  if on_edge {
    (Maybe, acc)
  } else {
    let n = acc.round() as i32;
    if n % 2 == 0 {
      (No, acc)
    } else {
      (Yes, acc)
    }
  }
}

impl Face {
  /// Converts a list of edges into a planar face
  pub fn from_edges(edges: Vec<Edge>) -> Result<Face, Vec<Edge>> {
    // one edge is not enough information to identify a plane and doesn't form a closed loop anyway
    if edges.len() < 2 {
      Err(edges)
    } else {
      // we start by identifying the plane
      // TODO: make this section a bit more general - things break if the first edge in the list is
      // small, for example.
      let a = (edges[0].a.pos - edges[0].b.pos);
      let mut b = (edges[1].a.pos - edges[1].b.pos);
      // this loop cycles through the edges in order to find one that results in an acceptably
      // large cross product. Otherwise the plane normal will not be accurate.
      for i in 1..edges.len() {
        b = (edges[i].a.pos - edges[i].b.pos);
        let c = a.cross(&b);
        if c * c > small {
          break;
        }
      }
      // Compute the normal - this unit vector should be perpendicular to all edges.
      let u: Unit = a.cross(&b).into();
      // Compute the shortest distance from origin to the plane.
      let x = edges[0].a.pos * u.0;
      // Find the maximum deviation of a point from the computed plane.
      let mut big = 0.0;
      for e in &edges {
        big = (e.a.pos * u.0 - x).abs().max(big);
        big = (e.b.pos * u.0 - x).abs().max(big);
      }
      if big >= small {
        // The max deviation is greater than an acceptable value -> edges are not coplanar, an
        // error.
        Err(edges)
      } else {
        // make the plane official
        let plane = Plane {
          point: edges[0].a,
          norm: u,
        };
        // turn the edges into edge loops
        // TODO: Verify that the edges are actually loops.
        // TODO: Verify that the edges do not cross.
        // TODO: Modify the distill function so that loops may not pass over each other by sharing
        // points.
        let (points, mut loops) = distill(&edges);

        // make the loop direction consistent; the inside of a shape is always on the left side of
        // the path.
        let mut loops: Vec<Vec<Point>> = loops
          .into_iter()
          .map(|l| l.into_iter().map(|i| points[i]).collect::<Vec<Point>>())
          .collect();
        let mut shell_count: Vec<usize> = loops.iter().map(|_| 0).collect();
        for i in 0..loops.len() {
          for j in 0..loops.len() {
            if i == j {
              continue;
            }
            match contains(&loops[j], &loops[i][0], &u) {
              (Yes, _) => {
                shell_count[i] += 1;
              }
              (No, _) => (),
              // TODO handle this better; test other points or something
              _ => {
                panic!();
              }
            }
          }
        }
        // actually perform the reversals
        for i in 0..loops.len() {
          let ccw = turns(&loops[i], &plane.norm) > 0.0;
          if ccw != (shell_count[i] % 2 == 0) {
            loops[i].reverse();
          }
        }
        Ok(Face {
          plane: plane,
          loops: loops,
        })
      }
    }
  }
  pub fn edges<'a>(&'a self) -> impl Iterator<Item = Edge> + 'a {
    (0..self.loops.len()).flat_map(move |i| {
      (0..self.loops[i].len()).map(move |j| Edge {
        a: self.loops[i][j],
        b: self.loops[i][(j + 1) % self.loops[i].len()],
      })
    })
  }
  /// Determine if a Point is contained by this face. Returns No if the point is not coplanar, and
  /// returns Maybe if the point is on an edge.
  pub fn contains(&self, p: &Point) -> Tern {
    dbg!(&p);
    dbg!(self);
    if !self.plane.intersect_point(p) {
      return No;
    }
    let mut i_count = 0;
    for l in &self.loops {
      let mut acc = 0.0;
      for i in 0..l.len() {
        let a = l[i];
        let b = l[(i + 1) % l.len()];
        let (da, db) = (a - *p, b - *p);
        let y = self.plane.norm.0 * da.cross(&db);
        let x = da * db;
        if y.abs() < small && x < small {
          return Maybe;
        }
        acc += y.atan2(x);
      }
      acc /= std::f64::consts::PI * 2.0;
      let n = acc.round() as i32;
      dbg!(n);
      dbg!(acc);
      if (acc - n as f64).abs() > small {
        // This is a fault - I feel like the panic here is a big much, but until we get a
        // system for this kind of thing it will have to do.
        panic!("accumulator error in Face::contains");
      }
      i_count += n;
    }
    match i_count % 2 == 1 {
      true => Yes,
      false => No,
    }
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Solid {
  pub faces: Vec<Face>,
}

impl Solid {
  pub fn make_box(size: [f64; 3]) -> Self {
    let s1 = [size[0] / 2.0, size[1] / 2.0, size[2] / 2.0];
    let s2 = [-size[0] / 2.0, -size[1] / 2.0, -size[2] / 2.0];
    let p = vec![
      [s1[0], s1[1], s1[2]],
      [s1[0], s1[1], s2[2]],
      [s1[0], s2[1], s1[2]],
      [s1[0], s2[1], s2[2]],
      [s2[0], s1[1], s1[2]],
      [s2[0], s1[1], s2[2]],
      [s2[0], s2[1], s1[2]],
      [s2[0], s2[1], s2[2]],
    ]
    .into_iter()
    .map(|x| Point::new(x))
    .collect::<Vec<Point>>();

    let e = vec![
      (p[0], p[1]),
      (p[2], p[3]),
      (p[4], p[5]),
      (p[6], p[7]),
      (p[0], p[2]),
      (p[1], p[3]),
      (p[4], p[6]),
      (p[5], p[7]),
      (p[0], p[4]),
      (p[1], p[5]),
      (p[2], p[6]),
      (p[3], p[7]),
    ]
    .into_iter()
    .map(|(a, b)| Edge { a: a, b: b })
    .collect::<Vec<Edge>>();

    let faces = vec![
      vec![e[0], e[1], e[4], e[5]],
      vec![e[2], e[3], e[6], e[7]],
      vec![e[0], e[2], e[8], e[9]],
      vec![e[1], e[3], e[10], e[11]],
      vec![e[4], e[6], e[8], e[10]],
      vec![e[5], e[7], e[9], e[11]],
    ]
    .into_iter()
    .map(|x| Face::from_edges(x).unwrap())
    .collect::<Vec<Face>>();

    Solid { faces: faces }
  }
}
