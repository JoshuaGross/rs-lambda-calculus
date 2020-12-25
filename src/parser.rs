extern crate nom;

use crate::term::{Term, TermOrDef, Program};

use nom::{
  branch::alt,
  bytes::complete::{tag},
  character::complete::{char, one_of, space0, multispace0},
  combinator::{recognize},
  sequence::{preceded, terminated},
  multi::{fold_many0, many0, separated_list1},
  IResult,
};

fn parse_ident(s: &str) -> IResult<&str, String> {
    let initial_chars: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let remaining_chars: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

    // Returns whole strings matched by the given parser.
    let (s, ident) = recognize(
      // Runs the first parser, if succeeded then runs second, and returns the second result.
      // Note that returned ok value of `preceded()` is ignored by `preceded()`; `recognize` returns
      // the whole string for us.
      preceded(
        // Parses a single character contained in the given string.
        one_of(initial_chars),
        // Parses the longest slice consisting of the given characters
        many0(one_of(remaining_chars)),
      )
    )(s)?;

    Ok((s, ident.to_string()))
}

pub fn parse_var(s: &str) -> IResult<&str, Term> {
    let (s, ident) = parse_ident(s)?;
    Ok((s, Term::Var(ident)))
}

pub fn parse_paren_term(s: &str) -> IResult<&str, Term> {
    let (s, _) = preceded(space0, terminated(tag("("), space0))(s)?;
    let (s, t) = parse_raw_term(s)?;
    let (s, _) = preceded(space0, terminated(tag(")"), space0))(s)?;
    Ok((s, t))
}

pub fn parse_abstraction(s: &str) -> IResult<&str, Term> {
    let (s, _) = preceded(space0, terminated(alt((tag("\\"), tag("λ"))), space0))(s)?;
    let (s, n) = parse_ident(s)?;
    let (s, _) = preceded(space0, terminated(tag("."), space0))(s)?;
    let (s, body) = parse_raw_term(s)?;
    Ok((s, Term::Abstraction(n, Box::new(body))))
}

pub fn parse_raw_term(s: &str) -> IResult<&str, Term> {
    let (s, term) = alt((parse_var, parse_paren_term, parse_abstraction))(s)?;

    // fold expressions
    fold_many0(
      preceded(space0, terminated(alt((parse_var, parse_paren_term, parse_abstraction)), space0)),
      term,
      |acc, term2| {
        Term::Application(Box::new(acc), Box::new(term2))
      }
    )(s)
}

pub fn parse_term(s: &str) -> IResult<&str, TermOrDef> {
  let (s, term) = parse_raw_term(s)?;
  Ok((s, TermOrDef::Term(term)))
}

pub fn parse_defn(s: &str) -> IResult<&str, TermOrDef> {
    let (s, n) = parse_ident(s)?;
    let (s, _) = preceded(space0, terminated(tag(":="), space0))(s)?;
    let (s, term) = parse_raw_term(s)?;
    Ok((s, TermOrDef::Definition(n, term)))
}

pub fn parse_expressions(s: &str) -> IResult<&str, Program> {
    let (s, vec) = terminated(preceded(multispace0, separated_list1(preceded(space0, terminated(char('\n'), space0)), alt((parse_defn, parse_term)))), multispace0)(s)?;
    Ok((s, Program(vec)))
}

pub fn parse_terms(s: &str) -> IResult<&str, Program> {
  return nom::combinator::all_consuming(parse_expressions)(s);
}
