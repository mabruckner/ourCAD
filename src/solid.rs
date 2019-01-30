use std::ops::{Mul, Add, Sub, Neg};

pub const small: f64 = 0.0001;

#[derive(Debug, Copy, Clone)]
pub struct Vector {
  pub c: [f64; 3],
}

impl Vector {
  pub fn new(c: [f64; 3]) -> Self {
    Vector { c: c }
  }
  pub fn cross(&self, other: &Vector) -> Self {
    [self.c[1] * other.c[2] - self.c[2] * other.c[1],
     self.c[2] * other.c[0] - self.c[0] * other.c[2],
     self.c[0] * other.c[1] - self.c[1] * other.c[0]]
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
    Point { pos: Vector::new(p) }
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
      point: Point { pos: self.point.pos + offset },
      norm: self.norm,
    }
  }
}

impl Eq for Plane {}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Face {
  pub edges: Vec<Edge>,
  pub plane: Plane,
}

impl Face {
  pub fn from_edges(edges: Vec<Edge>) -> Result<Face, Vec<Edge>> {
    if edges.len() < 2 {
      Err(edges)
    } else {
      let a = (edges[0].a.pos - edges[0].b.pos);
      let mut b = (edges[1].a.pos - edges[1].b.pos);
      for i in 1..edges.len() {
        b = (edges[i].a.pos - edges[i].b.pos);
        let c = a.cross(&b);
        if c * c > small {
          break;
        }
      }
      //println!("{:?} {:?}", a, b);
      let u: Unit = a.cross(&b).into();
      let x = edges[0].a.pos * u.0;
      let mut big = 0.0;
      for e in &edges {
        big = (e.a.pos * u.0 - x).abs().max(big);
        big = (e.b.pos * u.0 - x).abs().max(big);
      }
      if big >= small {
        Err(edges)
      } else {
        let plane = Plane {
          point: edges[0].a,
          norm: u,
        };
        Ok(Face {
          plane: plane,
          edges: edges,
        })
      }
    }
  }
  /// This has problems! It **will** cause problems! Frustrating problems! Fix it someday!
  pub fn contains(&self, p: &Point) -> bool {
    if !self.plane.intersect_point(p) {
      return false;
    }
    let mut i_count = 0;
    for edge in &self.edges {
      let a = edge.a - self.plane.point;
      let b = edge.b - self.plane.point;
      let p = *p - self.plane.point;
      let ab = a.cross(&b);
      let ap = a.cross(&p);
      let pb = p.cross(&b);
      if (ab * (ap + pb) < ab * ab) && ab * ap > 0.0 && ab * pb > 0.0 {
        i_count += 1;
      }
    }
    i_count % 2 == 1
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
    let p = vec![[s1[0], s1[1], s1[2]],
                 [s1[0], s1[1], s2[2]],
                 [s1[0], s2[1], s1[2]],
                 [s1[0], s2[1], s2[2]],
                 [s2[0], s1[1], s1[2]],
                 [s2[0], s1[1], s2[2]],
                 [s2[0], s2[1], s1[2]],
                 [s2[0], s2[1], s2[2]]]
      .into_iter()
      .map(|x| Point::new(x))
      .collect::<Vec<Point>>();

    let e = vec![(p[0], p[1]),
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
                 (p[3], p[7])]
      .into_iter()
      .map(|(a, b)| Edge { a: a, b: b })
      .collect::<Vec<Edge>>();

    let faces = vec![vec![e[0], e[1], e[4], e[5]],
                     vec![e[2], e[3], e[6], e[7]],
                     vec![e[0], e[2], e[8], e[9]],
                     vec![e[1], e[3], e[10], e[11]],
                     vec![e[4], e[6], e[8], e[10]],
                     vec![e[5], e[7], e[9], e[11]]]
      .into_iter()
      .map(|x| Face::from_edges(x).unwrap())
      .collect::<Vec<Face>>();

    Solid { faces: faces }
  }
}
