extern crate svgparser;
extern crate svg;
use svg::parse;

fn main() {
  let example = r#"
  <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<line x1="15" y1="0" x2="15" y2="30" stroke-width="2" stroke="black"/></svg>
  "#;
  println!("{:?}", parse(example))
}
