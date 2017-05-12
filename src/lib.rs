
extern crate svgparser;

pub use svgparser::svg::Token as SvgToken;


pub enum SvgEvent {
    Line {
        x1: f64,
        x2: f64,
        y1: f64,
        y2: f64,
    }
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
            Text(s) => Token::Text(std::str::from_utf8(s.slice()).unwrap().to_owned()),
            Whitespace(_) => Token::Whitespace,
            _ => unimplemented!()
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

    }
}