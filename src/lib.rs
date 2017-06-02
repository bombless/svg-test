
extern crate svgparser;

pub use svgparser::svg::Token as SvgToken;
use svgparser::svg as svg_parser;


#[derive(Debug)]
pub enum SvgEvent {
    Line {
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
        view_box: [f64; 4],
        stroke: svgparser::RgbColor,
        stroke_width: f64,
    },
    Text(String),
}

#[derive(Debug)]
pub struct ElementStack {
    kind: String,
    stack: Vec<Token>,
}

#[derive(Debug)]
enum Token {
    Attribute(String, String),
    Text(String),
    Whitespace,
}

impl<'a> From<SvgToken<'a>> for Token {
    fn from(t: SvgToken) -> Token {
        use SvgToken::*;
        match t {
            Attribute(k, v) => {
                Token::Attribute(std::str::from_utf8(k).unwrap().to_owned(), std::str::from_utf8(v.slice()).unwrap().to_owned())
            },
            Text(s) => Token::Text(decode::decode(std::str::from_utf8(s.slice()).unwrap()).unwrap()),
            Whitespace(_) => Token::Whitespace,
            _ => unimplemented!()
        }
    }
}

impl SvgEvent {
    fn from_stack(s: ElementStack, view_box: [f64; 4]) -> Result<Self, ()> {
        if s.kind == "line" {
            let mut x1 = None;
            let mut x2 = None;
            let mut y1 = None;
            let mut y2 = None;
            let mut stroke = None;
            let mut stroke_width = None;

            for t in &s.stack {
                if let &Token::Attribute(ref k, ref v) = t {
                    if k == "x1" {
                        if let Ok(v) = v.parse() {
                            x1 = Some(v)
                        }
                    }
                    else if k == "y1" {
                        if let Ok(v) = v.parse() {
                            y1 = Some(v)
                        }
                    }
                    else if k == "x2" {
                        if let Ok(v) = v.parse() {
                            x2 = Some(v)
                        }
                    }
                    else if k == "y2" {
                        if let Ok(v) = v.parse() {
                            y2 = Some(v)
                        }
                    }
                    else if k == "stroke" {
                        stroke = Some(svgparser::RgbColor::from_stream(&mut svgparser::Stream::new(v.as_bytes())).map_err(|_| ())?)
                    }
                    else if k == "stroke-width" {
                        stroke_width = v.parse().ok()
                    }
                }
            }
            match (x1, x2, y1, y2, stroke, stroke_width) {
                (Some(x1), Some(x2), Some(y1), Some(y2), Some(stroke), Some(stroke_width)) => return Ok(SvgEvent::Line {
                    x1, x2, y1, y2, view_box, stroke, stroke_width
                }),
                _ => ()
            }
        } else if s.kind == "text" {
            for t in &s.stack {
                if let &Token::Text(ref s) = t {
                    return Ok(SvgEvent::Text(s.to_owned()))
                }
            }
        }
        Err(())
    }
}

impl ElementStack {
    pub fn pop(self) -> Result<SvgEvent, ()> {
        unimplemented!()
    }

    pub fn new(name: String) -> ElementStack {
        ElementStack { kind: name, stack: Vec::new() }
    }

    pub fn push(&mut self, t: SvgToken) {
        self.stack.push(t.into())
    }
}

pub fn parse(src: &str) -> Result<Vec<SvgEvent>, ()> {
  let mut p = svg_parser::Tokenizer::new(src.as_bytes());
  let mut view_box = None;
  loop {
    let x = if let Ok(x) = p.parse_next() {
        x
    } else {
        return Err(())
    };
    if let SvgToken::EndOfStream = x {
      return Err(())
    }
    if let SvgToken::ElementEnd(_) = x {
      break
    }
    if let SvgToken::Attribute(k, v) = x {
        if k == b"viewBox" {
            fn parse(s: &str) -> Result<[f64; 4], ()> {
                let mut tmp = Vec::new();
                for slice in s.split(|x| x == ' ') {
                    tmp.push(if let Ok(r) = slice.parse() { r } else { return Err(()) })
                }
                if tmp.len() == 4 {
                    return Ok([tmp[0], tmp[1], tmp[2], tmp[3]])
                }
                return Err(())
            }
            if let Ok(s) = std::str::from_utf8(v.slice()) {
                if let Ok(rs) = parse(s) {
                    view_box = Some(rs)
                } else {
                    return Err(())
                }
            } else {
                return Err(())
            }
        }
    }
  }
  let mut ret = Vec::new();
  while let Ok(x) = p.parse_next() {
    if x == SvgToken::EndOfStream {
      return Ok(ret)
    }
    if let SvgToken::ElementStart(name) = x {
      if let Ok(s) = stack(std::str::from_utf8(name).unwrap().to_owned(), &mut p) {
          ret.push(SvgEvent::from_stack(s, view_box.unwrap_or_else(|| [0.0; 4])).map_err(|_| while let Ok(x) = p.parse_next() { println!("{:?}", x); if let SvgToken::ElementEnd(_) = x { return } })?)
      } else {
          return Err(())
      }
    }
  }
  return Err(())
}

fn stack(name: String, parser: &mut svg_parser::Tokenizer) -> Result<ElementStack, ()> {
  let mut stack = ElementStack::new(name);
  while let Ok(x) = parser.parse_next() {
    use svgparser::svg::ElementEnd::*;
    match &x {
        &SvgToken::ElementEnd(Close(_)) |
        &SvgToken::ElementEnd(Empty) => return Ok(stack),
        &SvgToken::ElementEnd(_) => continue,
        _ => (),
    }
    stack.push(x)
  }
  unimplemented!()
}

mod decode;