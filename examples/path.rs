extern crate svgparser;
extern crate svg;
use svg::parse;

fn main() {
  let example = r#"
    <svg width="300" height="300" viewBox="0 0 600 600" xmlns="http://www.w3.org/2000/svg">
		<path transform="rotate(90, 300, 300)" d="M100 100 A 250 250 0 1 0 500 100 L 300 300 L 100 100" fill="green"/>
    </svg>
  "#;
  println!("{:?}", parse(example))
}
