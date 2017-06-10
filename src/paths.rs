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
    // N.B.
    // doc says "Flags and booleans are interpolated as fractions between zero and one,
    // with any non-zero value considered to be a value of one/true"
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
      ('a', Seven(rx, ry, x_asix_rotation, large_arc_flag, sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, x, y,
        large_arc_flag: if large_arc_flag == 1. { true } else if large_arc_flag == 0. { false } else { return Err(()) },
        sweep_flag: if sweep_flag == 1. { true } else if sweep_flag == 0. { false } else { return Err(()) },
        relation: Relative,
      }),
      ('A', Seven(rx, ry, x_asix_rotation, large_arc_flag, sweep_flag, x, y)) => Ok(Arc {
        rx, ry, x_asix_rotation, x, y,
        large_arc_flag: large_arc_flag != 0.,
        sweep_flag: sweep_flag != 0.,
        relation: Absolute,
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

impl Numbers {
  fn parse(from: &[f64]) -> Option<Self> {    
    let len = from.len();
    Some(match len {
      0 => Zero,
      1 => One(from[0]),
      2 => Two(from[0], from[1]),
      4 => Four(from[0], from[1], from[2], from[3]),
      6 => Six(from[0], from[1], from[2], from[3], from[4], from[5]),
      7 => Seven(from[0], from[1], from[2], from[3], from[4], from[5], from[6]),
      _ => return None
    })
  }
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
    rx: f64, ry: f64, x_asix_rotation: f64, large_arc_flag: bool, sweep_flag: bool, x: f64, y: f64,
  }
}

#[derive(Debug, PartialEq)]
enum Element {
  Float(f64),
  Char(char),
}

fn parse_to_elements(s: &str) -> Result<Vec<Element>, ()> {
  use self::Element::*;
  let mut float = String::new();
  let mut ret = Vec::new();
  for c in s.chars() {
    if c.is_numeric() || c == '.' {
      float.push(c)
    } else if c.is_alphabetic() {
      if !float.is_empty() {
        ret.push(Float(float.parse().map_err(|_| ())?));
        float.clear();
      }
      ret.push(Char(c))
    } else if c.is_whitespace() {
      if !float.is_empty() {
        ret.push(Float(float.parse().map_err(|_| ())?));
        float.clear();
      }
    } else {
      return Err(())
    }
  }
  if !float.is_empty() {
    ret.push(Float(float.parse().map_err(|_| ())?))
  }
  Ok(ret)
}

#[cfg(test)]
mod tests {
  #[test]fn test() {
    use super::Element::*;
    assert_eq!(&super::parse_to_elements("a b c").unwrap()[..], [Char('a'), Char('b'), Char('c')]);
    assert_ne!(&super::parse_to_elements("a b c").unwrap()[..], [Char('a'), Char('b')])
  }
}

pub fn parse(s: &str) -> Result<Vec<Data>, ()> {
  use self::Element::*;
  use ::std::mem::replace;
  let mut s = parse_to_elements(s)?.into_iter();
  let mut floats = Vec::new();
  let mut ret = Vec::new();
  let mut prev_c = if let Some(Char(x)) = s.next() {
    x
  } else {
    return Err(())
  };
  for e in s {
    match e {
      Float(f) => floats.push(f),
      Char(c) => ret.push(Data::from_pattern(prev_c, if let Some(x) = Numbers::parse(&replace(&mut floats, Vec::new())) {
        prev_c = c;
        x
      } else {
        return Err(())
      })?)
    }
  }
  ret.push(Data::from_pattern(prev_c, if let Some(x) = Numbers::parse(&replace(&mut floats, Vec::new())) {
    x
  } else {
    return Err(())
  })?);
  Ok(ret)
}
