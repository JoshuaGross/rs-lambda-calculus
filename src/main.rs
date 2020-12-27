#![recursion_limit="100000"]

mod parser;
mod term;
mod machine;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

/**
 * main
 */
fn main() -> Result<(), Box<dyn Error>> {
  let filename = std::env::args().nth(1).expect("no filename given");

  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let parser_res = parser::parse_terms(&contents).unwrap().1;
  println!("Parsed:\n{}", parser_res);

  let reduced = machine::reduce(&parser_res);
  println!("Reduced:\n{}", reduced);

  Ok(())
}
