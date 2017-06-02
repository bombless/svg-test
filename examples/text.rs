extern crate svgparser;
extern crate svg;
use svg::parse;

fn main() {
  let example = r#"
  <svg width="30" height="30" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
		<text x="15" y="15" font-family="Verdana" text-anchor="middle" alignment-baseline="middle" font-size="15">&#x1f47b;</text>
	</svg>
  "#;
  println!("{:?}", parse(example))
}
