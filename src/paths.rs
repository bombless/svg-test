/// <https://www.w3.org/TR/SVG/paths.html>

/// The whole assumption is wrong,
/// that each command can actually contain seraval groups of arguments
/// for now we only handle one group of argument
/// and consider other options to be ill-formed



use self::Numbers::*;
use self::Data::*;
use self::Relation::*;

impl Data {
  fn from_pattern(c: char, data: Numbers) -> Result<Self, ()> {
    match (c, data) {
      ('m', Two(x, y)) => Ok(Moveto(Relative, x, y)),
      ('M', Two(x, y)) => Ok(Moveto(Absolute, x, y)),
      ('z', Zero) | ('Z', Zero) => Ok(Closepath),
      ('l', Two(x, y)) => Ok(Lineto(Relative, x, y)),
      ('L', Two(x, y)) => Ok(Lineto(Absolute, x, y)),
      ('h', One(x)) => Ok(HorizontalLineto(Relative, x)),
      ('H', One(x)) => Ok(HorizontalLineto(Absolute, x)),
      ('v', One(y)) => Ok(VerticalLineto(Relative, y)),
      ('V', One(y)) => Ok(VerticalLineto(Absolute, y)),
      ('c', Six(x1, y1, x2, y2, x, y)) => Ok(Curveto(Relative, x1, y1, x2, y2, x, y)),
      ('C', Six(x1, y1, x2, y2, x, y)) => Ok(Curveto(Absolute, x1, y1, x2, y2, x, y)),
      ('s', Four(x2, y2, x, y)) => Ok(SmoothCurveto(Relative, x2, y2, x, y)),
      ('S', Four(x2, y2, x, y)) => Ok(SmoothCurveto(Absolute, x2, y2, x, y)),
      ('q', Four(x1, y1, x, y)) => Ok(BezierCurveto(Relative, x1, y1, x, y)),
      ('Q', Four(x1, y1, x, y)) => Ok(BezierCurveto(Absolute, x1, y1, x, y)),
      ('t', Two(x, y)) => Ok(SmoothQuadraticBezierCurveto(Relative, x, y)),
      ('T', Two(x, y)) => Ok(SmoothQuadraticBezierCurveto(Absolute, x, y)),
      ('a', Seven(rx, ry, x_asix_rotation, 1., sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, sweep_flag, x, y, large_arc_flag: true, relation: Relative,
      }),
      ('a', Seven(rx, ry, x_asix_rotation, 0., sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, sweep_flag, x, y, large_arc_flag: false, relation: Relative,
      }),
      ('A', Seven(rx, ry, x_asix_rotation, 1., sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, sweep_flag, x, y, large_arc_flag: true, relation: Absolute,
      }),
      ('A', Seven(rx, ry, x_asix_rotation, 0., sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, sweep_flag, x, y, large_arc_flag: false, relation: Absolute,
      }),
      _ => Err(())
    }
  }
}

#[derive(Debug)]
pub enum Relation {
  Relative,
  Absolute,
}

enum Numbers {
  Zero, One(f64), Two(f64, f64), Four(f64, f64, f64, f64), Six(f64, f64, f64, f64, f64, f64), Seven(f64, f64, f64, f64, f64, f64, f64)
}

#[derive(Debug)]
pub enum Data {
  Moveto(Relation, f64, f64),
  Closepath,
  Lineto(Relation, f64, f64),
  HorizontalLineto(Relation, f64),
  VerticalLineto(Relation, f64),
  Curveto(Relation, f64, f64, f64, f64, f64, f64),
  SmoothCurveto(Relation, f64, f64, f64, f64),
  QuadraticBezierCurveto(Relation, f64, f64, f64, f64),
  BezierCurveto(Relation, f64, f64, f64, f64),
  SmoothQuadraticBezierCurveto(Relation, f64, f64),
  Arc {
    relation: Relation,
    rx: f64, ry: f64, x_asix_rotation: f64, large_arc_flag: bool, sweep_flag: f64, x: f64, y: f64,
  }
}
