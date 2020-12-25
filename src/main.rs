mod parser;
mod term;
mod machine;
mod name_gen;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

use std::env;
/**
 * main
 */
fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();

  let filename = std::env::args().nth(1).expect("no filename given");

  let mut file = File::open(filename)?;
  let mut contents = String::new();
  file.read_to_string(&mut contents)?;

  let parser_res = parser::parse_terms(&contents).unwrap().1;
  println!("Parsed:\n{}", parser_res);

  let mut name_gen = name_gen::NameGen{curr_ident: "x".to_string()};
  let reduced = machine::reduce(&parser_res, &mut name_gen);
  println!("Reduced:\n{}", reduced);

  Ok(())
}
