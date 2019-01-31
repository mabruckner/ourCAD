use std::io::prelude::*;
use std::io::Result;
use solid::*;
use ops::*;

pub fn write_stl<W: Write>(write: &mut W, solid: Solid, name: &str) -> Result<()> {
  let tris = triangulate_solid(solid);
  writeln!(write, "solid {}", name);
  for tri in tris {
    let normal: Unit = (tri[1] - tri[0]).cross(&(tri[2] - tri[0])).into();
    writeln!(write,
             "facet normal {} {} {}",
             normal.0.c[0],
             normal.0.c[1],
             normal.0.c[2])?;
    writeln!(write, "  outer loop")?;
    for i in 0..3 {
      writeln!(write,
               "    vertex {} {} {}",
               tri[i].pos.c[0],
               tri[i].pos.c[1],
               tri[i].pos.c[2])?;
    }
    writeln!(write, "  endloop")?;
    writeln!(write, "endfacet")?;

  }
  writeln!(write, "endsolid {}", name)?;
  Ok(())
}
