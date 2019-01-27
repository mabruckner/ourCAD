#[derive(Debug, Copy, Clone)]
struct Point {
    pos: [f64; 3],
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        for i in 0..3 {
            if (self.pos[i] - other.pos[i]).abs() > 0.0001 {
                return false;
            }
        }
        true
    }
}
impl Eq for Point {}
impl Point {
    fn new(p: [f64; 3]) -> Self {
        Point { pos: p }
    }
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    A: Point,
    B: Point,
}
impl PartialEq for Edge {
    fn eq(&self, other: &Edge) -> bool {
        (self.A == other.A && self.B == other.B) || (self.A == other.B && self.B == other.A)
    }
}

impl Eq for Edge {}

#[derive(Debug, PartialEq, Eq)]
struct Face {
    edges: Vec<Edge>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Solid {
    faces: Vec<Face>,
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
        .map(|(a, b)| Edge { A: a, B: b })
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
        .map(|x| Face { edges: x })
        .collect::<Vec<Face>>();

        Solid { faces: faces }
    }
}
