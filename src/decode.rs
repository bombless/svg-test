pub fn decode(src: &str) -> Result<String, ()> {
  let mut ret = String::new();
  let mut escaping = None;
  enum Hex {
    Yes(String), No(String), Unknown, Started
  }
  
  for c in src.chars() {
    escaping = if let Some(val) = escaping {
      if c == ';' {
        match val {
          Hex::Yes(val) => {
            ret.push(::std::char::from_u32(u32::from_str_radix(&val, 16).map_err(|_| ())?).unwrap());
            None
          },
          Hex::No(val) => {
            ret.push(::std::char::from_u32(val.parse().map_err(|_| ())?).unwrap());
            None
          },
          Hex::Unknown | Hex::Started => {
            return Err(())
          }
        }        
      } else {
        match val {
          Hex::Started => if c == '#' {
            Some(Hex::Unknown)
          } else {
            unimplemented!()
          },
          Hex::Unknown => if c == 'x' {
            Some(Hex::Yes(String::new()))
          } else {
            let mut s = String::new();
            s.push(c);
            Some(Hex::No(s))
          },
          Hex::Yes(mut s) => {
            s.push(c);
            Some(Hex::Yes(s))
          },
          Hex::No(mut s) => {
            s.push(c);
            Some(Hex::No(s))
          }
        }
      }
    } else if c == '&' {
      Some(Hex::Started)
    } else {
      ret.push(c);
      None
    }
  }
  Ok(ret)
}

#[cfg(test)]
mod test {
  #[test]
  fn test() {
    assert_eq!(super::decode("&#xa;").unwrap(), "\n")
  }
}