extern crate svgparser;
extern crate svg;

use svgparser::svg as svg_parser;
use svgparser::svg::Token as SvgToken;

fn main() {
  let example = r#"
  <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/></svg>
  "#;
  let mut p = svg_parser::Tokenizer::new(example.as_bytes());
  loop {
    let x = p.parse_next();
    if x.is_err() {
      return
    }
    if let Ok(SvgToken::EndOfStream) = x {
      return
    }
    if let Ok(SvgToken::ElementEnd(_)) = x {
      break
    }
  }
  while let Ok(x) = p.parse_next() {
    if x == SvgToken::EndOfStream {
      return
    }
    if let SvgToken::ElementStart(name) = x {
        println!("{:?}", stack(std::str::from_utf8(name).unwrap().to_owned(), &mut p))
    }
  }
}

fn stack(name: String, parser: &mut svg_parser::Tokenizer) -> Result<svg::ElementStack, ()> {
  let mut stack = svg::ElementStack::new(name);
  while let Ok(x) = parser.parse_next() {
    if let SvgToken::ElementEnd(_) = x {
      return Ok(stack)
    }
    stack.push(x)
  }
  unimplemented!()
}
